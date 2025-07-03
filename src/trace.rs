use std::fs::File;

use tracing::Level;
use tracing_subscriber::fmt::{SubscriberBuilder, writer::BoxMakeWriter};

use crate::error::AppResult as Result;

pub fn init() -> Result<()> {
    let file = File::create("/tmp/rolodex.log")?;
    let writer = BoxMakeWriter::new(file);

    SubscriberBuilder::default()
        .with_writer(writer)
        .with_max_level(Level::DEBUG)
        .init();
    Ok(())
}
