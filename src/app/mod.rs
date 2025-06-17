pub mod message;
pub mod state;
pub mod ui;
pub mod update;

pub use state::Model;

use crate::contact::load_contacts_from_file;

pub fn init_model(path: &str) -> color_eyre::Result<Model> {
    let contacts = load_contacts_from_file(path)?;
    Ok(Model::new(contacts))
}
