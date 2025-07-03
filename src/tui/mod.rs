use std::panic;

use crate::error::AppResult as Result;
use crossterm::{
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};

pub fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = std::io::stderr().execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
        original_hook(panic_info);
    }));
}

pub fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    std::io::stderr().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    Ok(terminal)
}

pub fn restore_terminal() -> Result<()> {
    std::io::stderr().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
