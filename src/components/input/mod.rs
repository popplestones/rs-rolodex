use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{prelude::*, widgets::*};
use tracing::info;

#[derive(Debug, Default)]
pub enum InputMode {
    #[default]
    Regular,
    Inline,
}

#[derive(Debug, Clone)]
pub enum InputMsg {
    Clear,
    CursorLeft,
    CursorRight,
    CursorStart,
    CursorEnd,
    Backspace,
    Delete,
    TypeChar(char),
}

#[derive(Debug, Clone)]
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
    max_len: usize,
}

impl Input {
    pub fn new(
        label: &str,
        value: &str,
        label_width: u16,
        mode: InputMode,
        max_len: usize,
    ) -> Self {
        Self {
            label: label.to_string(),
            label_width,
            value: value.to_string(),
            focused: false,
            cursor: value.len(),
            mode,
            max_len,
        }
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
        self.cursor = self.value.len();
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
            InputMsg::Clear => {
                self.value.clear();
                self.cursor = 0;
                Some(map(InputOutput::Changed(self.value.clone())))
            }
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
                if self.value.len() < self.max_len {
                    self.value.insert(self.cursor, c);
                    self.cursor += 1;
                    Some(map(InputOutput::Changed(self.value.clone())))
                } else {
                    None
                }
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

        let input = Paragraph::new(self.value.clone()).style(text_style);
        f.render_widget(input, inner);

        self.set_cursor_position(f, inner, focused);
    }
    fn set_cursor_position(&self, f: &mut Frame, area: Rect, focused: bool) {
        if focused {
            let clamped_cursor = self.cursor.min(self.value.len());
            let cursor_x = area.x + clamped_cursor as u16;
            let cursor_y = area.y;

            f.set_cursor_position(Position {
                x: cursor_x,
                y: cursor_y,
            });
        }
    }
    fn draw_inline(&self, f: &mut Frame, area: Rect, focused: bool) {
        let text_style = if focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let label_text = format!(
            "{:<width$}: ",
            self.label.clone(),
            width = self.label_width as usize
        );

        let label = Paragraph::new(label_text).style(Style::default().fg(Color::Cyan));
        let input = Paragraph::new(self.value.clone()).style(text_style);
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(self.label_width + 2), Constraint::Min(0)])
            .split(area);

        f.render_widget(label, layout[0]);
        f.render_widget(input, layout[1]);

        self.set_cursor_position(f, layout[1], focused);
    }
    pub fn draw(&self, f: &mut Frame, area: Rect, focused: bool) {
        match self.mode {
            InputMode::Regular => self.draw_regular(f, area, focused),
            InputMode::Inline => self.draw_inline(f, area, focused),
        }
    }

    pub fn handle_key(&self, event: KeyEvent) -> Option<InputMsg> {
        match event.code {
            KeyCode::Char('l') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                info!("Ctrl+L pressed - Clearing input");
                Some(InputMsg::Clear)
            }
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
