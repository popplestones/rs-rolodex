use crate::components::Component;
use crate::mode::AppMode;
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

#[derive(Debug, Clone)]
pub enum StatusBarMsg {
    UpdateCounts { total: usize, filtered: usize },
    UpdateMode(AppMode),
}

#[derive(Debug, Clone)]
pub struct StatusBar {
    total_contacts: usize,
    filtered_contacts: usize,
    current_mode: AppMode,
}

impl Default for StatusBar {
    fn default() -> Self {
        Self {
            total_contacts: 0,
            filtered_contacts: 0,
            current_mode: AppMode::Browse,
        }
    }
}

impl StatusBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, msg: StatusBarMsg) {
        match msg {
            StatusBarMsg::UpdateCounts { total, filtered } => {
                self.total_contacts = total;
                self.filtered_contacts = filtered;
            }
            StatusBarMsg::UpdateMode(mode) => {
                self.current_mode = mode;
            }
        }
    }

    fn get_help_text(&self) -> &'static str {
        match self.current_mode {
            AppMode::Browse => "↑↓: Navigate | Enter: Select | Ctrl+A: Add | Ctrl+E: Edit | Ctrl+D: Delete | Ctrl+Q: Quit",
            AppMode::ContactForm => "Tab: Next Field | Enter: Save | Esc: Cancel",
            AppMode::Delete => "Y: Confirm | N/Esc: Cancel",
            AppMode::Error(_) => "Esc: Dismiss",
        }
    }

    fn get_counts_text(&self) -> String {
        format!("Contacts: {}/{}", self.filtered_contacts, self.total_contacts)
    }
}

impl Component for StatusBar {
    type Msg = StatusBarMsg;
    type Output = StatusBarMsg;

    fn update<ParentMsg>(
        &mut self,
        msg: Self::Msg,
        _map: impl Fn(Self::Output) -> ParentMsg,
    ) -> Option<ParentMsg> {
        self.update(msg);
        None
    }

    fn draw(&self, frame: &mut Frame, area: Rect, _focused: bool) {
        let help_text = self.get_help_text();
        let counts_text = self.get_counts_text();

        // Create layout with help text on left and counts on right
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(1), Constraint::Length(counts_text.len() as u16 + 2)])
            .split(area);

        // Help text paragraph (left side)
        let help_paragraph = Paragraph::new(Line::from(vec![
            Span::styled(help_text, Style::default().fg(Color::Gray))
        ]))
        .style(Style::default().bg(Color::DarkGray))
        .alignment(Alignment::Left);

        // Counts paragraph (right side)
        let counts_paragraph = Paragraph::new(Line::from(vec![
            Span::styled(counts_text, Style::default().fg(Color::White))
        ]))
        .style(Style::default().bg(Color::DarkGray))
        .alignment(Alignment::Right);

        frame.render_widget(help_paragraph, chunks[0]);
        frame.render_widget(counts_paragraph, chunks[1]);
    }

    fn handle_key(&self, _event: KeyEvent) -> Option<Self::Msg> {
        // Status bar doesn't handle key events
        None
    }
}