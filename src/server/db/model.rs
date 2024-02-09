use rusqlite::Row;

#[derive(Debug, Clone)]
pub struct GroupsRecord {
    pub name: String,
    pub prefix: String,
}

impl <'a> TryFrom<&Row<'a>> for GroupsRecord {
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
