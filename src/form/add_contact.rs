use crate::{model::Contact, ui::components::text_field::TextField};

use crossterm::event::KeyCode;

pub struct AddContactForm {
    pub fields: Vec<TextField>,
    pub focused: usize,
}

impl AddContactForm {
    pub fn new() -> Self {
        Self {
            fields: vec![
                TextField::new("Name"),
                TextField::new("Company"),
                TextField::new("Email"),
                TextField::new("Phone"),
            ],
            focused: 0,
        }
    }
    pub fn handle_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Tab => {
                if self.focused < self.fields.len() - 1 {
                    self.focused += 1;
                } else {
                    self.focused = 0;
                }
            }
            KeyCode::BackTab => {
                if self.focused > 0 {
                    self.focused -= 1;
                } else {
                    self.focused = self.fields.len() - 1;
                }
            }
            _ => {
                if let Some(field) = self.fields.get_mut(self.focused) {
                    field.handle_key(code);
                }
            }
        }
    }
    pub fn to_contact(&self) -> Contact {
        Contact {
            id: 0,
            name: self.fields[0].value.trim().to_string(),
            company: opt(self.fields[1].value.trim()),
            email: opt(self.fields[2].value.trim()),
            phone: opt(self.fields[3].value.trim()),
        }
    }

    pub fn clear(&mut self) {
        for field in self.fields.iter_mut() {
            field.value.clear();
        }
        self.focused = 0;
    }
}

fn opt(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}
