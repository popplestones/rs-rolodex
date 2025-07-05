pub mod contact_list;
pub mod message;
pub mod search;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use ratatui::prelude::*;

use crate::{
    model::Contact,
    ui::components::{Component, app::browse::message::BrowseMessage},
};
use contact_list::ContactList;
use search::Search;

use crate::ui::components::app::message::AppMessage;

#[derive(Debug, Default)]
pub struct Browse {
    pub all_contacts: Vec<Contact>,
    pub contact_list: ContactList,
    pub search: Search,
}

impl Browse {
    pub fn new(contacts: &[Contact]) -> Self {
        Self {
            all_contacts: contacts.to_vec(),
            contact_list: ContactList::new(contacts),
            search: Search::new(),
        }
    }

    pub fn update_filter(&mut self) {
        let matcher = SkimMatcherV2::default();

        if self.search.search_input.trim().is_empty() {
            self.contact_list.filtered_contacts = self.all_contacts.to_vec();
            self.contact_list.selected_index = 0;
            return;
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
                    .fuzzy_match(&haystack, &self.search.search_input)
                    .map(|score| (score, c))
            })
            .collect();
        matches.sort_by(|a, b| b.0.cmp(&a.0)); // descending score 

        self.contact_list.filtered_contacts = matches.into_iter().map(|(_, c)| c.clone()).collect();
        self.contact_list.selected_index = 0;
    }

    pub fn get_selected_contact(&self) -> Option<Contact> {
        if self.contact_list.selected_index < self.contact_list.filtered_contacts.len() {
            Some(self.contact_list.filtered_contacts[self.contact_list.selected_index].clone())
        } else {
            None
        }
    }
}
impl Component<BrowseMessage, AppMessage> for Browse {
    fn draw(&self, f: &mut Frame, _rect: Rect, _is_focused: bool) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(f.area());

        self.search.draw(f, chunks[0], _is_focused);
        self.contact_list.draw(f, chunks[1], _is_focused);
    }

    fn update(&mut self, message: BrowseMessage) -> Option<AppMessage> {
        match message {
            BrowseMessage::Search(msg) => self.search.update(msg).map(AppMessage::Browse),
            BrowseMessage::List(msg) => self.contact_list.update(msg),
            BrowseMessage::Select => self
                .get_selected_contact()
                .clone()
                .map(AppMessage::SelectContact),
            BrowseMessage::FilterUpdated => {
                self.update_filter();
                None
            }
            BrowseMessage::Delete => {
                if let Some(contact) = self.get_selected_contact() {
                    Some(AppMessage::Delete(contact))
                } else {
                    None
                }
            }
        }
    }

    fn handle_key(&self, event: KeyEvent) -> Option<BrowseMessage> {
        // Handle Keys for the Contact List
        match event.code {
            KeyCode::Enter => return Some(BrowseMessage::Select),
            KeyCode::Up
            | KeyCode::Down
            | KeyCode::Home
            | KeyCode::End
            | KeyCode::PageUp
            | KeyCode::PageDown => {
                return self.contact_list.handle_key(event).map(BrowseMessage::List);
            }
            KeyCode::Char('d') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                return Some(BrowseMessage::Delete);
            }
            _ => None::<BrowseMessage>,
        };

        // Handle Keys for the Search
        self.search.handle_key(event).map(BrowseMessage::Search)
    }
}
