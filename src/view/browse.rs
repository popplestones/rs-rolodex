use ratatui::{prelude::*, widgets::*};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    draw_search(f, app, chunks[0]);
    draw_contact_list(f, app, chunks[1]);
}

pub fn draw_search(f: &mut Frame, app: &App, area: Rect) {
    let search = Paragraph::new(app.browse.search_input.as_str())
        .block(Block::default().borders(Borders::ALL).title("Search"));

    f.render_widget(search, area);
}

pub fn draw_contact_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .browse
        .filtered_contacts
        .iter()
        .map(|display| {
            ListItem::new(format!(
                "{} {} {} {}",
                display.name, display.company, display.email, display.phone
            ))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Contacts"))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(app.browse.selected_index));

    f.render_stateful_widget(list, area, &mut list_state);
}
