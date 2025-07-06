use crossterm::event::KeyEvent;

pub enum ContactFormMsg {
    NextField,
    PrevField,
    Confirm,
    Cancel,
    Input(KeyEvent, usize),
}
