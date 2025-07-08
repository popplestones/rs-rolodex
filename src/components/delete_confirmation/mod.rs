use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

use crate::{components::Component, model::Contact};

pub enum DeleteMsg {}
pub enum DeleteOutput {}

#[derive(Debug, Default)]
pub struct DeleteConfirmation {
    pub contact: Contact,
}

impl DeleteConfirmation {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn handle_key(&self, _event: KeyEvent) -> Option<DeleteMsg> {
        None
    }
    pub fn draw(&self, _f: &mut Frame, _area: Rect, _focused: bool) {}
    pub fn update<ParentMsg>(
        &mut self,
        _: DeleteMsg,
        _: impl Fn(DeleteOutput) -> ParentMsg,
    ) -> Option<ParentMsg> {
        None
    }
}

impl Component for DeleteConfirmation {
    type Msg = DeleteMsg;
    type Output = DeleteOutput;

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
    fn handle_key(&self, event: KeyEvent) -> Option<Self::Msg> {
        self.handle_key(event)
    }
}
