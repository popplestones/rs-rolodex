use std::time::{Duration, Instant};

use crossterm::{
    ExecutableCommand,
    cursor::SetCursorStyle,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
};
use ratatui::prelude::*;
use tracing::{debug, info};

use crate::{
    Db,
    components::{
        Component,
        browse::{Browse, BrowseMsg, BrowseOutput},
        delete_confirmation::{DeleteConfirmation, DeleteMsg, DeleteOutput},
        error_dialog::{ErrorDialog, ErrorMsg, ErrorOutput},
        form::{Form, FormMsg, FormOutput},
    },
    error::AppResult as Result,
    layout::fixed_centered_rect,
    model::Contact,
};

pub enum AppMsg {
    // UI events from components
    Browse(BrowseMsg),
    Form(FormMsg),
    DeleteDialog(DeleteMsg),
    ErrorDialog(ErrorMsg),

    //High-level app messages
    AddContact,
    EditContact(Contact),
    ConfirmDelete(Contact),
    ShowError(String),
    Quit,
}
pub type AppOutput = AppMsg;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AppMode {
    #[default]
    Browse,
    Form,
    Delete,
    Error,
}

pub struct App {
    pub db: Db,
    pub selected_contact: Option<Contact>,
    pub mode: AppMode,
    pub should_quit: bool,
    pub browse: Browse,
    pub contact_form: Form,
    pub error_dialog: ErrorDialog,
    pub delete_confirmation: DeleteConfirmation,
}

impl App {
    pub fn new(db: Db) -> Result<Self> {
        let all_contacts = db.load_customers()?;
        let browse = Browse::new(&all_contacts);
        let mut error_dialog = ErrorDialog::new();
        error_dialog.set_error("Error loading contacts");

        let selected_contact = browse.contact_list.get_selected_contact();
        let mut delete_confirmation = DeleteConfirmation::new();
        if let Some(contact) = selected_contact {
            delete_confirmation.set_contact(contact);
        }
        Ok(Self {
            db,
            selected_contact: browse.contact_list.get_selected_contact(),
            mode: AppMode::Browse,
            should_quit: false,
            browse,
            contact_form: Form::new(),
            error_dialog,
            delete_confirmation,
        })
    }
    pub fn run<B: Backend>(terminal: &mut Terminal<B>, db: Db) -> Result<Option<Contact>> {
        std::io::stderr()
            .execute(SetCursorStyle::BlinkingBar)
            .expect("Failed to set cursor style");
        let mut app = App::new(db)?;
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|f| app.draw(f, f.area(), false))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or(Duration::from_secs(0));

            if event::poll(timeout)?
                && let Event::Key(key_event) = event::read()?
                && let Some(mut msg) = app.handle_key(key_event)
            {
                while let Some(next) = app.update(msg, |msg| msg) {
                    msg = next;
                }
            }

            last_tick = Instant::now();

            if app.should_quit {
                debug!("Quitting");
                break;
            }
        }
        Ok(app.selected_contact)
    }
    fn dismiss_error(&mut self) {
        self.mode = AppMode::Browse;
        self.error_dialog.set_error("");
    }
    fn refresh_contacts(&mut self) -> Option<AppMsg> {
        let result = self.db.load_customers();
        if let Err(err) = result {
            return Some(AppMsg::ShowError(err.to_string()));
        }
        self.browse.set_contacts(&result.unwrap());
        None
    }
    pub fn draw(&self, f: &mut Frame, area: Rect, focused: bool) {
        // This is where we split our frame into multiple areas and delegate to our components to
        // draw themselves.

        // Draw the main UI
        self.browse.draw(f, area, focused);

        // Draw any mode related UI overlaying the main UI
        match self.mode {
            AppMode::Browse => {}
            AppMode::Form => {
                let overlay = fixed_centered_rect(50, 10, area);
                self.contact_form.draw(f, overlay, true);
            }
            AppMode::Delete => {
                let overlay = fixed_centered_rect(60, 12, area);
                self.delete_confirmation.draw(f, overlay, true);
            }
            AppMode::Error => {
                let overlay = fixed_centered_rect(40, 8, area);
                self.error_dialog.draw(f, overlay, true);
            }
        }
    }
    pub fn handle_key(&self, event: KeyEvent) -> Option<AppMsg> {
        // Handle global app keys
        match event.code {
            KeyCode::Char('q') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                info!("Ctrl+Q pressed - Quitting");
                return Some(AppMsg::Quit);
            }
            KeyCode::Char('a') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                info!("Ctrl+A pressed - Opening Add Contact Modal");
                return Some(AppMsg::AddContact);
            }
            KeyCode::Char('e') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                info!("Ctrl+E pressed - Opening Edit Contact Modal");
                return self.selected_contact.clone().map(AppMsg::EditContact);
            }
            KeyCode::Char('d') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                info!("Ctrl+D pressed - Opening Delete Modal");
                return self.selected_contact.clone().map(AppMsg::ConfirmDelete);
            }
            _ => {}
        }

        // Handle mode-specific keys
        match self.mode {
            AppMode::Browse => self.browse.handle_key(event).map(AppMsg::Browse),
            AppMode::Form => self.contact_form.handle_key(event).map(AppMsg::Form),
            AppMode::Delete => self
                .delete_confirmation
                .handle_key(event)
                .map(AppMsg::DeleteDialog),
            AppMode::Error => self.error_dialog.handle_key(event).map(AppMsg::ErrorDialog),
        }
    }
    pub fn update<ParentMsg>(
        &mut self,
        msg: AppMsg,
        map: impl Fn(AppOutput) -> ParentMsg,
    ) -> Option<ParentMsg> {
        match msg {
            AppMsg::Quit => {
                self.should_quit = true;
                self.selected_contact = None;
                None
            }
            AppMsg::Browse(browse_msg) => {
                match self.browse.update(browse_msg, |output| output) {
                    Some(BrowseOutput::ContactSelected(contact)) => {
                        info!("Contact selected: {:?}", contact);
                        self.selected_contact = Some(contact);
                    }
                    Some(BrowseOutput::ContactActivated(contact)) => {
                        self.selected_contact = Some(contact);
                        self.should_quit = true;
                    }
                    _ => {}
                }
                None
            }
            AppMsg::Form(form_msg) => {
                let form_output = self.contact_form.update(form_msg, |output| output);
                match form_output {
                    Some(FormOutput::Submitted(contact)) => {
                        info!("Contact submitted: {:?}", contact);
                        if contact.id == 0 {
                            let result = self.db.add_contact(contact);
                            if let Err(err) = result {
                                return Some(map(AppMsg::ShowError(err.to_string())));
                            }
                        } else {
                            let result = self.db.update_contact(contact.id, contact);
                            if let Err(err) = result {
                                return Some(map(AppMsg::ShowError(err.to_string())));
                            }
                        }
                        self.refresh_contacts();
                        self.mode = AppMode::Browse;
                    }
                    Some(FormOutput::Cancelled) => {
                        self.mode = AppMode::Browse;
                    }
                    None => {}
                }

                None
            }
            AppMsg::DeleteDialog(delete_msg) => {
                match self.delete_confirmation.update(delete_msg, |output| output) {
                    Some(DeleteOutput::Confirmed(contact)) => {
                        self.mode = AppMode::Browse;
                        let result = self.db.delete_contact(contact.id);
                        self.refresh_contacts();
                        if let Err(err) = result {
                            return Some(map(AppMsg::ShowError(err.to_string())));
                        }
                        None
                    }
                    Some(DeleteOutput::Cancelled) => {
                        self.mode = AppMode::Browse;
                        None
                    }
                    None => None,
                }
            }
            AppMsg::ErrorDialog(error_msg) => {
                if let Some(output) = self.error_dialog.update(error_msg, |output| output) {
                    match output {
                        ErrorOutput::Dismissed => self.dismiss_error(),
                    }
                }
                None
            }
            AppMsg::AddContact => {
                self.mode = AppMode::Form;
                self.contact_form.set_contact(Contact::default());
                None
            }
            AppMsg::EditContact(_contact) => {
                self.mode = AppMode::Form;
                if let Some(contact) = &self.selected_contact {
                    self.contact_form.set_contact(contact.clone());
                }
                None
            }
            AppMsg::ConfirmDelete(contact) => {
                self.delete_confirmation.set_contact(contact);
                self.mode = AppMode::Delete;
                None
            }
            AppMsg::ShowError(error) => {
                self.error_dialog.set_error(&error);
                self.mode = AppMode::Error;
                None
            }
        }
    }
}

impl Component for App {
    type Msg = AppMsg;
    type Output = AppMsg;

    fn update<ParentMsg>(
        &mut self,
        msg: Self::Msg,
        map: impl Fn(Self::Output) -> ParentMsg,
    ) -> Option<ParentMsg> {
        self.update(msg, map)
    }

    fn handle_key(&self, key: KeyEvent) -> Option<Self::Msg> {
        self.handle_key(key)
    }

    fn draw(&self, f: &mut Frame, area: Rect, focused: bool) {
        self.draw(f, area, focused)
    }
}
