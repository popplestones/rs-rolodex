use rolodex::config::Config;
use rolodex::error::AppResult as Result;
use rolodex::run::run_app;
use rolodex::{Db, tui};

fn main() -> Result<()> {
    tui::install_panic_hook();

    let config = Config::load()?;
    let db = Db::open(&config.database_path)?;

    let mut terminal = tui::init_terminal()?;
    let res = run_app(&mut terminal, db);
    tui::restore_terminal()?;

    res
}
