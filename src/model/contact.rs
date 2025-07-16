use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum ValidationError {
    EmptyName,
    InvalidEmail(String),
    InvalidPhone(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::EmptyName => write!(f, "Name cannot be empty"),
            ValidationError::InvalidEmail(email) => write!(f, "Invalid email format: {}", email),
            ValidationError::InvalidPhone(phone) => write!(f, "Invalid phone format: {}", phone),
        }
    }
}

impl std::error::Error for ValidationError {}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
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

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName);
        }

        if let Some(email) = &self.email {
            if !email.trim().is_empty() && !Self::is_valid_email(email) {
                return Err(ValidationError::InvalidEmail(email.clone()));
            }
        }

        if let Some(phone) = &self.phone {
            if !phone.trim().is_empty() && !Self::is_valid_phone(phone) {
                return Err(ValidationError::InvalidPhone(phone.clone()));
            }
        }

        Ok(())
    }

    fn is_valid_email(email: &str) -> bool {
        let email = email.trim();
        if email.len() <= 5 {
            return false;
        }
        
        let at_pos = email.find('@');
        let dot_pos = email.rfind('.');
        
        match (at_pos, dot_pos) {
            (Some(at), Some(dot)) => {
                at > 0 && dot > at + 1 && dot < email.len() - 1
            }
            _ => false,
        }
    }

    fn is_valid_phone(phone: &str) -> bool {
        let cleaned = phone.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
        cleaned.len() >= 8 && cleaned.len() <= 15
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_default() {
        let contact = Contact::default();
        assert_eq!(contact.id, 0);
        assert_eq!(contact.name, "");
        assert_eq!(contact.company, None);
        assert_eq!(contact.phone, None);
        assert_eq!(contact.email, None);
    }

    #[test]
    fn test_contact_creation() {
        let contact = Contact {
            id: 1,
            name: "John Doe".to_string(),
            company: Some("Acme Corp".to_string()),
            phone: Some("0412345678".to_string()),
            email: Some("john@acme.com".to_string()),
        };

        assert_eq!(contact.id, 1);
        assert_eq!(contact.name, "John Doe");
        assert_eq!(contact.company, Some("Acme Corp".to_string()));
        assert_eq!(contact.phone, Some("0412345678".to_string()));
        assert_eq!(contact.email, Some("john@acme.com".to_string()));
    }

    #[test]
    fn test_contact_clone() {
        let contact1 = Contact {
            id: 1,
            name: "Jane Smith".to_string(),
            company: Some("Tech Inc".to_string()),
            phone: Some("0487654321".to_string()),
            email: Some("jane@tech.com".to_string()),
        };

        let contact2 = contact1.clone();
        assert_eq!(contact1, contact2);
    }

    #[test]
    fn test_contact_serialization() {
        let contact = Contact {
            id: 1,
            name: "Alice Johnson".to_string(),
            company: Some("StartupXYZ".to_string()),
            phone: Some("0411111111".to_string()),
            email: Some("alice@startup.com".to_string()),
        };

        let json = serde_json::to_string(&contact).unwrap();
        assert!(json.contains("Alice Johnson"));
        assert!(json.contains("StartupXYZ"));
        assert!(json.contains("0411111111"));
        assert!(json.contains("alice@startup.com"));
    }

    #[test]
    fn test_contact_validation_valid() {
        let contact = Contact {
            id: 1,
            name: "John Doe".to_string(),
            company: Some("Acme Corp".to_string()),
            phone: Some("0412345678".to_string()),
            email: Some("john@acme.com".to_string()),
        };

        assert!(contact.validate().is_ok());
    }

    #[test]
    fn test_contact_validation_empty_name() {
        let contact = Contact {
            id: 1,
            name: "".to_string(),
            company: Some("Acme Corp".to_string()),
            phone: Some("0412345678".to_string()),
            email: Some("john@acme.com".to_string()),
        };

        assert!(matches!(contact.validate(), Err(ValidationError::EmptyName)));
    }

    #[test]
    fn test_contact_validation_invalid_email() {
        let contact = Contact {
            id: 1,
            name: "John Doe".to_string(),
            company: Some("Acme Corp".to_string()),
            phone: Some("0412345678".to_string()),
            email: Some("invalid-email".to_string()),
        };

        assert!(matches!(contact.validate(), Err(ValidationError::InvalidEmail(_))));
    }

    #[test]
    fn test_contact_validation_invalid_phone() {
        let contact = Contact {
            id: 1,
            name: "John Doe".to_string(),
            company: Some("Acme Corp".to_string()),
            phone: Some("123".to_string()),
            email: Some("john@acme.com".to_string()),
        };

        assert!(matches!(contact.validate(), Err(ValidationError::InvalidPhone(_))));
    }

    #[test]
    fn test_contact_validation_optional_fields() {
        let contact = Contact {
            id: 1,
            name: "John Doe".to_string(),
            company: None,
            phone: None,
            email: None,
        };

        assert!(contact.validate().is_ok());
    }

    #[test]
    fn test_email_validation() {
        assert!(Contact::is_valid_email("test@example.com"));
        assert!(Contact::is_valid_email("user.name@domain.co.uk"));
        assert!(!Contact::is_valid_email("invalid"));
        assert!(!Contact::is_valid_email("@domain.com"));
        assert!(!Contact::is_valid_email("user@"));
        assert!(!Contact::is_valid_email(""));
    }

    #[test]
    fn test_phone_validation() {
        assert!(Contact::is_valid_phone("0412345678"));
        assert!(Contact::is_valid_phone("+61 412 345 678"));
        assert!(Contact::is_valid_phone("(04) 1234-5678"));
        assert!(!Contact::is_valid_phone("123"));
        assert!(!Contact::is_valid_phone(""));
        assert!(!Contact::is_valid_phone("abc"));
    }
}
