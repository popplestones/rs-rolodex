use clap::Parser;
use rolodex::cli::Cli;
use rolodex::config::Config;
use rolodex::error::AppResult as Result;
use rolodex::run::run_app;
use rolodex::{Db, tui};

fn main() -> Result<()> {
    tui::install_panic_hook();

    let args = Cli::parse();
    let config = Config::load()?;
    let mut db = Db::open(&config.database_path)?;

    if let Some(count) = args.seed {
        db.seed(count)?;
        return Ok(());
    }

    let mut terminal = tui::init_terminal()?;
    let selected = run_app(&mut terminal, db)?;
    tui::restore_terminal()?;

    if let Some(contact) = selected {
        let json = serde_json::to_string_pretty(&contact)?;
        println!("{}", json);
    }

    Ok(())
}
