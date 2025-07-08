use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::*};

use crate::{components::Component, model::Contact};

pub enum DeleteMsg {}
pub enum DeleteOutput {}

#[derive(Debug, Default)]
pub struct DeleteConfirmation {
    contact: Contact,
}

impl DeleteConfirmation {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_contact(&mut self, contact: Contact) {
        self.contact = contact;
    }
    pub fn handle_key(&self, _event: KeyEvent) -> Option<DeleteMsg> {
        None
    }
    pub fn draw(&self, f: &mut Frame, area: Rect, _focused: bool) {
        f.render_widget(Clear, area);

        // Outer block for modal
        let outer_block = Block::default()
            .title(" Confirm Delete ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White).bg(Color::Black));
        let inner_area = outer_block.inner(area);
        f.render_widget(outer_block, area);

        // Layout: message | contact info (inset) | prompt
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(6),
                Constraint::Length(2),
            ])
            .split(inner_area);
        let msg_area = chunks[0];
        let contact_area = chunks[1];
        let prompt_area = chunks[2];
        let msg = Paragraph::new("\n  Are you sure you want to delete this contact?")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left);
        f.render_widget(msg, msg_area);

        // Inset contact area using horizontal margin
        let chunks = Layout::default()
            .horizontal_margin(4)
            .constraints([Constraint::Min(1)])
            .split(contact_area);
        let inset_area = chunks[0];

        // Padded/Aligned contact fields
        let contact = self.contact.clone();
        let contact_text = format!(
            "{:<9}{}\n{:<9}{}\n{:<9}{}\n{:<9}{}",
            "Name:",
            contact.name,
            "Company:",
            contact.company.as_deref().unwrap_or("N/A"),
            "Email:",
            contact.email.as_deref().unwrap_or("N/A"),
            "Phone:",
            contact.phone.as_deref().unwrap_or("N/A"),
        );

        let contact_paragraph = Paragraph::new(contact_text)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Left);

        f.render_widget(contact_paragraph, inset_area);

        let hint = Paragraph::new("\n[Y] Confirm  |  [N] Cancel")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(hint, prompt_area);
    }
    pub fn update<ParentMsg>(
        &mut self,
        _: DeleteMsg,
        _: impl Fn(DeleteOutput) -> ParentMsg,
    ) -> Option<ParentMsg> {
        None
    }
}

impl Component for DeleteConfirmation {
    type Msg = DeleteMsg;
    type Output = DeleteOutput;

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
    fn handle_key(&self, event: KeyEvent) -> Option<Self::Msg> {
        self.handle_key(event)
    }
}
