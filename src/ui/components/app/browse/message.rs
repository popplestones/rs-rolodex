use crate::ui::components::app::browse::{contact_list, search};

pub enum BrowseMsg {
    Search(search::message::SearchMessage),
    List(contact_list::message::ContactListMessage),
    Select,
    Activate,
    FilterUpdated,
    Delete,
    Add,
    Edit,
}
