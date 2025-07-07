use crate::model::Contact;

pub enum ContactListMsg {}
pub enum ContactListOutput {}

pub struct ContactList {
    pub filtered_contacts: Vec<Contact>,
    pub selected_index: usize,
}

impl ContactList {
    pub fn new(contacts: &[Contact]) -> Self {
        Self {
            filtered_contacts: contacts.to_vec(),
            selected_index: 0,
        }
    }
    pub fn get_selected_contact(&self) -> Option<Contact> {
        if self.selected_index < self.filtered_contacts.len() {
            Some(self.filtered_contacts[self.selected_index].clone())
        } else {
            None
        }
    }
}
