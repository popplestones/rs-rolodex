use crate::{app::App, ui::layout::centered_rect};
use ratatui::{prelude::*, widgets::*};

pub fn draw(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, f.area());

    let contact = app.selected_contact.as_ref();
    let name = contact.map(|c| c.name.as_str()).unwrap_or("this contact");

    let text = vec![
        Line::from(Span::styled(
            "Delete Contact",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!("Are you sure you want to delete '{name}'? [y/n]")),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Confirm")
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black).fg(Color::White));

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(block);

    f.render_widget(Clear, area); // clear beneath the popup
    f.render_widget(paragraph, area);
}

// center modal
