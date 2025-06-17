use std::time::Duration;

use crossterm::event::{self, Event, KeyModifiers};

use super::message::AppMessage;
use super::state::{Model, RunningState};
use crate::message::Message;

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::App(app_msg) => update_app(model, app_msg),
        Message::Modal(modal_msg) => update_modal(model, modal_msg),
    }
}

pub fn update_app(model: &mut Model, msg: AppMessage) -> Option<Message> {
    use AppMessage::*;
    match msg {
        MoveUp => model.selected_index = model.selected_index.saturating_sub(1),
        MoveDown => model.selected_index = model.selected_index.saturating_add(1),
        MoveStart => model.selected_index = 0,
        MoveEnd => model.selected_index = (model.filtered_contacts.len() as u32).saturating_sub(1),
        Quit => model.running_state = RunningState::Done,
        UpdateQueryAppend(c) => {
            model.search_query.push(c);
            apply_filter(model);
        }
        UpdateQueryBackspace => {
            model.search_query.pop();
            apply_filter(model);
        }
        ClearQuery => {
            if model.search_query.is_empty() {
                model.running_state = RunningState::Done;
            } else {
                model.search_query.clear();
                apply_filter(model);
            }
        }
        Select => model.running_state = RunningState::Done,
    };
    None
}

fn apply_filter(model: &mut Model) {
    let query = model.search_query.to_lowercase();
    model.filtered_contacts = model
        .all_contacts
        .iter()
        .filter(|c| {
            c.name.to_lowercase().contains(&query) || c.company.to_lowercase().contains(&query)
        })
        .cloned()
        .collect();
}
pub fn handle_event(_: &Model) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    use crossterm::event::KeyCode::*;

    match key.code {
        Char(c) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) && c == 'q' {
                Some(Message::App(AppMessage::Quit))
            } else {
                Some(Message::App(AppMessage::UpdateQueryAppend(c)))
            }
        }
        Home => Some(Message::App(AppMessage::MoveStart)),
        Up => Some(Message::App(AppMessage::MoveUp)),
        Down => Some(Message::App(AppMessage::MoveDown)),
        End => Some(Message::App(AppMessage::MoveEnd)),
        Esc => Some(Message::App(AppMessage::ClearQuery)),
        Enter => Some(Message::App(AppMessage::Select)),
        Backspace => Some(Message::App(AppMessage::UpdateQueryBackspace)),
        _ => None,
    }
}

fn update_modal(_model: &mut Model, _msg: crate::modal::message::ModalMessage) -> Option<Message> {
    None
}
