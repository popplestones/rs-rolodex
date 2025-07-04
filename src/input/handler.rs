// use crossterm::event::KeyEvent;
//
// use crate::{
//     error::AppError,
//     input::{add_contact, browse},
//     mode::AppMode,
//     ui::components::{app::App, delete_confirmation::DeleteConfirmation},
// };
//
// pub fn handle_input(app: &mut App, event: KeyEvent) -> Result<(), AppError> {
//     match app.mode {
//         AppMode::Browse => browse::handle_input(app, event),
//         AppMode::DeleteConfirmation => DeleteConfirmation::handle_input(app, event),
//         AppMode::AddContact => add_contact::handle_input(app, event),
//         _ => Ok(()),
//     }
// }
