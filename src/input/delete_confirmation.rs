use crate::{error::AppResult as Result, mode::AppMode};
use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

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
