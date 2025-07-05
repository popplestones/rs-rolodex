pub mod browse;
pub mod message;
use crate::{
    Db,
    error::AppResult as Result,
    mode::AppMode,
    model::Contact,
    ui::components::{
        Component,
        add_contact::AddContactForm,
        delete_confirmation::{DeleteConfirmation, message::DeleteMessage},
    },
    view::error,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use message::AppMessage;
use ratatui::prelude::*;

pub struct App {
    pub db: Db,
    pub selected_contact: Option<Contact>,
    pub all_contacts: Vec<Contact>,
    pub mode: AppMode,
    pub should_quit: bool,
    pub browse: browse::Browse,
    pub add_contact_form: AddContactForm,
    pub delete_confirmation: DeleteConfirmation,
}

impl App {
    pub fn new(db: Db) -> Result<Self> {
        let all_contacts = db.load_customers()?;
        let browse = browse::Browse::new(&all_contacts);
        Ok(Self {
            db,
            selected_contact: None,
            all_contacts,
            mode: AppMode::Browse,
            should_quit: false,
            browse,
            add_contact_form: AddContactForm::new(),
            delete_confirmation: DeleteConfirmation::new(),
        })
    }

    pub fn set_error(&mut self, msg: impl Into<String>) {
        self.mode = AppMode::Error(msg.into());
    }

    pub fn clear_error(&mut self) {
        if matches!(self.mode, AppMode::Error(_)) {
            self.mode = AppMode::Browse;
        }
    }
    // pub fn select_contact(&mut self) {
    //     self.selected_contact =
    //         Some(self.browse.filtered_contacts[self.browse.selected_index].clone());
    // }
    // pub fn unselect_contact(&mut self) {
    //     self.selected_contact = None;
    // }
}

impl Component<AppMessage, AppMessage> for App {
    fn draw(&self, f: &mut Frame, rect: Rect, _is_focussed: bool) {
        // Step 1: Draw main browse ui

        self.browse.draw(f, rect, false);

        // Step 2: overlay mode-specific view (like modals)
        match self.mode {
            AppMode::Error(_) => error::draw(f, self),
            AppMode::Delete => self.delete_confirmation.draw(f, f.area(), false),
            AppMode::Add => self.add_contact_form.draw(f, f.area(), false),
            _ => {}
        }
    }

    fn update(&mut self, message: AppMessage) -> Option<AppMessage> {
        match message {
            AppMessage::Add(msg) => self.add_contact_form.update(msg),
            AppMessage::Browse(msg) => self.browse.update(msg),
            AppMessage::SelectContact(contact) => {
                self.selected_contact = Some(contact);
                self.should_quit = true;
                None
            }
            AppMessage::Delete(contact) => {
                self.mode = AppMode::Delete;
                self.delete_confirmation.set_contact(contact);
                None
            }
            AppMessage::ConfirmDelete => {
                if let Some(id) = self.delete_confirmation.get_contact_id() {
                    match self.db.delete_contact(id) {
                        Ok(_) => {
                            self.mode = AppMode::Browse;
                            self.browse.update_filter();
                            self.delete_confirmation.clear_contact();
                            None
                        }
                        Err(e) => Some(AppMessage::Error(e.to_string())),
                    }
                } else {
                    Some(AppMessage::Error("Contact not found".to_string()))
                }
            }
            AppMessage::CancelDelete => {
                self.mode = AppMode::Browse;
                self.delete_confirmation.clear_contact();
                None
            }
            AppMessage::Error(msg) => {
                self.set_error(msg);
                None
            }
            AppMessage::Quit => {
                self.should_quit = true;
                None
            }
        }
    }

    fn handle_key(&self, event: KeyEvent) -> Option<AppMessage> {
        // Handle global app keys
        match event.code {
            KeyCode::Char('q') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                return Some(AppMessage::Quit);
            }
            _ => {}
        }

        // Handle mode-specific keys
        match self.mode {
            AppMode::Browse => self.browse.handle_key(event).map(AppMessage::Browse),
            AppMode::Add => self.add_contact_form.handle_key(event).map(AppMessage::Add),
            AppMode::Delete => match self.delete_confirmation.handle_key(event) {
                Some(DeleteMessage::Confirm) => Some(AppMessage::ConfirmDelete),
                Some(DeleteMessage::Cancel) => Some(AppMessage::CancelDelete),
                _ => None,
            },
            _ => None,
        }
    }
}
