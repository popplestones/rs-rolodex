use crate::model::Contact;
use crate::ui::components::app::browse::message::BrowseMsg;
use crate::ui::components::contact_form::message::ContactFormMsg;

pub enum AppMessage {
    OpenContactForm(Option<Contact>),
    ContactForm(ContactFormMsg),
    Delete(Contact),
    SaveContact(Contact),
    CancelForm,
    ConfirmDelete,
    CancelDelete,
    Browse(BrowseMsg),
    Error(String),
    SelectContact(Contact),
    Quit,
}
