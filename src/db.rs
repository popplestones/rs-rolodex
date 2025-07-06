use std::path::Path;

use crate::{
    error::{AppError, AppResult as Result},
    model::Contact,
};
use rand::Rng;
use rusqlite::{Connection, OpenFlags};
use tracing::info;

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::Database(format!("create db dir: {e}")))?;
        }

        let is_new = !path.exists();

        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )
        .map_err(|e| AppError::Database(format!("open: {e}")))?;

        if is_new {
            Self::init_schema(&conn)?;
        }

        Ok(Self { conn })
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    fn init_schema(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            r#"
            CREATE TABLE contacts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                email TEXT,
                phone TEXT,
                company TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            "#,
        )
        .map_err(|e| AppError::Database(format!("create schema: {e}")))?;

        Ok(())
    }
    pub fn seed(&mut self, count: u32) -> Result<()> {
        let tx = self
            .conn
            .transaction()
            .map_err(|e| AppError::Database(format!("begin tx: {e}")))?;

        let mut rng = rand::rng();
        let mut name_gen = names::Generator::default();
        let mut company_gen = names::Generator::with_naming(names::Name::Numbered);

        for _ in 0..count {
            let name = name_gen.next().unwrap_or("John Doe".into());
            let email = format!("{}@example.com", name.replace(' ', "").to_lowercase());
            let phone = format!("04{:08}", rng.random_range(0..=99999999));
            let company = format!("{} Pty Ltd", company_gen.next().unwrap_or("Acme".into()));

            tx.execute(
                "INSERT INTO contacts (name, email, phone, company) VALUES (?, ?, ?, ?)",
                (&name, &email, &phone, &company),
            )
            .map_err(|e| AppError::Database(format!("insert fake: {e}")))?;
        }
        tx.commit()
            .map_err(|e| AppError::Database(format!("commit tx: {e}")))?;

        Ok(())
    }

    pub fn add_contact(&self, contact: Contact) -> Result<()> {
        info!("Save contact: {:?}", contact);
        self.conn
            .execute(
                "INSERT INTO contacts (name, email, phone, company) VALUES (?, ?, ?, ?)",
                (
                    &contact.name,
                    &contact.email,
                    &contact.phone,
                    &contact.company,
                ),
            )
            .map_err(|e| AppError::Database(format!("insert: {e}")))?;
        Ok(())
    }

    pub fn load_customers(&self) -> Result<Vec<Contact>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, email, phone, company from contacts order by name asc")
            .map_err(|e| AppError::Database(format!("prepare load: {e}")))?;

        let rows = stmt
            .query_map([], Contact::from_row)
            .map_err(|e| AppError::Database(format!("query map: {e}")))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| AppError::Database(format!("collect: {e}")))?;

        Ok(rows)
    }

    pub fn delete_contact(&self, id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM contacts WHERE id = ?", [id])
            .map_err(|e| AppError::Database(format!("delete: {e}")))?;
        info!("Deleted contact with id: {}", id);
        Ok(())
    }

    pub fn update_contact(&self, id: i64, contact: Contact) -> Result<()> {
        self.conn
            .execute(
                "UPDATE contacts SET name = ?, email = ?, phone = ?, company = ? WHERE id = ?",
                (
                    &contact.name,
                    &contact.email,
                    &contact.phone,
                    &contact.company,
                    id,
                ),
            )
            .map_err(|e| AppError::Database(format!("update: {e}")))?;
        info!("Updated contact with id: {}", id);
        Ok(())
    }
}
