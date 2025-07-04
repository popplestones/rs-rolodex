use crate::model::Contact;
use crate::ui::components::app::browse::BrowseMessage;

pub enum AppMessage {
    Browse(BrowseMessage),
    SelectContact(Contact),
    Quit,
}
