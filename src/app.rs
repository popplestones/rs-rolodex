use crate::{Db, mode::AppMode};

pub struct App {
    pub db: Db,
    pub mode: AppMode,
    pub should_quit: bool,
    pub browse: BrowseState,
}

impl App {
    pub fn new(db: Db) -> Self {
        Self {
            db,
            mode: AppMode::Browse,
            should_quit: false,
            browse: BrowseState::default(),
        }
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

#[derive(Debug, Default)]
pub struct Contact {
    pub name: String,
    pub company: String,
    pub phone: String,
    pub email: String,
}
