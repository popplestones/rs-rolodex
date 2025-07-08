use clap::Parser;
use rolodex::cli::Cli;
use rolodex::components::app::App;
use rolodex::config::Config;
use rolodex::error::AppResult as Result;
use rolodex::{Db, trace, tui};
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
