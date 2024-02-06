use anyhow::{self, Ok};
use client_stub::{file_transfer_client::FileTransferClient, ListFilesRequest};

pub mod client_stub {
    tonic::include_proto!("syncer");
}


pub struct SyncerClientProxy {
    server_uri: String,
    pub client: FileTransferClient<tonic::transport::Channel>,
}


impl SyncerClientProxy {
    pub async fn new(server_uri: String) -> anyhow::Result<Self> {
        anyhow::Ok(Self {
            server_uri: server_uri.clone(),
            client: FileTransferClient::connect(server_uri).await?,
        })
    }
}


// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let mut client = FileTransferClient::connect("http://127.0.0.1:8080").await?;
//     let request = tonic::Request::new(ListFilesRequest {
//         request: "This is request content".into(),
//     });
//
//     let response = client.list_files(request).await?;
//
//     println!("Server response: {response:?}");
//
//     Ok(())
// }
