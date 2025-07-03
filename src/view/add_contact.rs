use ratatui::{prelude::*, widgets::*};
use tracing::info;

use crate::{app::App, ui::layout::centered_rect};

pub fn draw(f: &mut Frame, app: &App) {
    let form = &app.add_contact_form;

    info!("Fields: {}", form.fields.len());
    let area = centered_rect(60, (form.fields.len() * 3 + 2) as u16, f.area());

    f.render_widget(Clear, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Add Contact")
        .style(Style::default().bg(Color::Black));

    f.render_widget(block, area);

    let inner = area.inner(Margin {
        vertical: 1,
        horizontal: 1,
    });

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3); form.fields.len()])
        .split(inner);

    for (i, (field, rect)) in form.fields.iter().zip(chunks.iter()).enumerate() {
        let is_focused = i == form.focused;
        field.draw(f, *rect, is_focused);
    }
}
