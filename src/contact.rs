use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Contact {
    pub name: String,
    pub company: String,
    pub phone: String,
}

impl std::fmt::Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.phone)
    }
}

pub fn load_contacts_from_file<P: AsRef<Path>>(path: P) -> color_eyre::Result<Vec<Contact>> {
    if !path.as_ref().exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(path)?;
    let contacts: Vec<Contact> = serde_json::from_str(&contents)?;
    Ok(contacts)
}
