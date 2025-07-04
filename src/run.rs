use std::time::{Duration, Instant};

use crate::{
    Db,
    error::AppResult as Result,
    model::Contact,
    ui::components::{
        Component,
        app::{App, message::AppMessage},
    },
};
use crossterm::event::{self, Event};
use ratatui::prelude::*;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, db: Db) -> Result<Option<Contact>> {
    let mut app = App::new(db)?;
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| app.draw(f, f.area(), false))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_secs(0));

        if event::poll(timeout)?
            && let Event::Key(key_event) = event::read()?
            && let Some(msg) = app.handle_key(key_event)
        {
            propagate(&mut app, msg);
        }

        if last_tick.elapsed() <= tick_rate {
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }
    Ok(app.selected_contact)
}

fn propagate(app: &mut App, mut msg: AppMessage) {
    while let Some(next) = app.update(msg) {
        msg = next;
    }
}
