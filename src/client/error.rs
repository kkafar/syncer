use thiserror::{self, Error};

#[derive(Debug, Error)]
pub enum AddFileError {
    #[error("File does not exist")]
    FileNotExists,

    #[error("Network request failed")]
    RequestFailed,
}

#[derive(Debug, Error)]
pub enum AddGroupError {
    #[error("Group with name {0} already exists")]
    GroupExists(String),


    #[error("Network request failed")]
    RequestFailed,
}

#[derive(Debug, Error)]
pub enum RemoveGroupError {
    #[error("Network request failed")]
    RequestFailed,
}

#[derive(Debug, Error)]
pub enum RemoveFileError {
    #[error("Network request failed")]
    RequestFailed,
}


#[derive(Debug, Error)]
pub enum ListFilesError {
    #[error("Network request failed")]
    RequestFailed,
}
