use crate::ui::components::app::browse::{contact_list, search};

pub enum BrowseMessage {
    Search(search::message::SearchMessage),
    List(contact_list::message::ContactListMessage),
    Select,
    FilterUpdated,
    Delete,
}
