pub mod message;
use crate::ui::components::{Component, app::message::AppMsg};
use crossterm::event::{KeyCode, KeyEvent};
use message::TextFieldMsg;
use ratatui::{prelude::*, widgets::*};
use tracing::info;

#[derive(Debug, Default)]
pub struct TextField {
    pub label: String,
    pub value: String,
    pub cursor: usize,
}

impl TextField {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Default::default()
        }
    }
    pub fn end(&mut self) {
        self.cursor = self.value.len();
    }
}
impl Component<TextFieldMsg, AppMsg> for TextField {
    fn update(&mut self, message: TextFieldMsg) -> Option<AppMsg> {
        info!("update: {:?}", message);
        match message {
            TextFieldMsg::Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            TextFieldMsg::Right => {
                if self.cursor < self.value.len() {
                    self.cursor += 1;
                }
            }
            TextFieldMsg::Home => {
                self.cursor = 0;
            }
            TextFieldMsg::End => {
                self.end();
            }
            TextFieldMsg::AddChar(c) => {
                self.value.insert(self.cursor, c);
                self.cursor += 1;
            }
            TextFieldMsg::Backspace => {
                if self.cursor > 0 {
                    self.value.remove(self.cursor - 1);
                    self.cursor -= 1;
                }
            }
            TextFieldMsg::Delete => {
                if self.cursor < self.value.len() {
                    self.value.remove(self.cursor);
                }
            }
        };

        None
    }
    fn handle_key(&self, event: KeyEvent) -> Option<TextFieldMsg> {
        info!("key: {:?}", event.code);
        match event.code {
            KeyCode::Left => {
                info!("text_field::handle_key: left");
                Some(TextFieldMsg::Left)
            }
            KeyCode::Right => Some(TextFieldMsg::Right),
            KeyCode::Home => Some(TextFieldMsg::Home),
            KeyCode::End => Some(TextFieldMsg::End),
            KeyCode::Char(c) => Some(TextFieldMsg::AddChar(c)),
            KeyCode::Backspace => Some(TextFieldMsg::Backspace),
            KeyCode::Delete => Some(TextFieldMsg::Delete),
            _ => None,
        }
    }
    fn draw(&self, f: &mut Frame, area: Rect, is_focused: bool) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.label.clone())
            .border_style(if is_focused {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            });
        let para = Paragraph::new(Line::from(self.value.clone())).block(block);

        f.render_widget(para, area);

        if is_focused {
            let x = area.x + 1 + self.cursor as u16;
            let y = area.y + 1;
            f.set_cursor_position(Position::new(x, y));
        }
    }
}
