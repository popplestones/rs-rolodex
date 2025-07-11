use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

use crate::components::Component;

pub enum ErrorMsg {
    Dismiss,
}
pub enum ErrorOutput {
    Dismissed,
}
#[derive(Debug, Default)]
pub struct ErrorDialog {
    message: String,
}
impl ErrorDialog {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_error(&mut self, message: &str) {
        self.message = message.to_string();
    }
    pub fn handle_key(&self, event: KeyEvent) -> Option<ErrorMsg> {
        match event.code {
            KeyCode::Esc => Some(ErrorMsg::Dismiss),
            _ => None,
        }
    }
    pub fn draw(&self, f: &mut Frame, area: Rect, _focused: bool) {
        f.render_widget(Clear, area);

        // Main error message
        let text = format!("\n{}\n\n[Press Esc to dismiss]", self.message);
        let paragraph = Paragraph::new(text)
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Error ")
                    .style(Style::default().fg(Color::White).bg(Color::Black)),
            );
        f.render_widget(paragraph, area);
    }
    pub fn update<ParentMsg>(
        &mut self,
        msg: ErrorMsg,
        map: impl Fn(ErrorOutput) -> ParentMsg,
    ) -> Option<ParentMsg> {
        match msg {
            ErrorMsg::Dismiss => Some(map(ErrorOutput::Dismissed)),
        }
    }
}

impl Component for ErrorDialog {
    type Msg = ErrorMsg;
    type Output = ErrorOutput;

    fn update<ParentMsg>(
        &mut self,
        msg: Self::Msg,
        map: impl Fn(Self::Output) -> ParentMsg,
    ) -> Option<ParentMsg> {
        self.update(msg, map)
    }

    fn handle_key(&self, key: KeyEvent) -> Option<Self::Msg> {
        self.handle_key(key)
    }

    fn draw(&self, f: &mut Frame, area: Rect, focused: bool) {
        self.draw(f, area, focused)
    }
}
