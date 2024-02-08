pub mod error;

use std::path::PathBuf;

use anyhow::{self};
use client_stub::file_transfer_client::FileTransferClient;
use error::{AddFileError, AddGroupError, RemoveFileError};
use log::{debug, error, info};

use crate::client::client_stub::{AddFileRequest, AddGroupRequest, ListFilesRequest, RemoveFileRequest};

use self::{client_stub::RemoveGroupRequest, error::{ListFilesError, RemoveGroupError}};

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

    pub async fn list_files(&mut self, _group: Option<String>) -> Result<(), ListFilesError> {
        let request = tonic::Request::new(ListFilesRequest {
            request: "This is request content".into(),
        });
        let result = self.client.list_files(request).await;
        match result {
            Ok(response) => {
                info!("Server response: {response:?}");
            }
            Err(err) => {
                error!("Request failed with status {err:?}");
                return Err(ListFilesError::RequestFailed);
            }
        };
        Ok(())
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

    pub async fn remove_file(&mut self, file: PathBuf) -> Result<(), RemoveFileError> {
        let file = crate::util::path::absolute_path(file).unwrap();
        debug!("Resolved absolute path: {file:?}");

        let request = tonic::Request::new(RemoveFileRequest {
            file_path: file.to_str().unwrap().to_string(),
        });

        let result = self.client.remove_file(request).await;

        match result {
            Ok(response) => {
                info!("Server response: {response:?}");
            }
            Err(err) => {
                error!("Request failed with status {err:?}");
                return Err(RemoveFileError::RequestFailed);
            }
        }

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

    pub async fn remove_group(&mut self, name: String) -> Result<(), RemoveGroupError> {
        let request = tonic::Request::new(RemoveGroupRequest {
            name,
        });

        let result = self.client.remove_group(request).await;

        match result {
            Ok(response) => {
                info!("Server response: {response:?}");
            }
            Err(err) => {
                error!("Request failed with status {err:?}");
                return Err(RemoveGroupError::RequestFailed);
            }
        };

        Ok(())
    }
}

