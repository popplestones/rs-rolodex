use serde::Serialize;

#[derive(Serialize, Debug, Default, Clone)]
pub struct Contact {
    pub id: i64,
    pub name: String,
    pub company: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

impl Contact {
    pub fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            phone: row.get(3)?,
            company: row.get(4)?,
        })
    }
}
