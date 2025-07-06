pub mod message;
use crossterm::event::{KeyCode, KeyEvent};
use message::DeleteMessage;
use ratatui::{prelude::*, widgets::*};

use crate::model::Contact;
use crate::ui::components::Component;
use crate::ui::components::app::message::AppMsg;
use crate::ui::layout::centered_rect;
#[derive(Debug, Default)]

pub struct DeleteConfirmation {
    contact: Option<Contact>,
}

impl DeleteConfirmation {
    pub fn new() -> Self {
        Self { contact: None }
    }

    pub fn set_contact(&mut self, contact: Contact) {
        self.contact = Some(contact);
    }

    pub fn clear_contact(&mut self) {
        self.contact = None;
    }

    pub fn get_contact_id(&self) -> Option<i64> {
        self.contact.as_ref().map(|c| c.id)
    }
}

impl Component<DeleteMessage, AppMsg> for DeleteConfirmation {
    fn draw(&self, f: &mut Frame, rect: Rect, _is_focused: bool) {
        let outer = centered_rect(60, 15, rect);
        f.render_widget(Clear, outer);

        let Some(contact) = self.contact.as_ref() else {
            return;
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Confirm Delete")
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Black).fg(Color::White));

        f.render_widget(block, outer);

        let inner = Rect {
            x: outer.x + 4,
            y: outer.y + 2,
            width: outer.width.saturating_sub(8),
            height: outer.height.saturating_sub(4),
        };

        let lines = vec![
            Line::from(""), // top margin
            Line::from(Span::styled(
                "Delete Contact",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("Are you sure you want to delete this contact?"),
            Line::from(""),
            Line::from(format!("Name:    {}", contact.name)),
            Line::from(format!(
                "Company: {}",
                contact.company.as_deref().unwrap_or("-")
            )),
            Line::from(format!(
                "Phone:   {}",
                contact.phone.as_deref().unwrap_or("-")
            )),
            Line::from(format!(
                "Email:   {}",
                contact.email.as_deref().unwrap_or("-")
            )),
            Line::from(""),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled("Y", Style::default().add_modifier(Modifier::UNDERLINED)),
                Span::raw("es or "),
                Span::styled("N", Style::default().add_modifier(Modifier::UNDERLINED)),
                Span::raw("o to cancel."),
            ]),
        ];

        let paragraph = Paragraph::new(lines)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, inner);
    }
    fn handle_key(&self, event: KeyEvent) -> Option<DeleteMessage> {
        match event.code {
            KeyCode::Char('y') => Some(DeleteMessage::Confirm),
            KeyCode::Char('n') => Some(DeleteMessage::Cancel),
            KeyCode::Esc => Some(DeleteMessage::Cancel),
            _ => None,
        }
    }
    fn update(&mut self, message: DeleteMessage) -> Option<AppMsg> {
        match message {
            DeleteMessage::Confirm => Some(AppMsg::ConfirmDelete),
            DeleteMessage::Cancel => Some(AppMsg::CancelDelete),
        }
    }
}
