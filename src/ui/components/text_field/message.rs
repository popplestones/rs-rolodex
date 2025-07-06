#[derive(Debug)]
pub enum TextFieldMsg {
    Left,
    Right,
    Home,
    End,
    AddChar(char),
    Backspace,
    Delete,
    Clear,
}
