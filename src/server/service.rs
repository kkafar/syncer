use log::info;
use serde::{self, Deserialize, Serialize};
use server_stub::file_transfer_server::FileTransferServer;
use std::net::SocketAddrV4;
use std::path::PathBuf;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Status};

use self::server_stub::file_transfer_server::FileTransfer;
use self::server_stub::{
    AddFileRequest, AddFileResponse, AddGroupRequest, AddGroupResponse, ListFilesRequest,
    ListFilesResponse, ListGroupsRequest, ListGroupsResponse, RemoveFileRequest,
    RemoveFileResponse, RemoveGroupRequest, RemoveGroupResponse,
};
use crate::context::Context;
use crate::server::db;

pub mod server_stub {
    tonic::include_proto!("syncer");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerDescription {
    pub path: PathBuf,
}

pub struct ServerProxy {
    ctx: Context,
    sck_addr: SocketAddrV4,
}

impl ServerProxy {
    pub fn new(ctx: Context, sck_addr: SocketAddrV4) -> Self {
        Self { ctx, sck_addr }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let addr = self.sck_addr;
        tonic::transport::Server::builder()
            .add_service(FileTransferServer::new(self))
            .serve(std::net::SocketAddr::V4(addr))
            .await?;

        Ok(())
    }
}

#[tonic::async_trait]
impl FileTransfer for ServerProxy {
    type ListGroupsStream = Pin<Box<dyn Stream<Item = Result<ListGroupsResponse, Status>> + Send>>;

    async fn list_files(
        &self,
        request: Request<ListFilesRequest>,
    ) -> Result<tonic::Response<ListFilesResponse>, Status> {
        info!("Received client ListFiles request {request:?}");

        let reply = ListFilesResponse {
            response: "This is an response from server".to_owned(),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn add_file(
        &self,
        request: Request<AddFileRequest>,
    ) -> Result<tonic::Response<AddFileResponse>, Status> {
        info!("Received client AddFile request {request:?}");
        let reply = AddFileResponse { success: true };

        Ok(tonic::Response::new(reply))
    }

    async fn remove_file(
        &self,
        request: Request<RemoveFileRequest>,
    ) -> Result<tonic::Response<RemoveFileResponse>, Status> {
        info!("Received client RemoveFile request {request:?}");
        let reply = RemoveFileResponse { success: true };

        Ok(tonic::Response::new(reply))
    }

    async fn add_group(
        &self,
        request: Request<AddGroupRequest>,
    ) -> Result<tonic::Response<AddGroupResponse>, Status> {
        info!("Received client AddGroup request {request:?}");

        let data = request.into_inner();
        let mut guard = self.ctx.db.lock().unwrap();
        let mut db = guard.take().unwrap();
        let result = db.insert_group(db::model::GroupsRecord {
            name: data.name,
            prefix: data.prefix,
        });
        guard.replace(db);
        std::mem::drop(guard);

        let reply = AddGroupResponse {
            success: result.is_ok(),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn remove_group(
        &self,
        request: Request<RemoveGroupRequest>,
    ) -> Result<tonic::Response<RemoveGroupResponse>, Status> {
        info!("Received client RemoveGroup request {request:?}");

        let data = request.into_inner();
        let mut guard = self.ctx.db.lock().unwrap();
        let mut db = guard.take().unwrap();
        let result = db.delete_group(data.name);
        guard.replace(db);
        std::mem::drop(guard);

        let reply = RemoveGroupResponse {
            success: result.is_ok(),
        };
        Ok(tonic::Response::new(reply))
    }

    async fn list_groups(
        &self,
        request: Request<ListGroupsRequest>,
    ) -> Result<tonic::Response<Self::ListGroupsStream>, Status> {
        info!("Received client ListGroups request {request:?}");
        let mut guard = self.ctx.db.lock().unwrap();
        let mut db = guard.take().unwrap();
        let result = db.list_groups();
        guard.replace(db);
        std::mem::drop(guard);

        let res_vec = result.unwrap();
        let (tx, rx) = mpsc::channel(res_vec.len() + 1);

        let mut reply_stream = Box::pin(tokio_stream::iter(res_vec.into_iter().map(|rd| {
            ListGroupsResponse {
                group_name: rd.name,
                group_prefix: rd.prefix,
            }
        })));

        tokio::spawn(async move {
            while let Some(item) = reply_stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {}
                    Err(_item) => {
                        break;
                    }
                }
            }
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(tonic::Response::new(
            Box::pin(output_stream) as Self::ListGroupsStream
        ))
    }
}
