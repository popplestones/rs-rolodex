use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::*};

use crate::{components::Component, model::Contact};

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
    pub fn update<ParentMsg>(
        &mut self,
        _msg: ContactListMsg,
        _map: impl Fn(ContactListOutput) -> ParentMsg,
    ) -> Option<ParentMsg> {
        None
    }
    fn draw(&self, _f: &mut Frame, _area: Rect, _focused: bool) {}
    fn handle_key(&self, _key: KeyEvent) -> Option<ContactListMsg> {
        None
    }
}

impl Component for ContactList {
    type Msg = ContactListMsg;
    type Output = ContactListOutput;

    fn update<ParentMsg>(
        &mut self,
        msg: Self::Msg,
        map: impl Fn(Self::Output) -> ParentMsg,
    ) -> Option<ParentMsg> {
        self.update(msg, map)
    }

    fn draw(&self, f: &mut Frame, area: Rect, focused: bool) {
        self.draw(f, area, focused)
    }

    fn handle_key(&self, key: KeyEvent) -> Option<Self::Msg> {
        self.handle_key(key)
    }
}
