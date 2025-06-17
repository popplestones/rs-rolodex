pub mod confirm_delete;
pub mod edit_contact;
pub mod message;

pub use message::ModalMessage;

#[derive(Debug)]
pub enum Modal {
    ConfirmDelete,
    EditContact,
}

impl Modal {
    pub fn update(&mut self, msg: ModalMessage) -> Option<ModalMessage> {
        // Stubbed: real dispatch to submodules would go here
        Some(msg)
    }
}
