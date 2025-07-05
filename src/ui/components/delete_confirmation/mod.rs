pub mod message;
use crossterm::event::KeyEvent;
use message::DeleteMessage;
use ratatui::{prelude::*, widgets::*};

use crate::model::Contact;
use crate::ui::components::Component;
use crate::ui::components::app::message::AppMessage;
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

impl Component<DeleteMessage, AppMessage> for DeleteConfirmation {
    fn draw(&self, f: &mut Frame, _rect: Rect, _is_focused: bool) {
        let area = centered_rect(60, 20, f.area());

        // let contact = app.selected_contact.as_ref();
        // let name = contact.map(|c| c.name.as_str()).unwrap_or("this contact");
        let name = "this contact";

        let text = vec![
            Line::from(Span::styled(
                "Delete Contact",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(format!("Are you sure you want to delete '{name}'? [y/n]")),
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Confirm")
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Black).fg(Color::White));

        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(block);

        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);
    }

    fn handle_key(&self, _event: KeyEvent) -> Option<DeleteMessage> {
        // match event.code {
        //     KeyCode::Char('y') => {
        //         if let Some(contact) = app.selected_contact.take() {
        //             app.db.delete_contact(contact.id)?;
        //             app.all_contacts.retain(|c| c.id != contact.id);
        //             app.browse.update_filter(&app.all_contacts);
        //         }
        //         app.mode = AppMode::Browse;
        //     }
        //     KeyCode::Char('n') => {
        //         app.mode = AppMode::Browse;
        //     }
        //     KeyCode::Esc => {
        //         app.mode = AppMode::Browse;
        //     }
        //     _ => {}
        // }
        None
    }
    fn update(&mut self, _message: DeleteMessage) -> Option<AppMessage> {
        None
    }
}
