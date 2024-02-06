use core::panic;
use std::path::{Path, PathBuf};
use log::error;
use rusqlite::Connection;


pub struct DatabaseProxy {
    path: PathBuf,
    conn: Connection,
}

impl DatabaseProxy {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        let conn = match Connection::open(&path) {
            Ok(conn) => conn,
            Err(err) => {
                return Err(anyhow::Error::new(err));
            }
        };
        anyhow::Ok(Self {
            path,
            conn,
        })
    }

    fn create_groups_table(&mut self) -> anyhow::Result<()> {
        let query = include_str!("../../query/create_groups_table.sql");
        let result = self.conn.execute(
            query,
            []
        );
        result.map(|_| ()).map_err(|err| anyhow::Error::new(err))
    }

    fn create_file_table(&mut self) -> anyhow::Result<()> {
        let query = include_str!("../../query/create_files_table.sql");
        let result = self.conn.execute(query, []);
        result.map(|_| ()).map_err(|err| anyhow::Error::new(err))
    }

    pub fn ensure_tables_exist(&mut self) {
        if let Err(err) = self.create_groups_table() {
            error!("Creating groups table failed with error {err:?}");
            panic!("Creating groups table failed with error {err:?}");
        }
        if let Err(err) = self.create_file_table() {
            error!("Creating file table failed with error {err:?}");
            panic!("Creating file table failed with error {err:?}");

        }
    }
}

