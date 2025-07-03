use crossterm::event::KeyCode;
use ratatui::{prelude::*, widgets::*};

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
    pub fn handle_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char(c) => {
                self.value.insert(self.cursor, c);
                self.cursor += 1;
            }
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.value.remove(self.cursor);
                }
            }
            KeyCode::Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor < self.value.len() {
                    self.cursor += 1;
                }
            }
            KeyCode::End => {
                self.cursor = self.value.len();
            }
            KeyCode::Home => {
                self.cursor = 0;
            }
            KeyCode::Delete => {
                if self.cursor < self.value.len() {
                    self.value.remove(self.cursor);
                }
            }
            _ => {}
        }
    }

    pub fn line(&self) -> Line {
        Line::from(self.value.clone())
    }
    pub fn draw(&self, f: &mut Frame, area: Rect, is_focused: bool) {
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
