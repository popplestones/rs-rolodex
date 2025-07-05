pub mod message;
use crossterm::event::{KeyCode, KeyEvent};
use message::ContactListMessage;
use ratatui::{prelude::*, widgets::*};

use crate::{model::Contact, ui::components::Component};

#[derive(Debug, Default)]
pub struct ContactList {
    pub filtered_contacts: Vec<Contact>,
    pub selected_index: usize,
}

impl ContactList {
    pub fn new(contacts: &[Contact]) -> Self {
        Self {
            filtered_contacts: contacts.to_vec(),
            selected_index: 0,
        }
    }
}
use crate::ui::components::app::message::AppMessage;
impl Component<ContactListMessage, AppMessage> for ContactList {
    fn draw(&self, f: &mut Frame, rect: Rect, _is_focused: bool) {
        let items: Vec<ListItem> = self
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
        list_state.select(Some(self.selected_index));

        f.render_stateful_widget(list, rect, &mut list_state);

        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .thumb_style(Style::default().bg(Color::Gray))
            .track_style(Style::default().bg(Color::DarkGray));
        let mut scrollbar_state =
            ScrollbarState::new(self.filtered_contacts.len()).position(self.selected_index);

        f.render_stateful_widget(
            scrollbar,
            rect.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut scrollbar_state,
        );
    }

    fn handle_key(&self, event: KeyEvent) -> Option<ContactListMessage> {
        match event.code {
            KeyCode::Up => Some(ContactListMessage::Previous),
            KeyCode::Down => Some(ContactListMessage::Next),
            KeyCode::Home => Some(ContactListMessage::First),
            KeyCode::End => Some(ContactListMessage::Last),
            KeyCode::PageUp => Some(ContactListMessage::PgUp),
            KeyCode::PageDown => Some(ContactListMessage::PgDown),
            _ => None,
        }

        // match event.code {
        //     KeyCode::Up => {
        //         if self.selected_index > 0 {
        //             self.selected_index -= 1;
        //         }
        //     }
        //     KeyCode::Down => {
        //         if self.selected_index < self.filtered_contacts.len() - 1 {
        //             self.selected_index += 1;
        //         }
        //     }
        //     KeyCode::Home => {
        //         self.selected_index = 0;
        //     }
        //     KeyCode::End => {
        //         self.selected_index = self.filtered_contacts.len() - 1;
        //     }
        //     _ => {}
        // }
    }

    fn update(&mut self, message: ContactListMessage) -> Option<AppMessage> {
        match message {
            ContactListMessage::First => {
                self.selected_index = 0;
            }
            ContactListMessage::Last => {
                self.selected_index = self.filtered_contacts.len() - 1;
            }
            ContactListMessage::Previous => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            ContactListMessage::Next => {
                if self.selected_index < self.filtered_contacts.len() - 1 {
                    self.selected_index += 1;
                }
            }
            ContactListMessage::PgUp => {
                if self.selected_index < 10 {
                    self.selected_index = 0;
                } else {
                    self.selected_index -= 10;
                }
            }
            ContactListMessage::PgDown => {
                if self.selected_index > self.filtered_contacts.len() - 10 {
                    self.selected_index = self.filtered_contacts.len() - 1;
                } else {
                    self.selected_index += 10;
                }
            }
        };
        None
    }
}
