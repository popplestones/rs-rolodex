use std::time::{Duration, Instant};

use crate::{Db, app::App, error::AppResult as Result, input::handler::handle_input, view::draw};
use crossterm::event::{self, Event};
use ratatui::prelude::*;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, db: Db) -> Result<()> {
    let mut app = App::new(db);
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| draw(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key_event) = event::read()? {
                if let Err(err) = handle_input(&mut app, key_event) {
                    app.set_error(err.to_string());
                }
            }
        }

        if last_tick.elapsed() <= tick_rate {
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
