pub mod error;

use std::path::PathBuf;

use anyhow::{self};
use client_stub::file_transfer_client::FileTransferClient;
use error::{AddFileError, AddGroupError};
use log::{debug, error, info};

use crate::client::client_stub::{AddFileRequest, AddGroupRequest};

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
            }
            Err(err) => {
                error!("Request failed with status {err:?}");
                return Err(AddFileError::RequestFailed);
            }
        };

        Ok(())
    }

    pub async fn add_group(&mut self, name: String, prefix: PathBuf) -> Result<(), AddGroupError> {
        let abs_prefix = if prefix.is_absolute() {
            prefix
        } else {
            crate::util::path::absolute_path(prefix).unwrap()
        };

        debug!("Resolved absolute path: {abs_prefix:?}");

        let request = tonic::Request::new(AddGroupRequest {
            name,
            prefix: abs_prefix.to_str().unwrap().to_owned(),
        });

        let result = self.client.add_group(request).await;

        match result {
            Ok(response) => {
                info!("Server response: {response:?}");
            }
            Err(err) => {
                error!("Request failed with status {err:?}");
                return Err(AddGroupError::RequestFailed);
            }
        };

        Ok(())
    }
}

