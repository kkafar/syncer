use log::trace;
use serde::{self, Deserialize, Serialize};
use server_stub::file_transfer_server::FileTransferServer;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::path::PathBuf;
use tonic::{Request, Status};

use self::server_stub::file_transfer_server::FileTransfer;
use self::server_stub::{ListFilesRequest, ListFilesResponse};

pub mod server_stub {
    tonic::include_proto!("syncer");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerDescription {
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct ServerProxy {
    sck_addr: SocketAddrV4,
}

impl ServerProxy {
    pub fn new(sck_addr: SocketAddrV4) -> Self {
        Self {
            sck_addr,
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let addr = self.sck_addr.clone();
        tonic::transport::Server::builder()
            .add_service(FileTransferServer::new(self))
            .serve(std::net::SocketAddr::V4(addr))
            .await?;

        Ok(())
    }

}

#[tonic::async_trait]
impl FileTransfer for ServerProxy {
    async fn list_files(
        &self,
        _request: Request<ListFilesRequest>,
    ) -> Result<tonic::Response<ListFilesResponse>, Status> {
        let reply = ListFilesResponse {
            response: "This is an response from server".to_owned(),
        };

        Ok(tonic::Response::new(reply))
    }
}

