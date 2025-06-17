use crate::modal::confirm_delete::ConfirmDeleteMessage;
use crate::modal::edit_contact::EditContactMessage;

#[derive(Debug)]
pub enum ModalMessage {
    ConfirmDelete(ConfirmDeleteMessage),
    EditContact(EditContactMessage),
}
