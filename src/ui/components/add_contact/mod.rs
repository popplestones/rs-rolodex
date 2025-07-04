pub mod message;
use message::AddContactFormMessage;

use crate::{
    model::Contact,
    ui::{
        components::{Component, text_field::TextField},
        layout::centered_rect,
    },
};
use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::*};
use tracing::info;

#[derive(Default)]
pub struct AddContactForm {
    pub fields: Vec<TextField>,
    pub focused: usize,
}

impl AddContactForm {
    pub fn new() -> Self {
        Self {
            fields: vec![
                TextField::new("Name"),
                TextField::new("Company"),
                TextField::new("Email"),
                TextField::new("Phone"),
            ],
            focused: 0,
        }
    }
    pub fn to_contact(&self) -> Contact {
        Contact {
            id: 0,
            name: self.fields[0].value.trim().to_string(),
            company: opt(self.fields[1].value.trim()),
            email: opt(self.fields[2].value.trim()),
            phone: opt(self.fields[3].value.trim()),
        }
    }

    pub fn clear(&mut self) {
        for field in self.fields.iter_mut() {
            field.value.clear();
        }
        self.focused = 0;
    }
}
use crate::ui::components::app::message::AppMessage;
impl Component<AddContactFormMessage, AppMessage> for AddContactForm {
    fn handle_key(&self, _event: KeyEvent) -> Option<AddContactFormMessage> {
        // match event.code {
        //     KeyCode::Tab => {
        //         if self.focused < self.fields.len() - 1 {
        //             self.focused += 1;
        //         } else {
        //             self.focused = 0;
        //         }
        //     }
        //     KeyCode::BackTab => {
        //         if self.focused > 0 {
        //             self.focused -= 1;
        //         } else {
        //             self.focused = self.fields.len() - 1;
        //         }
        //     }
        //     _ => {
        //         if let Some(field) = self.fields.get_mut(self.focused) {
        //             field.handle_key(event);
        //         }
        //     }
        // }
        None
    }

    fn draw(&self, f: &mut Frame, _: Rect, _: bool) {
        info!("Fields: {}", self.fields.len());
        let area = centered_rect(60, (self.fields.len() * 3 + 2) as u16, f.area());

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
            .constraints(vec![Constraint::Length(3); self.fields.len()])
            .split(inner);

        for (i, (field, rect)) in self.fields.iter().zip(chunks.iter()).enumerate() {
            let is_focused = i == self.focused;
            field.draw(f, *rect, is_focused);
        }
    }

    fn update(&mut self, _message: AddContactFormMessage) -> Option<AppMessage> {
        todo!()
    }
}
fn opt(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}
