use crate::model::Contact;
use crate::ui::components::add_contact::message::AddMessage;
use crate::ui::components::app::browse::message::BrowseMessage;
use crate::ui::components::delete_confirmation::message::DeleteMessage;

pub enum AppMessage {
    Add(AddMessage),
    Delete(DeleteMessage),
    Browse(BrowseMessage),
    SelectContact(Contact),
    Quit,
}
