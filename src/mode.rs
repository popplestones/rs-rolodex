#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppMode {
    #[default]
    Browse,
    Delete,
    Add,
    Error,
}
