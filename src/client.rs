pub mod error;

use std::path::PathBuf;

use anyhow::{self};
use client_stub::file_transfer_client::FileTransferClient;
use log::{debug, error, info};
use error::AddFileError;

use crate::client::client_stub::AddFileRequest;

pub mod client_stub {
    tonic::include_proto!("syncer");
}

pub struct SyncerClientProxy {
    _server_uri: String,
    pub client: FileTransferClient<tonic::transport::Channel>,
}

impl SyncerClientProxy {
    pub async fn new(server_uri: String) -> anyhow::Result<Self> {
        anyhow::Ok(Self {
            _server_uri: server_uri.clone(),
            client: FileTransferClient::connect(server_uri).await?,
        })
    }

    pub async fn add_file(&mut self, file: PathBuf) -> Result<(), AddFileError> {
        // Let's first make sure that this file exists (should be done here or by server?)

        let file = crate::util::path::absolute_path(file).unwrap();

        debug!("Resolved absolute path: {file:?}");

        if !file.is_file() {
            return Err(AddFileError::FileNotExists);
        }

        let request = tonic::Request::new(AddFileRequest {
            file_path: file.to_str().unwrap().to_string(),
        });

        let result = self.client.add_file(request).await;
        match result {
            Ok(response) => {
                info!("Server response: {response:?}");
            },
            Err(err) => {
                error!("Request failed with status {err:?}");
                return Err(AddFileError::RequestFailed);
            }
        };

        Ok(())
    }
}

