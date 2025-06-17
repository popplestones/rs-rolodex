use crate::contact::Contact;
use crate::modal::Modal;

#[derive(Debug, Default)]
pub struct Model {
    pub all_contacts: Vec<Contact>,
    pub filtered_contacts: Vec<Contact>,
    pub search_query: String,
    pub selected_index: u32,
    #[allow(dead_code)]
    pub mode: Mode,
    pub running_state: RunningState,
    pub modal: Option<Modal>,
}

impl Model {
    pub fn new(contacts: Vec<Contact>) -> Self {
        Self {
            filtered_contacts: contacts.clone(),
            all_contacts: contacts,
            selected_index: 0,
            search_query: String::new(),
            running_state: RunningState::Running,
            mode: Mode::Browse,
            modal: None,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
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
