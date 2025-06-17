#[derive(Debug)]
pub enum EditContactMessage {
    AppendChar(char),
    Backspace,
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorStart,
    MoveCursorEnd,
    FocusNext,
    FocusPrev,
    Submit,
    Cancel,
}
