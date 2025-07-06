pub mod contact_list;
pub mod message;
pub mod search;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use ratatui::prelude::*;

use crate::{
    model::Contact,
    ui::components::{Component, app::browse::message::BrowseMsg},
};
use contact_list::ContactList;
use search::Search;

use crate::ui::components::app::message::AppMsg;

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

    pub fn delete_contact(&mut self, id: i64) {
        self.all_contacts.retain(|c| c.id != id);
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
}
impl Component<BrowseMsg, AppMsg> for Browse {
    fn draw(&self, f: &mut Frame, _rect: Rect, _is_focused: bool) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(f.area());

        self.search.draw(f, chunks[0], _is_focused);
        self.contact_list.draw(f, chunks[1], _is_focused);
    }

    fn update(&mut self, message: BrowseMsg) -> Option<AppMsg> {
        match message {
            BrowseMsg::Search(msg) => self.search.update(msg).map(AppMsg::Browse),
            BrowseMsg::List(msg) => self.contact_list.update(msg),
            BrowseMsg::Activate => self
                .contact_list
                .get_selected_contact()
                .map(AppMsg::ActivateContact),
            BrowseMsg::Select => self
                .contact_list
                .get_selected_contact()
                .map(AppMsg::SelectContact),
            BrowseMsg::FilterUpdated => {
                self.update_filter();
                None
            }
            BrowseMsg::Delete => self.contact_list.get_selected_contact().map(AppMsg::Delete),
            BrowseMsg::Edit => self
                .contact_list
                .get_selected_contact()
                .map(|contact| AppMsg::OpenContactForm(Some(contact))),
            BrowseMsg::Add => Some(AppMsg::OpenContactForm(None)),
            BrowseMsg::App(msg) => Some(*msg),
        }
    }

    fn handle_key(&self, event: KeyEvent) -> Option<BrowseMsg> {
        // Handle Keys for the Contact List
        match event.code {
            KeyCode::Enter => return Some(BrowseMsg::Activate),
            KeyCode::Up
            | KeyCode::Down
            | KeyCode::Home
            | KeyCode::End
            | KeyCode::PageUp
            | KeyCode::PageDown => {
                return self.contact_list.handle_key(event).map(BrowseMsg::List);
            }
            KeyCode::Char('d') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                return Some(BrowseMsg::Delete);
            }
            KeyCode::Char('a') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                return Some(BrowseMsg::Add);
            }
            KeyCode::Char('e') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                return Some(BrowseMsg::Edit);
            }
            _ => None::<BrowseMsg>,
        };

        // Handle Keys for the Search
        self.search.handle_key(event).map(BrowseMsg::Search)
    }
}
