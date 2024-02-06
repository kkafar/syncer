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
pub struct Server {
    sck_addr: SocketAddrV4,
}

impl Server {
    pub fn new(local_port: u16) -> Self {
        trace!("Creating new server instance. Binding to local port: {local_port}");
        Self {
            sck_addr: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), local_port),
        }
    }

    pub fn run(self) {
        let _listener = TcpListener::bind(self.sck_addr).unwrap();
    }
}

#[tonic::async_trait]
impl FileTransfer for Server {
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    let addr = std::net::SocketAddr::V4(addr);
    let server = Server::new(8080);

    tonic::transport::Server::builder()
        .add_service(FileTransferServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
