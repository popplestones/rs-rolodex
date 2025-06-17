use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::contact::{Contact, load_contacts_from_file};

#[derive(Debug, Default)]
pub struct Model {
    pub all_contacts: Vec<Contact>,
    pub filtered_contacts: Vec<Contact>,
    pub search_query: String,
    pub selected_index: u32,
    #[allow(dead_code)]
    pub mode: Mode,
    pub running_state: RunningState,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(PartialEq)]
pub enum Message {
    MoveUp,
    MoveDown,
    MoveStart,
    MoveEnd,
    Quit,
    UpdateQueryAppend(char),
    UpdateQueryBackspace,
    ClearQuery,
    Select,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Mode {
    #[default]
    Browse,
    Add,
    Edit,
    Delete,
}
fn apply_filter(model: &mut Model) {
    model.filtered_contacts = model
        .all_contacts
        .iter()
        .filter(|c| {
            let query = model.search_query.to_lowercase();

            c.name.to_lowercase().contains(&query) || c.company.to_lowercase().contains(&query)
        })
        .cloned()
        .collect();
}

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::MoveUp => model.selected_index = model.selected_index.saturating_sub(1),
        Message::MoveDown => model.selected_index = model.selected_index.saturating_add(1),
        Message::MoveStart => model.selected_index = 0,
        Message::MoveEnd => {
            model.selected_index = (model.filtered_contacts.len() as u32).saturating_sub(1)
        }
        Message::Quit => {
            model.running_state = RunningState::Done;
        }
        Message::UpdateQueryAppend(c) => {
            model.search_query.push(c);
            apply_filter(model);
        }
        Message::UpdateQueryBackspace => {
            model.search_query.pop();
            apply_filter(model);
        }
        Message::ClearQuery => {
            if model.search_query.is_empty() {
                model.running_state = RunningState::Done;
            } else {
                model.search_query.clear();
                apply_filter(model);
            }
        }
        Message::Select => {
            model.running_state = RunningState::Done;
        }
    };
    None
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
    match key.code {
        KeyCode::Char(c) => {
            if key.modifiers.contains(event::KeyModifiers::CONTROL) && c == 'q' {
                Some(Message::Quit)
            } else {
                Some(Message::UpdateQueryAppend(c))
            }
        }
        KeyCode::Home => Some(Message::MoveStart),
        KeyCode::Up => Some(Message::MoveUp),
        KeyCode::Down => Some(Message::MoveDown),
        KeyCode::End => Some(Message::MoveEnd),
        KeyCode::Esc => Some(Message::ClearQuery),
        KeyCode::Enter => Some(Message::Select),
        KeyCode::Backspace => Some(Message::UpdateQueryBackspace),
        _ => None,
    }
}

pub fn init_model(path: &str) -> color_eyre::Result<Model> {
    let contacts = load_contacts_from_file(path)?;
    Ok(Model {
        all_contacts: contacts.clone(),
        filtered_contacts: contacts,
        search_query: String::new(),
        selected_index: 0,
        mode: Mode::Browse,
        running_state: RunningState::Running,
    })
}
