use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

use crate::ui::components::app::App;
use crate::ui::layout::centered_rect;
use crate::{error::AppResult as Result, mode::AppMode};

pub struct DeleteConfirmation;

impl DeleteConfirmation {
    pub fn draw(f: &mut Frame, app: &App) {
        let area = centered_rect(60, 20, f.area());

        let contact = app.selected_contact.as_ref();
        let name = contact.map(|c| c.name.as_str()).unwrap_or("this contact");

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

    pub fn handle_input(app: &mut App, event: KeyEvent) -> Result<()> {
        match event.code {
            KeyCode::Char('y') => {
                if let Some(contact) = app.selected_contact.take() {
                    app.db.delete_contact(contact.id)?;
                    app.all_contacts.retain(|c| c.id != contact.id);
                    app.browse.update_filter(&app.all_contacts);
                }
                app.mode = AppMode::Browse;
            }
            KeyCode::Char('n') => {
                app.mode = AppMode::Browse;
            }
            KeyCode::Esc => {
                app.mode = AppMode::Browse;
            }
            _ => {}
        }
        Ok(())
    }
}
