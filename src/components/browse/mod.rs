use crossterm::event::{KeyCode, KeyEvent};
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use ratatui::prelude::*;
use tracing::info;

use crate::{
    components::{
        Component,
        contact_list::{ContactList, ContactListMsg, ContactListOutput},
        input::{Input, InputMode, InputMsg, InputOutput},
    },
    model::Contact,
};

pub enum BrowseMsg {
    List(ContactListMsg),
    Input(InputMsg),
}

pub enum BrowseOutput {
    ContactSelected(Contact),
    ContactActivated(Contact),
}

pub struct Browse {
    pub search: Input,
    pub contact_list: ContactList,
    pub all_contacts: Vec<Contact>,
}

impl Browse {
    pub fn new(contacts: &[Contact]) -> Self {
        Self {
            search: Input::new("Search", "", 10, InputMode::Regular, 40),
            contact_list: ContactList::new(contacts),
            all_contacts: contacts.to_vec(),
        }
    }
    pub fn set_contacts(&mut self, contacts: &[Contact]) {
        let query = self.search.value.clone();
        self.all_contacts = contacts.to_vec();
        self.filter_contacts(&query);
    }
    pub fn handle_key(&self, event: KeyEvent) -> Option<BrowseMsg> {
        match event.code {
            KeyCode::Home
            | KeyCode::End
            | KeyCode::PageUp
            | KeyCode::PageDown
            | KeyCode::Up
            | KeyCode::Down
            | KeyCode::Enter => self.contact_list.handle_key(event).map(BrowseMsg::List),
            _ => self.search.handle_key(event).map(BrowseMsg::Input),
        }
    }
    pub fn draw(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        self.search.draw(f, chunks[0], true);
        self.contact_list.draw(f, chunks[1], false);
    }
    pub fn update<ParentMsg>(
        &mut self,
        msg: BrowseMsg,
        map: impl Fn(BrowseOutput) -> ParentMsg,
    ) -> Option<ParentMsg> {
        match msg {
            BrowseMsg::List(list_msg) => {
                self.contact_list
                    .update(list_msg, |list_output| match list_output {
                        ContactListOutput::ContactActivated(contact) => {
                            map(BrowseOutput::ContactActivated(contact))
                        }
                        ContactListOutput::ContactSelected(contact) => {
                            map(BrowseOutput::ContactSelected(contact))
                        }
                    })
            }
            BrowseMsg::Input(input_msg) => {
                let result = self.search.update(input_msg, |output| output);
                if let Some(InputOutput::Changed(value)) = result {
                    self.filter_contacts(&value);
                }
                None
            }
        }
    }
    fn filter_contacts(&mut self, query: &str) {
        let matcher = SkimMatcherV2::default();

        if query.trim().is_empty() {
            self.contact_list.filtered_contacts = self.all_contacts.clone();
        }

        let mut matches: Vec<(i64, &Contact)> = self
            .all_contacts
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
                    .fuzzy_match(&haystack, query)
                    .map(|score| (score, c))
            })
            .collect();
        matches.sort_by(|a, b| b.0.cmp(&a.0)); // descending score

        self.contact_list.filtered_contacts = matches.into_iter().map(|(_, c)| c.clone()).collect();
    }
}

impl Component for Browse {
    type Msg = BrowseMsg;
    type Output = BrowseOutput;

    fn update<ParentMsg>(
        &mut self,
        msg: Self::Msg,
        map: impl Fn(Self::Output) -> ParentMsg,
    ) -> Option<ParentMsg> {
        self.update(msg, map)
    }

    fn handle_key(&self, key: KeyEvent) -> Option<Self::Msg> {
        self.handle_key(key)
    }

    fn draw(&self, f: &mut Frame, area: Rect, focused: bool) {
        self.draw(f, area, focused);
    }
}
