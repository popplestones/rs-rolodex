use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::App, error::AppError, mode::AppMode};

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
        KeyCode::Home => {
            app.browse.selected_index = 0;
        }
        KeyCode::End => {
            app.browse.selected_index = app.browse.filtered_contacts.len() - 1;
        }
        KeyCode::Enter => {
            app.select_contact();
            app.should_quit = true;
        }
        KeyCode::Char('q') if event.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
            app.browse.search_input.clear();
            app.browse.update_filter(&app.all_contacts);
        }
        KeyCode::Char('d') if event.modifiers.contains(KeyModifiers::CONTROL) => {
            app.select_contact();
            app.mode = AppMode::DeleteConfirmation;
        }
        KeyCode::Char('a') if event.modifiers.contains(KeyModifiers::CONTROL) => {
            app.mode = AppMode::AddContact;
        }
        KeyCode::Esc => {
            if app.browse.search_input.is_empty() {
                app.should_quit = true;
            }
            app.browse.search_input.clear();
            app.browse.update_filter(&app.all_contacts);
        }
        KeyCode::Char(c) => {
            app.browse.search_input.push(c);
            app.browse.update_filter(&app.all_contacts);
        }
        KeyCode::Backspace => {
            app.browse.search_input.pop();
            app.browse.update_filter(&app.all_contacts);
        }
        _ => {}
    }
    Ok(())
}
