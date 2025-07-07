use crate::{
    Db,
    components::{browse::Browse, delete_confirmation::DeleteConfirmation, form::Form},
    error::AppResult as Result,
    model::Contact,
};

pub enum AppMsg {}
pub enum AppOutput {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AppMode {
    #[default]
    Browse,
    Form,
    Delete,
    Error(String),
}

pub struct App {
    pub db: Db,
    pub selected_contact: Option<Contact>,
    pub mode: AppMode,
    pub should_quit: bool,
    pub browse: Browse,
    pub contact_form: Form,
    pub delete_confirmation: DeleteConfirmation,
}

impl App {
    pub fn new(db: Db) -> Result<Self> {
        let all_contacts = db.load_customers()?;
        let browse = Browse::new(&all_contacts);
        Ok(Self {
            db,
            selected_contact: browse.contact_list.get_selected_contact(),
            mode: AppMode::Browse,
            should_quit: false,
            browse,
            contact_form: Form::new(),
            delete_confirmation: DeleteConfirmation::new(),
        })
    }
}
