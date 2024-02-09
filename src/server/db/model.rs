use rusqlite::Row;

#[derive(Debug, Clone)]
pub struct GroupsRecord {
    pub name: String,
    pub prefix: String,
}

impl<'a> TryFrom<&Row<'a>> for GroupsRecord {
    type Error = rusqlite::Error;

    fn try_from(value: &Row) -> Result<Self, Self::Error> {
        let name = match value.get::<usize, String>(0) {
            Ok(name) => name,
            Err(err) => return Err(err),
        };

        let prefix = match value.get::<usize, String>(1) {
            Ok(prefix) => prefix,
            Err(err) => return Err(err),
        };

        Ok(Self { name, prefix })
    }
}

#[derive(Debug, Clone)]
pub struct FileRecord {
    pub group_name: String,
    pub abs_path: String,
}

impl<'a> TryFrom<&Row<'a>> for FileRecord {
    type Error = rusqlite::Error;

    fn try_from(value: &Row<'a>) -> Result<Self, Self::Error> {
        let group_name = value.get::<usize, String>(0)?;
        let abs_path = value.get::<usize, String>(1)?;
        Ok(Self { group_name, abs_path })
    }
}

#[derive(Debug, Clone)]
pub struct InsertFileQuery {
    pub file_path: String,
    pub group_name: String,
}
