mod app;
mod contact;
mod tui;
mod ui;

use app::{RunningState, handle_event, init_model, update};
use ui::view;

fn main() -> color_eyre::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut model = init_model("contacts.json")?;

    while model.running_state != RunningState::Done {
        terminal.draw(|f| view(&mut model, f))?;

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
