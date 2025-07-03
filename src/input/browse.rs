use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::App, error::AppError};

pub fn handle_input(app: &mut App, event: KeyEvent) -> Result<(), AppError> {
    match event.code {
        KeyCode::Up => {
            if app.browse.selected_index > 0 {
                app.browse.selected_index -= 1;
            }
        }
        KeyCode::Down => {
            if app.browse.selected_index < app.browse.filtered_contacts.len() - 1 {
                app.browse.selected_index += 1;
            }
        }
        KeyCode::Char('q') if event.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        _ => {}
    }
    Ok(())
}
