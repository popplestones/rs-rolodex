use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tracing::info;

#[derive(Debug, Default)]
pub enum InputMode {
    #[default]
    Regular,
    Inline,
}

pub enum InputMsg {
    CursorLeft,
    CursorRight,
    CursorStart,
    CursorEnd,
    Backspace,
    Delete,
    TypeChar(char),
}

pub enum InputOutput {
    Changed(String),
}

#[derive(Debug, Default)]
pub struct Input {
    label: String,
    label_width: u16,
    pub value: String,
    cursor: usize,
    focused: bool,
    mode: InputMode,
}

impl Input {
    pub fn new(label: &str, value: &str, label_width: u16, mode: InputMode) -> Self {
        Self {
            label: label.to_string(),
            label_width,
            value: value.to_string(),
            focused: false,
            cursor: value.len(),
            mode,
        }
    }
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }
    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }

    pub fn update<ParentMsg>(
        &mut self,
        msg: InputMsg,
        map: impl Fn(InputOutput) -> ParentMsg,
    ) -> Option<ParentMsg> {
        match msg {
            InputMsg::CursorLeft => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                None
            }
            InputMsg::CursorRight => {
                if self.cursor < self.value.len() {
                    self.cursor += 1;
                }
                None
            }
            InputMsg::CursorStart => {
                self.cursor = 0;
                None
            }
            InputMsg::CursorEnd => {
                self.cursor = self.value.len();
                None
            }
            InputMsg::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.value.remove(self.cursor);
                    Some(map(InputOutput::Changed(self.value.clone())))
                } else {
                    None
                }
            }
            InputMsg::Delete => {
                if self.cursor < self.value.len() {
                    self.value.remove(self.cursor);
                    Some(map(InputOutput::Changed(self.value.clone())))
                } else {
                    None
                }
            }
            InputMsg::TypeChar(c) => {
                self.value.insert(self.cursor, c);
                self.cursor += 1;
                Some(map(InputOutput::Changed(self.value.clone())))
            }
        }
    }

    fn draw_regular(&self, f: &mut Frame, area: Rect, focused: bool) {
        let block = Block::default()
            .title(self.label.clone())
            .borders(Borders::ALL)
            .border_style(if focused {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            });
        f.render_widget(&block, area);

        let text_style = if focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let inner = block.inner(area);

        let padded_value = format!("  {}", self.value.clone());
        let input = Paragraph::new(padded_value).style(text_style);
        f.render_widget(input, inner);

        if focused {
            let clamped_cursor = self.cursor.min(self.value.len());
            let cursor_x = inner.x + clamped_cursor as u16;
            let cursor_y = inner.y;
            info!("Cursor position: {cursor_x}, {cursor_y}");

            f.set_cursor_position(Position {
                x: cursor_x + 2,
                y: cursor_y,
            });
        }
    }
    fn draw_inline(&self, f: &mut Frame, area: Rect, focused: bool) {}
    pub fn draw(&self, f: &mut Frame, area: Rect, focused: bool) {
        match self.mode {
            InputMode::Regular => self.draw_regular(f, area, focused),
            InputMode::Inline => self.draw_inline(f, area, focused),
        }
    }

    pub fn handle_key(&self, event: KeyEvent) -> Option<InputMsg> {
        match event.code {
            KeyCode::Left => Some(InputMsg::CursorLeft),
            KeyCode::Right => Some(InputMsg::CursorRight),
            KeyCode::Home => Some(InputMsg::CursorStart),
            KeyCode::End => Some(InputMsg::CursorEnd),
            KeyCode::Backspace => Some(InputMsg::Backspace),
            KeyCode::Delete => Some(InputMsg::Delete),
            KeyCode::Char(c) => Some(InputMsg::TypeChar(c)),
            _ => None,
        }
    }
}

impl crate::components::Component for Input {
    type Msg = InputMsg;
    type Output = InputOutput;

    fn draw(&self, f: &mut Frame, area: Rect, focused: bool) {
        self.draw(f, area, focused);
    }
    fn handle_key(&self, event: KeyEvent) -> Option<Self::Msg> {
        self.handle_key(event)
    }

    fn update<ParentMsg>(
        &mut self,
        msg: Self::Msg,
        map: impl Fn(Self::Output) -> ParentMsg,
    ) -> Option<ParentMsg> {
        self.update(msg, map)
    }
}
