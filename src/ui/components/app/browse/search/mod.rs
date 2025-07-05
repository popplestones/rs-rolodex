pub mod message;
use crossterm::cursor::SetCursorStyle;
use crossterm::event::KeyCode;
use crossterm::execute;
use message::SearchMessage;
use ratatui::{prelude::*, widgets::*};

use crate::ui::components::{Component, app::browse::message::BrowseMessage};

#[derive(Debug, Default)]
pub struct Search {
    pub search_input: String,
    pub search_input_cursor: usize,
}

impl Search {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Component<SearchMessage, BrowseMessage> for Search {
    fn update(&mut self, message: SearchMessage) -> Option<BrowseMessage> {
        match message {
            SearchMessage::Left => {
                if self.search_input_cursor > 0 {
                    self.search_input_cursor -= 1;
                }
            }
            SearchMessage::Right => {
                if self.search_input_cursor < self.search_input.len() {
                    self.search_input_cursor += 1;
                }
            }
            SearchMessage::Backspace => {
                if self.search_input_cursor > 0 {
                    self.search_input.remove(self.search_input_cursor - 1);
                    self.search_input_cursor -= 1;
                }
                return Some(BrowseMessage::FilterUpdated);
            }
            SearchMessage::Delete => {
                if self.search_input_cursor < self.search_input.len() {
                    self.search_input.remove(self.search_input_cursor);
                }
                return Some(BrowseMessage::FilterUpdated);
            }
            SearchMessage::Clear => {
                self.search_input.clear();
                self.search_input_cursor = 0;
                return Some(BrowseMessage::FilterUpdated);
            }
            SearchMessage::AddChar(c) => {
                self.search_input.insert(self.search_input_cursor, c);
                self.search_input_cursor += 1;
                return Some(BrowseMessage::FilterUpdated);
            }
        };
        None
    }

    fn draw(&self, f: &mut Frame, rect: Rect, _is_focused: bool) {
        let search = Paragraph::new(self.search_input.as_str())
            .block(Block::default().borders(Borders::ALL).title("Search"));

        f.render_widget(search, rect);
        let _ = execute!(std::io::stderr(), SetCursorStyle::BlinkingBar);
        f.set_cursor_position(Position {
            x: rect.x + 1 + self.search_input_cursor as u16,
            y: rect.y + 1,
        });
    }

    fn handle_key(&self, event: crossterm::event::KeyEvent) -> Option<SearchMessage> {
        match event.code {
            KeyCode::Left => Some(SearchMessage::Left),
            KeyCode::Right => Some(SearchMessage::Right),
            KeyCode::Backspace => Some(SearchMessage::Backspace),
            KeyCode::Delete => Some(SearchMessage::Delete),
            KeyCode::Esc => Some(SearchMessage::Clear),
            KeyCode::Char(c) => Some(SearchMessage::AddChar(c)),
            _ => None,
        }
    }
}
