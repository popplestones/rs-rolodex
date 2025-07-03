use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::App, error::AppError};

pub fn handle_input(app: &mut App, event: KeyEvent) -> Result<(), AppError> {
    if event.code == KeyCode::Char('q') && event.modifiers.contains(KeyModifiers::CONTROL) {
        app.should_quit = true;
    }
    Ok(())
}
