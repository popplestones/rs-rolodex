#[derive(Debug)]
pub enum Message {
    App(crate::app::message::AppMessage),
    Modal(crate::modal::ModalMessage),
}
