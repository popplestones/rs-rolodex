use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

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

pub struct Input {
    label: String,
    pub value: String,
    cursor: usize,
    focused: bool,
}

impl Input {
    pub fn new(label: &str, value: &str) -> Self {
        Self {
            label: label.to_string(),
            value: value.to_string(),
            focused: false,
            cursor: value.len(),
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

    pub fn view(&self, f: &mut Frame, area: Rect) {
        f.render_widget(
            Paragraph::new(self.label.clone()).alignment(Alignment::Left),
            area,
        );
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

    fn view(&self, f: &mut Frame, area: Rect) {
        self.view(f, area);
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
