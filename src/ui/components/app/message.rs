use crate::model::Contact;
use crate::ui::components::add_contact::message::AddMessage;
use crate::ui::components::app::browse::message::BrowseMessage;

pub enum AppMessage {
    Add(AddMessage),
    Delete(Contact),
    ConfirmDelete,
    CancelDelete,
    Browse(BrowseMessage),
    Error(String),
    SelectContact(Contact),
    Quit,
}
