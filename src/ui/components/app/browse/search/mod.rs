pub mod message;
use message::SearchMessage;
use ratatui::{prelude::*, widgets::*};

use crate::ui::components::Component;

#[derive(Debug, Default)]
pub struct Search {
    pub search_input: String,
}

impl Search {
    pub fn new() -> Self {
        Self::default()
    }
}
use crate::ui::components::app::message::AppMessage;
impl Component<SearchMessage, AppMessage> for Search {
    fn update(&mut self, _message: SearchMessage) -> Option<AppMessage> {
        None
    }

    fn draw(&self, f: &mut Frame, rect: Rect, _is_focused: bool) {
        let search = Paragraph::new(self.search_input.as_str())
            .block(Block::default().borders(Borders::ALL).title("Search"));

        f.render_widget(search, rect);
    }

    fn handle_key(&self, _event: crossterm::event::KeyEvent) -> Option<SearchMessage> {
        todo!()
    }
}
