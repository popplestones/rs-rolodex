#[derive(Debug)]
pub enum AppMessage {
    MoveUp,
    MoveDown,
    MoveStart,
    MoveEnd,
    Quit,
    UpdateQueryAppend(char),
    UpdateQueryBackspace,
    ClearQuery,
    Select,
}
