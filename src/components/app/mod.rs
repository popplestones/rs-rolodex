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
        browse::{Browse, BrowseMsg},
        delete_confirmation::{DeleteConfirmation, DeleteMsg},
        error_dialog::{ErrorDialog, ErrorMsg},
        form::{Form, FormMsg},
    },
    error::AppResult as Result,
    layout::fixed_centered_rect,
    model::Contact,
};

pub enum AppMsg {
    Browse(BrowseMsg),
    Form(FormMsg),
    Delete(DeleteMsg),
    Error(ErrorMsg),
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

            if event::poll(timeout)? {
                if let Event::Key(key_event) = event::read()? {
                    if let Some(mut msg) = app.handle_key(key_event) {
                        while let Some(next) = app.update(msg, |msg| msg) {
                            msg = next;
                        }
                    }
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
    pub fn draw(&self, f: &mut Frame, area: Rect, focused: bool) {
        // This is where we split our frame into multiple areas and delegate to our components to
        // draw themselves.

        // Draw the main UI
        self.browse.draw(f, area, focused);

        // Draw any mode related UI overlaying the main UI
        match self.mode {
            AppMode::Browse => {}
            AppMode::Form => {
                let overlay = fixed_centered_rect(60, 20, area);
                self.contact_form.draw(f, overlay, true);
            }
            AppMode::Delete => {
                let overlay = fixed_centered_rect(60, 10, area);
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
            _ => {}
        }

        // Handle mode-specific keys
        match self.mode {
            AppMode::Browse => self.browse.handle_key(event).map(AppMsg::Browse),
            AppMode::Form => self.contact_form.handle_key(event).map(AppMsg::Form),
            AppMode::Delete => self
                .delete_confirmation
                .handle_key(event)
                .map(AppMsg::Delete),
            AppMode::Error => self.error_dialog.handle_key(event).map(AppMsg::Error),
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
                self.browse.update(browse_msg, |_| {});
                None
            }
            AppMsg::Form(_) => todo!(),
            AppMsg::Delete(_) => todo!(),
            AppMsg::Error(_) => todo!(),
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
