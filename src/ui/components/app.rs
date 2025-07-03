use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};

use crate::{
    Db,
    error::AppResult as Result,
    mode::AppMode,
    model::Contact,
    ui::components::{
        Component, add_contact::AddContactForm, delete_confirmation::DeleteConfirmation,
    },
    view::browse,
    view::error,
};

pub struct App {
    pub db: Db,
    pub selected_contact: Option<Contact>,
    pub all_contacts: Vec<Contact>,
    pub mode: AppMode,
    pub should_quit: bool,
    pub browse: BrowseState,
    pub add_contact_form: AddContactForm,
}

impl App {
    pub fn new(db: Db) -> Result<Self> {
        let all_contacts = db.load_customers()?;
        let browse = BrowseState::new(&all_contacts);
        Ok(Self {
            db,
            selected_contact: None,
            all_contacts,
            mode: AppMode::Browse,
            should_quit: false,
            browse,
            add_contact_form: AddContactForm::new(),
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
    pub fn select_contact(&mut self) {
        self.selected_contact =
            Some(self.browse.filtered_contacts[self.browse.selected_index].clone());
    }
    pub fn unselect_contact(&mut self) {
        self.selected_contact = None;
    }
}

impl Component for App {
    fn draw(&self, f: &mut ratatui::Frame, rect: ratatui::prelude::Rect, is_focussed: bool) {
        // Step 1: Draw main browse ui

        browse::draw(f, self);

        // Step 2: overlay mode-specific view (like modals)
        match self.mode {
            AppMode::Error => error::draw(f, self),
            AppMode::DeleteConfirmation => DeleteConfirmation::draw(f, self),
            AppMode::AddContact => self.add_contact_form.draw(f, f.area(), false),
            _ => {}
        }
    }

    fn handle_key(&mut self, code: crossterm::event::KeyCode) {
        todo!()
    }
}
#[derive(Debug, Default)]
pub struct BrowseState {
    pub search_input: String,
    pub filtered_contacts: Vec<Contact>,
    pub selected_index: usize,
}

impl BrowseState {
    pub fn new(contacts: &[Contact]) -> Self {
        Self {
            filtered_contacts: contacts.to_vec().clone(),
            ..Default::default()
        }
    }

    pub fn update_filter(&mut self, all_contacts: &[Contact]) {
        let matcher = SkimMatcherV2::default();

        if self.search_input.trim().is_empty() {
            self.filtered_contacts = all_contacts.to_vec();
            self.selected_index = 0;
            return;
        }

        let mut matches: Vec<(i64, &Contact)> = all_contacts
            .iter()
            .filter_map(|c| {
                let haystack = format!(
                    "{} {} {} {}",
                    c.name,
                    c.company.as_deref().unwrap_or(""),
                    c.email.as_deref().unwrap_or(""),
                    c.phone.as_deref().unwrap_or("")
                );
                matcher
                    .fuzzy_match(&haystack, &self.search_input)
                    .map(|score| (score, c))
            })
            .collect();
        matches.sort_by(|a, b| b.0.cmp(&a.0)); // descending score 

        self.filtered_contacts = matches.into_iter().map(|(_, c)| c.clone()).collect();
        self.selected_index = 0;
    }
}
