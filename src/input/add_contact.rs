use crossterm::event::{KeyCode, KeyEvent};
use tracing::info;

use crate::{error::AppResult as Result, mode::AppMode, ui::components::app::App};

pub fn handle_input(app: &mut App, event: KeyEvent) -> Result<()> {
    match event.code {
        KeyCode::Char('c') => {
            app.mode = AppMode::Browse;
        }
        KeyCode::Char(c) => {
            info!("Add contact: {}", c);
        }
        _ => {}
    }
    Ok(())
}
