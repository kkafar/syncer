use std::path::PathBuf;
use serde::{self, Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ServerDescription {
    pub path: PathBuf,
}


struct Server {

}

