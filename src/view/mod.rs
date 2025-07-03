use ratatui::prelude::*;

use crate::{app::App, mode::AppMode};

pub mod add_contact;
pub mod browse;
pub mod delete_confirmation;
pub mod error;

pub fn draw(f: &mut Frame, app: &App) {
    // Step 1: Draw main browse ui

    browse::draw(f, app);

    // Step 2: overlay mode-specific view (like modals)
    match app.mode {
        AppMode::Error => error::draw(f, app),
        AppMode::DeleteConfirmation => delete_confirmation::draw(f, app),
        AppMode::AddContact => add_contact::draw(f, app),
        _ => {}
    }
}
