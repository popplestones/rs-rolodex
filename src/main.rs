mod app;
mod contact;
mod message;
mod modal;
mod tui;

use crate::update::update;
use app::state::RunningState;
use app::update::handle_event;
use app::{init_model, update};

fn main() -> color_eyre::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut model = init_model("contacts.json")?;

    while model.running_state != RunningState::Done {
        terminal.draw(|f| app::ui::view(&mut model, f))?;

        let mut current_msg = handle_event(&model)?;

        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    tui::restore_terminal()?;

    if let Some(selected) = model.filtered_contacts.get(model.selected_index as usize) {
        if let Ok(json) = serde_json::to_string(selected) {
            println!("{json}");
        }
    }
    Ok(())
}
