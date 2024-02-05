use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use anyhow::{self, Ok};
use client_stub::{file_transfer_client::FileTransferClient, ListFilesRequest};
use tonic::transport::Uri;

pub mod client_stub {
    tonic::include_proto!("syncer");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = FileTransferClient::connect("http://127.0.0.1:8080").await?;
    let request = tonic::Request::new(ListFilesRequest {
        request: "This is request content".into(),
    });

    let response = client.list_files(request).await?;

    println!("Server response: {response:?}");

    Ok(())
}
