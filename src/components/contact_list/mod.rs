use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

use crate::{components::Component, model::Contact};

pub enum ContactListMsg {
    Activate,
    Next,
    Prev,
    First,
    Last,
    PgUp,
    PgDown,
}

pub enum ContactListOutput {
    ContactActivated(Contact),
}

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
    pub fn get_selected_contact(&self) -> Option<Contact> {
        if self.selected_index < self.filtered_contacts.len() {
            Some(self.filtered_contacts[self.selected_index].clone())
        } else {
            None
        }
    }
    pub fn update<ParentMsg>(
        &mut self,
        msg: ContactListMsg,
        map: impl Fn(ContactListOutput) -> ParentMsg,
    ) -> Option<ParentMsg> {
        match msg {
            ContactListMsg::Next => {
                if self.selected_index < self.filtered_contacts.len().saturating_sub(1) {
                    self.selected_index += 1;
                }
                None
            }
            ContactListMsg::Prev => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
                None
            }
            ContactListMsg::First => {
                self.selected_index = 0;
                None
            }
            ContactListMsg::Last => {
                self.selected_index = self.filtered_contacts.len().saturating_sub(1);
                None
            }
            ContactListMsg::PgUp => {
                if self.selected_index < 10 {
                    self.selected_index = 0;
                } else {
                    self.selected_index -= 10;
                }
                None
            }
            ContactListMsg::PgDown => {
                if self.selected_index > self.filtered_contacts.len().saturating_sub(10) {
                    self.selected_index = self.filtered_contacts.len().saturating_sub(1);
                } else {
                    self.selected_index += 10;
                }
                None
            }
            ContactListMsg::Activate => self
                .get_selected_contact()
                .map(|contact| map(ContactListOutput::ContactActivated(contact))),
        }
    }
    fn draw(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let block = Block::default().borders(Borders::ALL).title("Contacts");
        f.render_widget(block, area);

        let inner = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)])
            .margin(1)
            .split(area);

        // Render column headings
        let header = Paragraph::new(format!(
            "   {:<20} {:<20} {:<35} {:<15}",
            "Name", "Company", "Email", "Phone"
        ))
        .style(Style::default().add_modifier(Modifier::UNDERLINED | Modifier::BOLD));

        f.render_widget(header, inner[0]);

        let items: Vec<ListItem> = self
            .filtered_contacts
            .iter()
            .map(|c| {
                ListItem::new(format!(
                    "{:<20} {:<20} {:<35} {:<15}",
                    c.name,
                    c.company.as_deref().unwrap_or("-"),
                    c.email.as_deref().unwrap_or("-"),
                    c.phone.as_deref().unwrap_or("-")
                ))
            })
            .collect();

        let list = List::new(items)
            .highlight_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        let mut state = ListState::default();
        state.select(Some(self.selected_index));

        f.render_stateful_widget(list, inner[1], &mut state);

        // Draw the scrollbar on the right
        let mut scroll_state = ScrollbarState::new(self.filtered_contacts.len())
            .position(self.selected_index)
            .content_length(self.filtered_contacts.len());

        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .thumb_style(Style::default().bg(Color::Cyan));

        f.render_stateful_widget(scrollbar, inner[1], &mut scroll_state);
    }

    fn handle_key(&self, key: KeyEvent) -> Option<ContactListMsg> {
        match key.code {
            KeyCode::Down => Some(ContactListMsg::Next),
            KeyCode::Up => Some(ContactListMsg::Prev),
            KeyCode::Home => Some(ContactListMsg::First),
            KeyCode::End => Some(ContactListMsg::Last),
            KeyCode::Enter => Some(ContactListMsg::Activate),
            _ => None,
        }
    }
}

impl Component for ContactList {
    type Msg = ContactListMsg;
    type Output = ContactListOutput;

    fn update<ParentMsg>(
        &mut self,
        msg: Self::Msg,
        map: impl Fn(Self::Output) -> ParentMsg,
    ) -> Option<ParentMsg> {
        self.update(msg, map)
    }

    fn draw(&self, f: &mut Frame, area: Rect, focused: bool) {
        self.draw(f, area, focused)
    }

    fn handle_key(&self, key: KeyEvent) -> Option<Self::Msg> {
        self.handle_key(key)
    }
}
