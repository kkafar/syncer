use thiserror::{self, Error};

#[derive(Debug, Error)]
pub enum AddFileError {
    #[error("File does not exist")]
    FileNotExists,

    #[error("Request failed")]
    RequestFailed,
}
