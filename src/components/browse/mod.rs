use crossterm::event::KeyEvent;
use ratatui::{Frame, prelude::Rect};

use crate::{
    components::{Component, contact_list::ContactList, search::Search},
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
    pub fn handle_key(&self, _event: KeyEvent) -> Option<BrowseMsg> {
        None
    }
    pub fn draw(&self, _f: &mut Frame, _area: Rect, _focused: bool) {}
    pub fn update<ParentMsg>(
        &mut self,
        _: BrowseMsg,
        _: impl Fn(BrowseOutput) -> ParentMsg,
    ) -> Option<ParentMsg> {
        None
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
