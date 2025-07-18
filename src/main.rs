use clap::Parser;
use rolodex_tui::cli::Cli;
use rolodex_tui::components::app::App;
use rolodex_tui::config::Config;
use rolodex_tui::error::AppResult as Result;
use rolodex_tui::{Db, trace, tui};
use tracing::{debug, info};

fn main() -> Result<()> {
    trace::init()?;
    info!("Starting log");
    tui::install_panic_hook();

    debug!("Parsing CLI Args");
    let args = Cli::parse();

    debug!("Loading config");
    let config = Config::load()?;

    debug!("Opening database");
    let mut db = Db::open(&config.database_path)?;

    if let Some(count) = args.seed {
        debug!("Seeding database with {count} contacts");
        db.seed(count)?;
        return Ok(());
    }

    if let Some(backup_path) = args.backup {
        debug!("Backing up contacts to {}", backup_path.display());
        db.backup_to_file(&backup_path)?;
        println!("Contacts backed up to {}", backup_path.display());
        return Ok(());
    }

    if let Some(restore_path) = args.restore {
        debug!("Restoring contacts from {}", restore_path.display());
        let count = db.restore_from_file(&restore_path)?;
        println!("Restored {} contacts from {}", count, restore_path.display());
        return Ok(());
    }

    let mut terminal = tui::init_terminal()?;
    debug!("Running app");
    let selected = App::run(&mut terminal, db)?;
    tui::restore_terminal()?;

    if let Some(contact) = selected {
        let json = serde_json::to_string_pretty(&contact)?;
        debug!("Selected contact: {json}");
        println!("{json}");
    }

    Ok(())
}
