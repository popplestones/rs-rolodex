use std::time::{Duration, Instant};

use crossterm::{
    ExecutableCommand,
    cursor::SetCursorStyle,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
};
use ratatui::{
    prelude::*,
    layout::{Constraint, Direction, Layout},
};
use tracing::{debug, info};

use crate::{
    Db,
    components::{
        Component,
        browse::{Browse, BrowseMsg, BrowseOutput},
        delete_confirmation::{DeleteConfirmation, DeleteMsg, DeleteOutput},
        error_dialog::{ErrorDialog, ErrorMsg, ErrorOutput},
        form::{Form, FormMsg, FormOutput},
        status_bar::{StatusBar, StatusBarMsg},
    },
    error::AppResult as Result,
    layout::fixed_centered_rect,
    model::Contact,
    mode::AppMode,
};

pub enum AppMsg {
    // UI events from components
    Browse(BrowseMsg),
    Form(FormMsg),
    DeleteDialog(DeleteMsg),
    ErrorDialog(ErrorMsg),
    StatusBar(StatusBarMsg),

    //High-level app messages
    AddContact,
    EditContact(Contact),
    ConfirmDelete(Contact),
    ShowError(String),
    Quit,
}
pub type AppOutput = AppMsg;



pub struct App {
    pub db: Db,
    pub selected_contact: Option<Contact>,
    pub mode: AppMode,
    pub should_quit: bool,
    pub browse: Browse,
    pub contact_form: Form,
    pub error_dialog: ErrorDialog,
    pub delete_confirmation: DeleteConfirmation,
    pub status_bar: StatusBar,
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
        let mut status_bar = StatusBar::new();
        status_bar.update(StatusBarMsg::UpdateCounts {
            total: all_contacts.len(),
            filtered: all_contacts.len(),
        });
        status_bar.update(StatusBarMsg::UpdateMode(AppMode::Browse));

        Ok(Self {
            db,
            selected_contact: browse.contact_list.get_selected_contact(),
            mode: AppMode::Browse,
            should_quit: false,
            browse,
            contact_form: Form::new(),
            error_dialog,
            delete_confirmation,
            status_bar,
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
        self.update_status_bar_mode();
    }
    fn refresh_contacts(&mut self) -> Option<AppMsg> {
        let result = self.db.load_customers();
        if let Err(err) = result {
            return Some(AppMsg::ShowError(err.to_string()));
        }
        self.browse.set_contacts(&result.unwrap());
        self.update_status_bar_counts();
        None
    }

    fn update_status_bar_counts(&mut self) {
        let total = self.browse.all_contacts.len();
        let filtered = self.browse.contact_list.filtered_contacts.len();
        self.status_bar.update(StatusBarMsg::UpdateCounts { total, filtered });
    }

    fn update_status_bar_mode(&mut self) {
        self.status_bar.update(StatusBarMsg::UpdateMode(self.mode.clone()));
    }
    pub fn draw(&self, f: &mut Frame, area: Rect, focused: bool) {
        // This is where we split our frame into multiple areas and delegate to our components to
        // draw themselves.

        // Split the area into main content and status bar
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        let main_area = chunks[0];
        let status_area = chunks[1];

        // Draw the main UI
        self.browse.draw(f, main_area, focused);

        // Draw the status bar
        self.status_bar.draw(f, status_area, false);

        // Draw any mode related UI overlaying the main UI
        match self.mode {
            AppMode::Browse => {}
            AppMode::ContactForm => {
                let overlay = fixed_centered_rect(50, 10, main_area);
                self.contact_form.draw(f, overlay, true);
            }
            AppMode::Delete => {
                let overlay = fixed_centered_rect(60, 12, main_area);
                self.delete_confirmation.draw(f, overlay, true);
            }
            AppMode::Error(_) => {
                let overlay = fixed_centered_rect(40, 8, main_area);
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
            AppMode::ContactForm => self.contact_form.handle_key(event).map(AppMsg::Form),
            AppMode::Delete => self
                .delete_confirmation
                .handle_key(event)
                .map(AppMsg::DeleteDialog),
            AppMode::Error(_) => self.error_dialog.handle_key(event).map(AppMsg::ErrorDialog),
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
                // Update status bar counts after any browse operation (search might have changed filtered count)
                self.update_status_bar_counts();
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
                        self.update_status_bar_mode();
                    }
                    Some(FormOutput::Cancelled) => {
                        self.mode = AppMode::Browse;
                        self.update_status_bar_mode();
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
                        self.update_status_bar_mode();
                        if let Err(err) = result {
                            return Some(map(AppMsg::ShowError(err.to_string())));
                        }
                        None
                    }
                    Some(DeleteOutput::Cancelled) => {
                        self.mode = AppMode::Browse;
                        self.update_status_bar_mode();
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
                self.mode = AppMode::ContactForm;
                self.contact_form.set_contact(Contact::default());
                self.update_status_bar_mode();
                None
            }
            AppMsg::EditContact(_contact) => {
                self.mode = AppMode::ContactForm;
                if let Some(contact) = &self.selected_contact {
                    self.contact_form.set_contact(contact.clone());
                }
                self.update_status_bar_mode();
                None
            }
            AppMsg::ConfirmDelete(contact) => {
                self.delete_confirmation.set_contact(contact);
                self.mode = AppMode::Delete;
                self.update_status_bar_mode();
                None
            }
            AppMsg::ShowError(error) => {
                self.error_dialog.set_error(&error);
                self.mode = AppMode::Error(error);
                self.update_status_bar_mode();
                None
            }
            AppMsg::StatusBar(_) => {
                // Status bar messages don't need handling at app level
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
