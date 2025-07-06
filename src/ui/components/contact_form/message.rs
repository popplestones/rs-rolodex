use crossterm::event::KeyEvent;

use crate::ui::components::text_field::message::TextFieldMsg;

pub enum ContactFormMsg {
    TextField(usize, TextFieldMsg),
    NextField,
    PrevField,
    Confirm,
    Cancel,
    Input(KeyEvent, usize),
}
