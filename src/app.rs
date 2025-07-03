use crate::error::AppResult as Result;
use crate::{Db, mode::AppMode, model::contact::Contact};
pub struct App {
    pub db: Db,
    pub mode: AppMode,
    pub should_quit: bool,
    pub browse: BrowseState,
}

impl App {
    pub fn new(db: Db) -> Result<Self> {
        let contacts = db.load_customers()?;
        let browse = BrowseState::new(contacts);
        Ok(Self {
            db,
            mode: AppMode::Browse,
            should_quit: false,
            browse,
        })
    }

    pub fn set_error(&mut self, _msg: impl Into<String>) {
        self.mode = AppMode::Error;
    }

    pub fn clear_error(&mut self) {
        if self.mode == AppMode::Error {
            self.mode = AppMode::Browse;
        }
    }
}

#[derive(Debug, Default)]
pub struct BrowseState {
    pub search_input: String,
    pub all_contacts: Vec<Contact>,
    pub filtered_contacts: Vec<Contact>,
    pub selected_index: usize,
}

impl BrowseState {
    pub fn new(contacts: Vec<Contact>) -> Self {
        Self {
            all_contacts: contacts.clone(),
            filtered_contacts: contacts,
            ..Default::default()
        }
    }
}
