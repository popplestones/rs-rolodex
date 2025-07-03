use ratatui::{prelude::*, widgets::*};

use crate::ui::components::app::App;

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
        .map(|c| {
            let company = c.company.as_deref().unwrap_or("<none>");
            let email = c.email.as_deref().unwrap_or("<none>");
            let phone = c.phone.as_deref().unwrap_or("<none>");

            let line = format!("{:<24} {:<40} {:<40} {:<12}", c.name, company, email, phone);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Contacts"))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(app.browse.selected_index));

    f.render_stateful_widget(list, area, &mut list_state);

    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .thumb_style(Style::default().bg(Color::Gray))
        .track_style(Style::default().bg(Color::DarkGray));
    let mut scrollbar_state =
        ScrollbarState::new(app.browse.filtered_contacts.len()).position(app.browse.selected_index);

    f.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut scrollbar_state,
    );
}
