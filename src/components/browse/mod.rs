use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

use crate::{
    components::{
        Component,
        contact_list::{ContactList, ContactListMsg, ContactListOutput},
        input::{Input, InputMode, InputMsg},
    },
    model::Contact,
};

pub enum BrowseMsg {
    List(ContactListMsg),
    Input(InputMsg),
}

pub enum BrowseOutput {
    ContactActivated(Contact),
}

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
    pub fn handle_key(&self, event: KeyEvent) -> Option<BrowseMsg> {
        match event.code {
            KeyCode::Home
            | KeyCode::End
            | KeyCode::PageUp
            | KeyCode::PageDown
            | KeyCode::Up
            | KeyCode::Down => self.contact_list.handle_key(event).map(BrowseMsg::List),
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
                    })
            }
            BrowseMsg::Input(msg) => {
                self.search.update(msg, |_| {});
                None
            }
        }
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
