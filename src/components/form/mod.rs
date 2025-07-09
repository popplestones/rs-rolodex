use std::iter::repeat_n;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tracing::info;

use crate::{
    components::{
        component::opt,
        input::{Input, InputMode, InputMsg, InputOutput},
    },
    model::Contact,
};

#[derive(Debug, Clone)]
pub enum FormMsg {
    Input(InputMsg),
    Next,
    Previous,
    Submit,
    Cancel,
}

pub enum FormOutput {
    Submitted(Contact),
    Cancelled,
}

#[derive(Debug, Clone)]
pub enum FormField {
    Name,
    Company,
    Email,
    Phone,
}

const FIELD_ORDER: [FormField; 4] = [
    FormField::Name,
    FormField::Company,
    FormField::Email,
    FormField::Phone,
];

#[derive(Debug, Default)]
pub struct Form {
    fields: Vec<Input>,
    contact: Contact,
    focused: usize,
    editing_id: Option<i64>,
}

impl Form {
    pub fn new() -> Self {
        let contact = Contact::default();
        let contact_clone = contact.clone();

        Self {
            fields: vec![
                Input::new("Name", &contact.name, 10, InputMode::Inline, 30),
                Input::new(
                    "Company",
                    &contact.company.unwrap_or_default(),
                    10,
                    InputMode::Inline,
                    30,
                ),
                Input::new(
                    "Email",
                    &contact.email.unwrap_or_default(),
                    10,
                    InputMode::Inline,
                    30,
                ),
                Input::new(
                    "Phone",
                    &contact.phone.unwrap_or_default(),
                    10,
                    InputMode::Inline,
                    20,
                ),
            ],
            contact: contact_clone,
            focused: 0,
            editing_id: None,
        }
    }
    pub fn set_contact(&mut self, contact: Contact) {
        self.editing_id = Some(contact.id);
        self.contact = contact.clone();
        self.fields[0].value = contact.name;
        self.fields[1].value = contact.company.unwrap_or_default();
        self.fields[2].value = contact.email.unwrap_or_default();
        self.fields[3].value = contact.phone.unwrap_or_default();
        self.focused = 0;
        self.fields[0].set_focused(true);
    }

    pub fn update<ParentMsg>(
        &mut self,
        msg: FormMsg,
        map: impl Fn(FormOutput) -> ParentMsg,
    ) -> Option<ParentMsg> {
        info!("Form update: {:?}", msg);
        info!("Contact: {:?}", self.contact);
        match msg {
            FormMsg::Input(input_msg) => {
                info!("Form input: {:?}", input_msg);
                if let Some(field) = self.fields.get_mut(self.focused) {
                    let field_key = &FIELD_ORDER[self.focused];
                    info!("Field key: {:?}", field_key);

                    if let Some(InputOutput::Changed(val)) = field.update(input_msg, |out| out) {
                        info!("Field value: {:?}", val);
                        match field_key {
                            FormField::Name => self.contact.name = val,
                            FormField::Company => self.contact.company = opt(val),
                            FormField::Email => self.contact.email = opt(val),
                            FormField::Phone => self.contact.phone = opt(val),
                        }
                    }
                }
                None
            }
            FormMsg::Next => {
                if !self.fields.is_empty() {
                    self.fields[self.focused].set_focused(false);
                    self.focused = (self.focused + 1) % self.fields.len();
                    self.fields[self.focused].set_focused(true);
                }
                None
            }

            FormMsg::Previous => {
                if !self.fields.is_empty() {
                    self.fields[self.focused].set_focused(false);
                    self.focused = (self.focused + self.fields.len() - 1) % self.fields.len();
                    self.fields[self.focused].set_focused(true);
                }
                None
            }
            FormMsg::Submit => Some(map(FormOutput::Submitted(self.contact.clone()))),
            FormMsg::Cancel => Some(map(FormOutput::Cancelled)),
        }
    }

    pub fn draw(&self, f: &mut Frame, area: Rect, _focused: bool) {
        f.render_widget(Clear, area);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Add Contact ")
            .border_type(BorderType::Rounded)
            .padding(Padding {
                left: 2,
                right: 2,
                top: 1,
                bottom: 1,
            });

        f.render_widget(block.clone(), area);

        let inner = block.inner(area);

        let num_fields = self.fields.len();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                repeat_n(Constraint::Length(1), num_fields)
                    .chain([Constraint::Length(1), Constraint::Length(1)])
                    .collect::<Vec<_>>(),
            )
            .split(inner);

        for (i, field) in self.fields.iter().enumerate() {
            let is_focused = self.focused == i;
            field.draw(f, chunks[i], is_focused);
        }

        let button_area = chunks[num_fields + 1];
        let text = Span::styled(
            "[Enter] = Save / [Esc] = Cancel",
            Style::default().fg(Color::DarkGray),
        );
        let paragraph = Paragraph::new(text).alignment(Alignment::Center);
        f.render_widget(paragraph, button_area);
    }
    pub fn handle_key(&self, event: KeyEvent) -> Option<FormMsg> {
        match event.code {
            KeyCode::Tab => Some(FormMsg::Next),
            KeyCode::BackTab => Some(FormMsg::Previous),
            KeyCode::Enter => Some(FormMsg::Submit),
            KeyCode::Esc => Some(FormMsg::Cancel),
            _ => self.fields[self.focused]
                .handle_key(event)
                .map(FormMsg::Input),
        }
    }
}

impl crate::components::Component for Form {
    type Msg = FormMsg;
    type Output = FormOutput;

    fn draw(&self, f: &mut ratatui::Frame, area: Rect, focused: bool) {
        self.draw(f, area, focused);
    }
    fn handle_key(&self, event: KeyEvent) -> Option<Self::Msg> {
        self.handle_key(event)
    }

    fn update<ParentMsg>(
        &mut self,
        msg: Self::Msg,
        map: impl Fn(Self::Output) -> ParentMsg,
    ) -> Option<ParentMsg> {
        self.update(msg, map)
    }
}
