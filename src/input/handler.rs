use crossterm::event::KeyEvent;

use crate::{app::App, error::AppError, input::browse, mode::AppMode};

pub fn handle_input(app: &mut App, event: KeyEvent) -> Result<(), AppError> {
    match app.mode {
        AppMode::Browse => browse::handle_input(app, event),
        _ => Ok(()),
    }
}
