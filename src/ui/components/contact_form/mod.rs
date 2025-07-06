pub mod message;
use message::ContactFormMsg;
use tracing::info;

use crate::{
    model::Contact,
    ui::{
        components::{Component, app::message::AppMsg, text_field::TextField},
        layout::centered_rect,
    },
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

#[derive(Default)]
pub struct ContactForm {
    pub fields: Vec<TextField>,
    pub focused: usize,
    pub editing: Option<i64>,
}

impl ContactForm {
    pub fn new() -> Self {
        Self {
            fields: vec![
                TextField::new("Name"),
                TextField::new("Company"),
                TextField::new("Email"),
                TextField::new("Phone"),
            ],
            focused: 0,
            editing: None,
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

    pub fn set_contact(&mut self, contact: Contact) {
        self.editing = Some(contact.id);
        self.fields[0].value = contact.name;
        self.fields[1].value = contact.company.unwrap_or_default();
        self.fields[2].value = contact.email.unwrap_or_default();
        self.fields[3].value = contact.phone.unwrap_or_default();
        self.focused = 0;
        self.fields[0].end();
    }
}
impl Component<ContactFormMsg, AppMsg> for ContactForm {
    fn handle_key(&self, event: KeyEvent) -> Option<ContactFormMsg> {
        match event.code {
            KeyCode::Tab => Some(ContactFormMsg::NextField),
            KeyCode::BackTab => Some(ContactFormMsg::PrevField),
            KeyCode::Enter => Some(ContactFormMsg::Confirm),
            KeyCode::Esc => Some(ContactFormMsg::Cancel),
            _ => Some(ContactFormMsg::Input(event, self.focused)),
        }
    }

    fn draw(&self, f: &mut Frame, _: Rect, _: bool) {
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

    fn update(&mut self, message: ContactFormMsg) -> Option<AppMsg> {
        match message {
            ContactFormMsg::NextField => {
                self.focused += 1;
                if self.focused >= self.fields.len() {
                    self.focused = 0;
                }
                self.fields[self.focused].end();
                None
            }
            ContactFormMsg::PrevField => {
                if self.focused == 0 {
                    self.focused = self.fields.len() - 1;
                } else {
                    self.focused -= 1;
                }
                self.fields[self.focused].end();
                None
            }
            ContactFormMsg::Confirm => Some(AppMsg::SaveContact(self.to_contact())),
            ContactFormMsg::Cancel => {
                self.editing = None;
                Some(AppMsg::CancelForm)
            }
            ContactFormMsg::Input(event, index) => {
                if let Some(field) = self.fields.get_mut(index) {
                    field
                        .handle_key(event)
                        .map(|msg| AppMsg::ContactForm(ContactFormMsg::TextField(index, msg)))
                } else {
                    None
                }
            }
            ContactFormMsg::TextField(index, msg) => {
                if let Some(field) = self.fields.get_mut(index) {
                    field.update(msg)
                } else {
                    None
                }
            }
        }
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
