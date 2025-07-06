use crate::ui::components::app::{
    browse::{contact_list, search},
    message::AppMsg,
};

pub enum BrowseMsg {
    App(Box<AppMsg>),
    Search(search::message::SearchMessage),
    List(contact_list::message::ContactListMessage),
    Select,
    Activate,
    FilterUpdated,
    Delete,
    Add,
    Edit,
}

impl BrowseMsg {
    pub fn app(msg: AppMsg) -> Self {
        Self::App(Box::new(msg))
    }
}
