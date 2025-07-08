use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect};

use crate::{
    components::{
        component::opt,
        input::{Input, InputMsg, InputOutput},
    },
    model::Contact,
};

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

#[derive(Clone)]
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
                Input::new("Name", &contact.name, 10),
                Input::new("Company", &contact.company.unwrap_or_default(), 10),
                Input::new("Email", &contact.email.unwrap_or_default(), 10),
                Input::new("Phone", &contact.phone.unwrap_or_default(), 10),
            ],
            contact: contact_clone,
            focused: 0,
            editing_id: None,
        }
    }
    pub fn set_contact(&mut self, contact: Contact) {
        self.editing_id = Some(contact.id);
        self.fields[0].value = contact.name;
        self.fields[1].value = contact.company.unwrap_or_default();
        self.fields[2].value = contact.email.unwrap_or_default();
        self.fields[3].value = contact.phone.unwrap_or_default();
        self.focused = 0;
    }

    pub fn update<ParentMsg>(
        &mut self,
        msg: FormMsg,
        map: impl Fn(FormOutput) -> ParentMsg,
    ) -> Option<ParentMsg> {
        match msg {
            FormMsg::Input(input_msg) => {
                if let Some(field) = self.fields.get_mut(self.focused) {
                    let field_key = &FIELD_ORDER[self.focused];

                    if let Some(InputOutput::Changed(val)) = field.update(input_msg, |out| out) {
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

    pub fn draw(&self, _f: &mut Frame, _area: Rect, _focused: bool) {}
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
