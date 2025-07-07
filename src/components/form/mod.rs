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

pub struct Form {
    fields: Vec<Input>,
    contact: Contact,
    focused: usize,
}

impl Form {
    pub fn new(contact: Contact) -> Self {
        let mut fields = vec![
            Input::new("Name", &contact.name),
            Input::new("Company", &contact.company.clone().unwrap_or_default()),
            Input::new("Email", &contact.email.clone().unwrap_or_default()),
            Input::new("Phone", &contact.phone.clone().unwrap_or_default()),
        ];
        fields[0].set_focused(true);
        Self {
            fields,
            contact,
            focused: 0,
        }
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
}
