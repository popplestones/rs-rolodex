use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::*};

use crate::{
    components::{
        Component,
        contact_list::ContactList,
        input::{Input, InputMode},
    },
    model::Contact,
};

pub enum BrowseMsg {}
pub enum BrowseOutput {}

pub struct Browse {
    pub search: Input,
    pub contact_list: ContactList,
}

impl Browse {
    pub fn new(contacts: &[Contact]) -> Self {
        Self {
            search: Input::new("Search", "foo", 10, InputMode::Regular),
            contact_list: ContactList::new(contacts),
        }
    }
    pub fn handle_key(&self, _event: KeyEvent) -> Option<BrowseMsg> {
        None
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
