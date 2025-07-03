use ratatui::prelude::*;

use crate::{app::App, mode::AppMode};

pub mod browse;
pub mod error;

pub fn draw(f: &mut Frame, app: &App) {
    // Step 1: Draw main browse ui

    browse::draw(f, app);

    // Step 2: overlay mode-specific view (like modals)
    match app.mode {
        AppMode::Error => error::draw(f, app),
        _ => {}
    }
}
