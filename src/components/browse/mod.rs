use crate::{
    components::{contact_list::ContactList, search::Search},
    model::Contact,
};

pub enum BrowseMsg {}
pub enum BrowseOutput {}

pub struct Browse {
    pub search: Search,
    pub contact_list: ContactList,
}

impl Browse {
    pub fn new(contacts: &[Contact]) -> Self {
        Self {
            search: Search::new(),
            contact_list: ContactList::new(contacts),
        }
    }
}
