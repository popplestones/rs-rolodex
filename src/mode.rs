#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AppMode {
    #[default]
    Browse,
    Delete,
    Add,
    Error(String),
}
