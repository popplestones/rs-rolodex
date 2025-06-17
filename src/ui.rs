use ratatui::{
    Frame,
    style::Style,
    text::{Line, Span, Text},
    widgets::Paragraph,
};

use crate::app::Model;

pub fn view(model: &mut Model, frame: &mut Frame) {
    use ratatui::style::Stylize;
    use ratatui::widgets::{Block, Borders};

    let mut lines = vec![];

    lines.push(Line::raw("Search: ".to_string() + &model.search_query));
    lines.push(Line::raw(""));
    lines.push(Line::raw(format!(
        "{:<20} {:<20} {:<15}",
        "Name", "Company", "Phone"
    )));
    lines.push(Line::raw(format!("{:<20} {:<20} {:<15}", "", "", "")));

    for (i, contact) in model.filtered_contacts.iter().enumerate() {
        let line = format!(
            "{:<20} {:<20} {:<15}",
            contact.name, contact.company, contact.phone
        );

        if i == model.selected_index as usize {
            lines.push(Line::from(Span::styled(line, Style::default().reversed())));
        } else {
            lines.push(Line::raw(contact.name.clone()));
        }
    }

    let paragraph = Paragraph::new(Text::from(lines))
        .block(Block::default().title("Contacts").borders(Borders::ALL));

    frame.render_widget(paragraph, frame.area());
}
