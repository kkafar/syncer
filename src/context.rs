use std::sync::Arc;

use crate::env::AppDirectories;
use crate::server::db::DatabaseProxy;
use std::sync::Mutex;

pub struct Context {
    pub app_dirs: AppDirectories,
    pub db: Arc<Mutex<Option<DatabaseProxy>>>, // Present only in case we run the server
}

impl Context {
    pub fn new(app_dirs: AppDirectories, db: Option<DatabaseProxy>) -> Self {
        Self { app_dirs, db: Arc::new(Mutex::new(db)) }
    }

    pub fn inject_db(&self, db: DatabaseProxy) {
        self.db.lock()
            .expect("Error while acquiring lock for db injection")
            .insert(db);
    }
}
