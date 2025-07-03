use crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, error::AppResult as Result, mode::AppMode};

pub fn handle_input(app: &mut App, event: KeyEvent) -> Result<()> {
    match event.code {
        KeyCode::Char('c') => {
            app.mode = AppMode::Browse;
        }
        _ => {}
    }
    Ok(())
}
