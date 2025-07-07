use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

pub trait Component {
    type Msg;
    type Output;

    /// Handle a component-level message and optionally emit output to parent.
    fn update<ParentMsg>(
        &mut self,
        msg: Self::Msg,
        map: impl Fn(Self::Output) -> ParentMsg,
    ) -> Option<ParentMsg>;

    /// Map a key event to an internal message.
    fn handle_key(&self, key: KeyEvent) -> Option<Self::Msg>;

    /// Draw the component to the given area.
    fn view(&self, f: &mut Frame, area: Rect);
}

pub fn opt(val: String) -> Option<String> {
    if val.is_empty() { None } else { Some(val) }
}
