use crate::env::AppDirectories;
use crate::server::db::DatabaseProxy;
use rusqlite::Connection;
use xdg::BaseDirectories;

pub struct Context {
    pub app_dirs: AppDirectories,
    pub db: Option<DatabaseProxy>, // Present only in case we run the server
}

impl Context {
    pub fn new(app_dirs: AppDirectories, db: Option<DatabaseProxy>) -> Self {
        Self { app_dirs, db }
    }
}
