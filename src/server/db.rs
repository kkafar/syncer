pub mod model;

use core::panic;
use log::{error, info, warn};
use rusqlite::{params, Connection};
use std::path::PathBuf;

use self::model::{FileRecord, GroupsRecord, InsertFileQuery};

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
        anyhow::Ok(Self { path, conn })
    }

    fn create_groups_table(&mut self) -> anyhow::Result<()> {
        let query = include_str!("../../query/create_groups_table.sql");
        let result = self.conn.execute(query, []);
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

    pub fn insert_group(&mut self, record: GroupsRecord) -> anyhow::Result<()> {
        let result = self.conn.execute(
            "INSERT INTO groups (name, prefix) VALUES (?1, ?2);",
            params![record.name, record.prefix],
        );

        match result {
            Ok(count) => {
                info!("Group successfully inserted, altered {count} rows");
                Ok(())
            }
            Err(err) => {
                warn!("Group insertion failed with error {err:?}");
                Err(anyhow::Error::new(err))
            }
        }
    }

    pub fn delete_group(&mut self, name: impl AsRef<str>) -> anyhow::Result<()> {
        let name = name.as_ref();

        match self
            .conn
            .execute("DELETE FROM groups WHERE name = ?1", [name])
        {
            Ok(count) => {
                info!("Group successfully inserted, altered {count} rows");
                Ok(())
            }
            Err(err) => {
                warn!("Group deletion failed with error {err:?}");
                Err(anyhow::Error::new(err))
            }
        }
    }

    pub fn list_groups(&mut self) -> anyhow::Result<Vec<GroupsRecord>> {
        let mut stmt = self.conn.prepare("SELECT name, prefix FROM groups")?;
        let rows = stmt
            .query_map([], |row| GroupsRecord::try_from(row))?
            .filter_map(Result::ok);

        Ok(rows.collect())
    }

    pub fn list_files(&mut self) -> anyhow::Result<Vec<FileRecord>> {
        let mut stmt = self
            .conn
            .prepare("SELECT group_name, abs_path FROM files")?;
        let rows = stmt
            .query_map([], |row| FileRecord::try_from(row))?
            .filter_map(Result::ok);

        Ok(rows.collect())
    }

    pub fn insert_file(&mut self, query_data: InsertFileQuery) -> anyhow::Result<()> {
        // We need to also find last modified time
        let last_modified_time = std::fs::metadata(&query_data.file_path)?.modified()?;
        let timestamp = last_modified_time
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        // We ignore file's hash right now

        let result = self.conn.execute(
            "INSERT INTO files (group_name, abs_path, lmtime, hash) VALUES (?1, ?2, ?3, ?4);",
            params![
                query_data.group_name,
                query_data.file_path,
                timestamp,
                None::<String>
            ],
        );
        match result {
            Ok(count) => {
                info!("File successfully inserted, altered {count} rows");
                Ok(())
            }
            Err(err) => {
                warn!("File insertion failed with error {err:?}");
                Err(anyhow::Error::new(err))
            }
        }
    }
}
