use ratatui::{Frame, layout::Rect};

pub mod add_contact;
pub mod app;
pub mod delete_confirmation;
pub mod text_field;

pub trait Component {
    fn draw(&self, f: &mut Frame, rect: Rect, is_focused: bool);
    fn handle_key(&mut self, code: crossterm::event::KeyCode);
}
