pub mod browse;
pub mod message;
use crate::{
    Db,
    error::AppResult as Result,
    mode::AppMode,
    model::Contact,
    ui::components::{
        Component,
        contact_form::ContactForm,
        delete_confirmation::{DeleteConfirmation, message::DeleteMessage},
    },
    view::error,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use message::AppMsg;
use ratatui::prelude::*;
use tracing::info;

pub struct App {
    pub db: Db,
    pub selected_contact: Option<Contact>,
    pub mode: AppMode,
    pub should_quit: bool,
    pub browse: browse::Browse,
    pub contact_form: ContactForm,
    pub delete_confirmation: DeleteConfirmation,
}

impl App {
    pub fn new(db: Db) -> Result<Self> {
        let all_contacts = db.load_customers()?;
        let browse = browse::Browse::new(&all_contacts);
        Ok(Self {
            db,
            selected_contact: browse.contact_list.get_selected_contact(),
            mode: AppMode::Browse,
            should_quit: false,
            browse,
            contact_form: ContactForm::new(),
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
    pub fn delete_selected_contact(&mut self) -> Option<AppMsg> {
        info!("Selected contact: {:?}", self.selected_contact);
        if let Some(contact) = &self.selected_contact {
            match self.db.delete_contact(contact.id) {
                Ok(_) => {
                    self.browse.delete_contact(contact.id);
                    self.mode = AppMode::Browse;
                    self.browse.update_filter();
                    self.selected_contact = self.browse.contact_list.get_selected_contact();
                }
                Err(e) => self.set_error(e.to_string()),
            }
        }
        None
    }
}

impl Component<AppMsg, AppMsg> for App {
    fn draw(&self, f: &mut Frame, rect: Rect, _is_focussed: bool) {
        // Step 1: Draw main browse ui

        self.browse.draw(f, rect, false);

        // Step 2: overlay mode-specific view (like modals)
        match self.mode {
            AppMode::Error(_) => error::draw(f, self),
            AppMode::Delete => self.delete_confirmation.draw(f, f.area(), false),
            AppMode::ContactForm => self.contact_form.draw(f, f.area(), false),
            _ => {}
        }
    }

    fn update(&mut self, message: AppMsg) -> Option<AppMsg> {
        match message {
            AppMsg::ContactForm(msg) => self.contact_form.update(msg),
            AppMsg::Browse(msg) => self.browse.update(msg),
            AppMsg::SelectContact(contact) => {
                self.selected_contact = Some(contact);
                None
            }
            AppMsg::Delete(contact) => {
                self.mode = AppMode::Delete;
                self.delete_confirmation.set_contact(contact);
                None
            }
            AppMsg::OpenContactForm(contact) => {
                self.mode = AppMode::ContactForm;
                if let Some(contact) = contact {
                    self.contact_form.set_contact(contact);
                }
                None
            }
            AppMsg::ConfirmDelete => self.delete_selected_contact(),
            AppMsg::CancelDelete => {
                self.mode = AppMode::Browse;
                self.delete_confirmation.clear_contact();
                None
            }
            AppMsg::Error(msg) => {
                self.set_error(msg);
                None
            }
            AppMsg::Quit => {
                self.should_quit = true;
                self.selected_contact = None;
                None
            }
            AppMsg::SaveContact(contact) => todo!(),
            AppMsg::CancelForm => {
                self.contact_form = ContactForm::new();
                self.mode = AppMode::Browse;
                None
            }
        }
    }

    fn handle_key(&self, event: KeyEvent) -> Option<AppMsg> {
        // Handle global app keys
        match event.code {
            KeyCode::Char('q') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                return Some(AppMsg::Quit);
            }
            _ => {}
        }

        // Handle mode-specific keys
        match self.mode {
            AppMode::Browse => self.browse.handle_key(event).map(AppMsg::Browse),
            AppMode::ContactForm => self.contact_form.handle_key(event).map(AppMsg::ContactForm),
            AppMode::Delete => match self.delete_confirmation.handle_key(event) {
                Some(DeleteMessage::Confirm) => Some(AppMsg::ConfirmDelete),
                Some(DeleteMessage::Cancel) => Some(AppMsg::CancelDelete),
                _ => None,
            },
            _ => None,
        }
    }
}
