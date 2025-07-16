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

    pub fn search_contacts(&self, query: &str) -> Result<Vec<Contact>> {
        let search_term = format!("%{}%", query.to_lowercase());
        
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, email, phone, company 
                 FROM contacts 
                 WHERE LOWER(name) LIKE ?1 
                    OR LOWER(COALESCE(company, '')) LIKE ?1 
                    OR LOWER(COALESCE(email, '')) LIKE ?1 
                    OR LOWER(COALESCE(phone, '')) LIKE ?1
                 ORDER BY name ASC"
            )
            .map_err(|e| AppError::Database(format!("prepare search: {e}")))?;

        let rows = stmt
            .query_map([&search_term], Contact::from_row)
            .map_err(|e| AppError::Database(format!("search query map: {e}")))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| AppError::Database(format!("search collect: {e}")))?;

        Ok(rows)
    }

    pub fn search_by_field(&self, field: &str, query: &str) -> Result<Vec<Contact>> {
        let search_term = format!("%{}%", query.to_lowercase());
        
        let sql = match field {
            "name" => "SELECT id, name, email, phone, company FROM contacts WHERE LOWER(name) LIKE ?1 ORDER BY name ASC",
            "company" => "SELECT id, name, email, phone, company FROM contacts WHERE LOWER(COALESCE(company, '')) LIKE ?1 ORDER BY name ASC",
            "email" => "SELECT id, name, email, phone, company FROM contacts WHERE LOWER(COALESCE(email, '')) LIKE ?1 ORDER BY name ASC",
            "phone" => "SELECT id, name, email, phone, company FROM contacts WHERE LOWER(COALESCE(phone, '')) LIKE ?1 ORDER BY name ASC",
            _ => return Err(AppError::Database(format!("invalid search field: {}", field))),
        };

        let mut stmt = self
            .conn
            .prepare(sql)
            .map_err(|e| AppError::Database(format!("prepare field search: {e}")))?;

        let rows = stmt
            .query_map([&search_term], Contact::from_row)
            .map_err(|e| AppError::Database(format!("field search query map: {e}")))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| AppError::Database(format!("field search collect: {e}")))?;

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

    pub fn export_contacts_json(&self) -> Result<String> {
        let contacts = self.load_customers()?;
        serde_json::to_string_pretty(&contacts)
            .map_err(|e| AppError::Database(format!("export json: {e}")))
    }

    pub fn import_contacts_json(&mut self, json_data: &str) -> Result<usize> {
        let contacts: Vec<Contact> = serde_json::from_str(json_data)
            .map_err(|e| AppError::Database(format!("parse json: {e}")))?;

        let tx = self
            .conn
            .transaction()
            .map_err(|e| AppError::Database(format!("begin import tx: {e}")))?;

        let mut imported_count = 0;
        for contact in contacts {
            if let Err(validation_err) = contact.validate() {
                info!("Skipping invalid contact: {}", validation_err);
                continue;
            }

            tx.execute(
                "INSERT INTO contacts (name, email, phone, company) VALUES (?, ?, ?, ?)",
                (
                    &contact.name,
                    &contact.email,
                    &contact.phone,
                    &contact.company,
                ),
            )
            .map_err(|e| AppError::Database(format!("import contact: {e}")))?;
            
            imported_count += 1;
        }

        tx.commit()
            .map_err(|e| AppError::Database(format!("commit import tx: {e}")))?;

        info!("Imported {} contacts", imported_count);
        Ok(imported_count)
    }

    pub fn backup_to_file<P: AsRef<Path>>(&self, backup_path: P) -> Result<()> {
        let json_data = self.export_contacts_json()?;
        std::fs::write(backup_path, json_data)
            .map_err(|e| AppError::Database(format!("write backup: {e}")))?;
        Ok(())
    }

    pub fn restore_from_file<P: AsRef<Path>>(&mut self, backup_path: P) -> Result<usize> {
        let json_data = std::fs::read_to_string(backup_path)
            .map_err(|e| AppError::Database(format!("read backup: {e}")))?;
        self.import_contacts_json(&json_data)
    }

    pub fn find_potential_duplicates(&self) -> Result<Vec<Vec<Contact>>> {
        let all_contacts = self.load_customers()?;
        let mut duplicates = Vec::new();
        let mut processed = std::collections::HashSet::new();

        for (i, contact) in all_contacts.iter().enumerate() {
            if processed.contains(&i) {
                continue;
            }

            let mut duplicate_group = vec![contact.clone()];
            
            for (j, other_contact) in all_contacts.iter().enumerate().skip(i + 1) {
                if processed.contains(&j) {
                    continue;
                }

                if Self::are_potential_duplicates(contact, other_contact) {
                    duplicate_group.push(other_contact.clone());
                    processed.insert(j);
                }
            }

            if duplicate_group.len() > 1 {
                duplicates.push(duplicate_group);
                processed.insert(i);
            }
        }

        Ok(duplicates)
    }

    fn are_potential_duplicates(contact1: &Contact, contact2: &Contact) -> bool {
        let name_similarity = Self::calculate_name_similarity(&contact1.name, &contact2.name);
        
        let email_match = match (&contact1.email, &contact2.email) {
            (Some(e1), Some(e2)) => e1.to_lowercase() == e2.to_lowercase(),
            _ => false,
        };

        let phone_match = match (&contact1.phone, &contact2.phone) {
            (Some(p1), Some(p2)) => {
                let clean_p1: String = p1.chars().filter(|c| c.is_ascii_digit()).collect();
                let clean_p2: String = p2.chars().filter(|c| c.is_ascii_digit()).collect();
                clean_p1 == clean_p2 && !clean_p1.is_empty()
            }
            _ => false,
        };

        name_similarity > 0.8 || email_match || phone_match
    }

    fn calculate_name_similarity(name1: &str, name2: &str) -> f64 {
        let name1 = name1.to_lowercase();
        let name2 = name2.to_lowercase();
        
        if name1 == name2 {
            return 1.0;
        }

        let words1: std::collections::HashSet<&str> = name1.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> = name2.split_whitespace().collect();
        
        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();
        
        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    fn create_test_db() -> (Db, tempfile::TempDir) {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Db::open(&db_path).unwrap();
        (db, temp_dir)
    }

    #[test]
    fn test_db_creation() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        
        assert!(!db_path.exists());
        let _db = Db::open(&db_path).unwrap();
        assert!(db_path.exists());
    }

    #[test]
    fn test_add_contact() {
        let (db, _temp_dir) = create_test_db();
        
        let contact = Contact {
            id: 0,
            name: "Test User".to_string(),
            email: Some("test@example.com".to_string()),
            phone: Some("0412345678".to_string()),
            company: Some("Test Corp".to_string()),
        };

        let result = db.add_contact(contact);
        assert!(result.is_ok());
    }

    #[test]
    fn test_load_customers() {
        let (db, _temp_dir) = create_test_db();
        
        let contact = Contact {
            id: 0,
            name: "Alice Smith".to_string(),
            email: Some("alice@example.com".to_string()),
            phone: Some("0487654321".to_string()),
            company: Some("Alice Corp".to_string()),
        };

        db.add_contact(contact).unwrap();
        
        let contacts = db.load_customers().unwrap();
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].name, "Alice Smith");
        assert_eq!(contacts[0].email, Some("alice@example.com".to_string()));
    }

    #[test]
    fn test_update_contact() {
        let (db, _temp_dir) = create_test_db();
        
        let contact = Contact {
            id: 0,
            name: "Original Name".to_string(),
            email: Some("original@example.com".to_string()),
            phone: Some("0411111111".to_string()),
            company: Some("Original Corp".to_string()),
        };

        db.add_contact(contact).unwrap();
        let contacts = db.load_customers().unwrap();
        let contact_id = contacts[0].id;

        let updated_contact = Contact {
            id: contact_id,
            name: "Updated Name".to_string(),
            email: Some("updated@example.com".to_string()),
            phone: Some("0422222222".to_string()),
            company: Some("Updated Corp".to_string()),
        };

        let result = db.update_contact(contact_id, updated_contact);
        assert!(result.is_ok());

        let contacts = db.load_customers().unwrap();
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].name, "Updated Name");
        assert_eq!(contacts[0].email, Some("updated@example.com".to_string()));
    }

    #[test]
    fn test_delete_contact() {
        let (db, _temp_dir) = create_test_db();
        
        let contact = Contact {
            id: 0,
            name: "To Delete".to_string(),
            email: Some("delete@example.com".to_string()),
            phone: Some("0433333333".to_string()),
            company: Some("Delete Corp".to_string()),
        };

        db.add_contact(contact).unwrap();
        let contacts = db.load_customers().unwrap();
        assert_eq!(contacts.len(), 1);
        
        let contact_id = contacts[0].id;
        let result = db.delete_contact(contact_id);
        assert!(result.is_ok());

        let contacts = db.load_customers().unwrap();
        assert_eq!(contacts.len(), 0);
    }

    #[test]
    fn test_seed_contacts() {
        let (mut db, _temp_dir) = create_test_db();
        
        let result = db.seed(5);
        assert!(result.is_ok());

        let contacts = db.load_customers().unwrap();
        assert_eq!(contacts.len(), 5);
        
        for contact in contacts {
            assert!(!contact.name.is_empty());
            assert!(contact.email.is_some());
            assert!(contact.phone.is_some());
            assert!(contact.company.is_some());
        }
    }

    #[test]
    fn test_contacts_sorted_by_name() {
        let (db, _temp_dir) = create_test_db();
        
        let contacts = vec![
            Contact {
                id: 0,
                name: "Zoe".to_string(),
                email: Some("zoe@example.com".to_string()),
                phone: Some("0411111111".to_string()),
                company: Some("Z Corp".to_string()),
            },
            Contact {
                id: 0,
                name: "Alice".to_string(),
                email: Some("alice@example.com".to_string()),
                phone: Some("0422222222".to_string()),
                company: Some("A Corp".to_string()),
            },
            Contact {
                id: 0,
                name: "Bob".to_string(),
                email: Some("bob@example.com".to_string()),
                phone: Some("0433333333".to_string()),
                company: Some("B Corp".to_string()),
            },
        ];

        for contact in contacts {
            db.add_contact(contact).unwrap();
        }

        let loaded_contacts = db.load_customers().unwrap();
        assert_eq!(loaded_contacts.len(), 3);
        assert_eq!(loaded_contacts[0].name, "Alice");
        assert_eq!(loaded_contacts[1].name, "Bob");
        assert_eq!(loaded_contacts[2].name, "Zoe");
    }

    #[test]
    fn test_export_contacts_json() {
        let (db, _temp_dir) = create_test_db();
        
        let contact = Contact {
            id: 0,
            name: "Export Test".to_string(),
            email: Some("export@test.com".to_string()),
            phone: Some("0444444444".to_string()),
            company: Some("Export Corp".to_string()),
        };

        db.add_contact(contact).unwrap();
        
        let json = db.export_contacts_json().unwrap();
        assert!(json.contains("Export Test"));
        assert!(json.contains("export@test.com"));
        assert!(json.contains("Export Corp"));
    }

    #[test]
    fn test_import_contacts_json() {
        let (mut db, _temp_dir) = create_test_db();
        
        let json_data = r#"[
            {
                "id": 0,
                "name": "Import Test 1",
                "email": "import1@test.com",
                "phone": "0455555555",
                "company": "Import Corp 1"
            },
            {
                "id": 0,
                "name": "Import Test 2",
                "email": "import2@test.com",
                "phone": "0466666666",
                "company": "Import Corp 2"
            }
        ]"#;

        let imported_count = db.import_contacts_json(json_data).unwrap();
        assert_eq!(imported_count, 2);

        let contacts = db.load_customers().unwrap();
        assert_eq!(contacts.len(), 2);
        assert_eq!(contacts[0].name, "Import Test 1");
        assert_eq!(contacts[1].name, "Import Test 2");
    }

    #[test]
    fn test_backup_and_restore() {
        let (db, temp_dir) = create_test_db();
        
        let contact = Contact {
            id: 0,
            name: "Backup Test".to_string(),
            email: Some("backup@test.com".to_string()),
            phone: Some("0477777777".to_string()),
            company: Some("Backup Corp".to_string()),
        };

        db.add_contact(contact).unwrap();
        
        let backup_path = temp_dir.path().join("backup.json");
        db.backup_to_file(&backup_path).unwrap();
        assert!(backup_path.exists());

        let (mut new_db, _new_temp_dir) = create_test_db();
        let restored_count = new_db.restore_from_file(&backup_path).unwrap();
        assert_eq!(restored_count, 1);

        let contacts = new_db.load_customers().unwrap();
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].name, "Backup Test");
    }

    #[test]
    fn test_import_invalid_contacts() {
        let (mut db, _temp_dir) = create_test_db();
        
        let json_data = r#"[
            {
                "id": 0,
                "name": "",
                "email": "invalid@test.com",
                "phone": "0455555555",
                "company": "Invalid Corp"
            },
            {
                "id": 0,
                "name": "Valid Contact",
                "email": "valid@test.com",
                "phone": "0466666666",
                "company": "Valid Corp"
            }
        ]"#;

        let imported_count = db.import_contacts_json(json_data).unwrap();
        assert_eq!(imported_count, 1);

        let contacts = db.load_customers().unwrap();
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].name, "Valid Contact");
    }

    #[test]
    fn test_search_contacts() {
        let (db, _temp_dir) = create_test_db();
        
        let contacts = vec![
            Contact {
                id: 0,
                name: "Alice Smith".to_string(),
                email: Some("alice@acme.com".to_string()),
                phone: Some("0411111111".to_string()),
                company: Some("Acme Corp".to_string()),
            },
            Contact {
                id: 0,
                name: "Bob Johnson".to_string(),
                email: Some("bob@tech.com".to_string()),
                phone: Some("0422222222".to_string()),
                company: Some("Tech Inc".to_string()),
            },
            Contact {
                id: 0,
                name: "Charlie Brown".to_string(),
                email: Some("charlie@acme.com".to_string()),
                phone: Some("0433333333".to_string()),
                company: Some("Acme Corp".to_string()),
            },
        ];

        for contact in contacts {
            db.add_contact(contact).unwrap();
        }

        let results = db.search_contacts("acme").unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].name, "Alice Smith");
        assert_eq!(results[1].name, "Charlie Brown");

        let results = db.search_contacts("tech").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Bob Johnson");

        let results = db.search_contacts("smith").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Alice Smith");
    }

    #[test]
    fn test_search_by_field() {
        let (db, _temp_dir) = create_test_db();
        
        let contact = Contact {
            id: 0,
            name: "John Doe".to_string(),
            email: Some("john@example.com".to_string()),
            phone: Some("0444444444".to_string()),
            company: Some("Example Corp".to_string()),
        };

        db.add_contact(contact).unwrap();

        let results = db.search_by_field("name", "john").unwrap();
        assert_eq!(results.len(), 1);

        let results = db.search_by_field("company", "example").unwrap();
        assert_eq!(results.len(), 1);

        let results = db.search_by_field("email", "example").unwrap();
        assert_eq!(results.len(), 1);

        let results = db.search_by_field("phone", "0444").unwrap();
        assert_eq!(results.len(), 1);

        let results = db.search_by_field("name", "nonexistent").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_by_invalid_field() {
        let (db, _temp_dir) = create_test_db();
        
        let result = db.search_by_field("invalid_field", "query");
        assert!(result.is_err());
    }

    #[test]
    fn test_find_potential_duplicates() {
        let (db, _temp_dir) = create_test_db();
        
        let contacts = vec![
            Contact {
                id: 0,
                name: "John Smith".to_string(),
                email: Some("john@example.com".to_string()),
                phone: Some("0411111111".to_string()),
                company: Some("Acme Corp".to_string()),
            },
            Contact {
                id: 0,
                name: "John Smith".to_string(),
                email: Some("john.smith@example.com".to_string()),
                phone: Some("0411111111".to_string()),
                company: Some("Acme Corp".to_string()),
            },
            Contact {
                id: 0,
                name: "Jane Doe".to_string(),
                email: Some("jane@example.com".to_string()),
                phone: Some("0422222222".to_string()),
                company: Some("Tech Inc".to_string()),
            },
            Contact {
                id: 0,
                name: "Bob Johnson".to_string(),
                email: Some("bob@example.com".to_string()),
                phone: Some("0433333333".to_string()),
                company: Some("Other Corp".to_string()),
            },
        ];

        for contact in contacts {
            db.add_contact(contact).unwrap();
        }

        let duplicates = db.find_potential_duplicates().unwrap();
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0].len(), 2);
        assert_eq!(duplicates[0][0].name, "John Smith");
        assert_eq!(duplicates[0][1].name, "John Smith");
    }

    #[test]
    fn test_duplicate_detection_by_email() {
        let (db, _temp_dir) = create_test_db();
        
        let contacts = vec![
            Contact {
                id: 0,
                name: "John Smith".to_string(),
                email: Some("john@example.com".to_string()),
                phone: Some("0411111111".to_string()),
                company: Some("Acme Corp".to_string()),
            },
            Contact {
                id: 0,
                name: "J. Smith".to_string(),
                email: Some("john@example.com".to_string()),
                phone: Some("0422222222".to_string()),
                company: Some("Different Corp".to_string()),
            },
        ];

        for contact in contacts {
            db.add_contact(contact).unwrap();
        }

        let duplicates = db.find_potential_duplicates().unwrap();
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0].len(), 2);
    }

    #[test]
    fn test_duplicate_detection_by_phone() {
        let (db, _temp_dir) = create_test_db();
        
        let contacts = vec![
            Contact {
                id: 0,
                name: "John Smith".to_string(),
                email: Some("john1@example.com".to_string()),
                phone: Some("0411111111".to_string()),
                company: Some("Acme Corp".to_string()),
            },
            Contact {
                id: 0,
                name: "Different Name".to_string(),
                email: Some("different@example.com".to_string()),
                phone: Some("(04) 1111-1111".to_string()),
                company: Some("Different Corp".to_string()),
            },
        ];

        for contact in contacts {
            db.add_contact(contact).unwrap();
        }

        let duplicates = db.find_potential_duplicates().unwrap();
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0].len(), 2);
    }

    #[test]
    fn test_no_duplicates() {
        let (db, _temp_dir) = create_test_db();
        
        let contacts = vec![
            Contact {
                id: 0,
                name: "John Smith".to_string(),
                email: Some("john@example.com".to_string()),
                phone: Some("0411111111".to_string()),
                company: Some("Acme Corp".to_string()),
            },
            Contact {
                id: 0,
                name: "Jane Doe".to_string(),
                email: Some("jane@example.com".to_string()),
                phone: Some("0422222222".to_string()),
                company: Some("Tech Inc".to_string()),
            },
        ];

        for contact in contacts {
            db.add_contact(contact).unwrap();
        }

        let duplicates = db.find_potential_duplicates().unwrap();
        assert_eq!(duplicates.len(), 0);
    }
}
