use ratatui::{prelude::*, widgets::*};

use crate::ui::components::app::App;

pub fn draw(f: &mut Frame, _app: &App) {
    let block = Block::default().title("Error").borders(Borders::ALL);
    f.render_widget(block, f.area());
}
