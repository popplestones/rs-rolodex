pub enum SearchMsg {}
pub enum SearchOutput {}

#[derive(Debug, Default)]
pub struct Search {}

impl Search {
    pub fn new() -> Self {
        Self::default()
    }
}
