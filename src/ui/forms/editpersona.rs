use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    view::ViewWrapper,
    views::{Dialog, DialogFocus, TextArea},
    wrap_impl, View,
};

use crate::{
    data_handler::{persona::Persona, Represent},
    ui::utils::{dismiss, get_area_from, linear_layout_form},
};

pub struct EditPersonaForm {
    view: Dialog,
    persona: Persona,
}

impl EditPersonaForm {
    const FAMILY_INDEX: usize = 0;
    const NAME_INDEX: usize = 1;
    const SURNAME_INDEX: usize = 2;
    const EMAIL_INDEX: usize = 3;

    pub fn new(persona: Persona) -> Self {
        Self {
            view: init_dialog(&persona),
            persona,
        }
    }

    fn button_event(&self, n: usize) -> EventResult {
        match n {
            0 => self.event_ok(),
            1 => self.event_cancel(),
            _ => EventResult::Ignored,
        }
    }

    fn event_ok(&self) -> EventResult {
        let family = self.get_area(Self::FAMILY_INDEX).get_content().to_string();
        let name = self.get_area(Self::NAME_INDEX).get_content().to_string();
        let surname = self.get_area(Self::SURNAME_INDEX).get_content().to_string();
        let email = self.get_area(Self::EMAIL_INDEX).get_content().to_string();
        let mut persona = self.persona.borrow_mut();
        persona.set_family(family);
        persona.set_name(name);
        persona.set_surname(surname);
        persona.set_email(email);
        drop(persona);
        dismiss()
    }

    fn event_cancel(&self) -> EventResult {
        dismiss()
    }

    fn get_area(&self, entry_index: usize) -> &TextArea {
        get_area_from(&self.view, entry_index)
    }
}

impl ViewWrapper for EditPersonaForm {
    wrap_impl!(self.view: Dialog);

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Mouse {
                offset: _,
                position: _,
                event: MouseEvent::Press(btn),
            } => {
                if btn == MouseButton::Left {
                    self.with_view_mut(|v| v.on_event(event))
                        .unwrap_or(EventResult::Ignored);
                    match self.view.focus() {
                        DialogFocus::Button(n) => self.button_event(n),
                        _ => EventResult::Ignored,
                    }
                } else {
                    EventResult::Ignored
                }
            }
            Event::Key(Key::Enter) => match self.view.focus() {
                DialogFocus::Button(n) => self.button_event(n),
                _ => self
                    .with_view_mut(|v| v.on_event(event))
                    .unwrap_or(EventResult::Ignored),
            },
            Event::Key(Key::Esc) => self.event_cancel(),
            _ => self
                .with_view_mut(|v| v.on_event(event))
                .unwrap_or(EventResult::Ignored),
        }
    }
}

fn init_dialog(persona: &Persona) -> Dialog {
    Dialog::around(init_view(persona))
        .title(format!(
            "Редактируем {}",
            persona.as_ref().borrow().identity()
        ))
        .button("Ok", |_| {})
        .button("Cancel", |_| {})
}

fn init_view(persona: &Persona) -> impl View {
    let persona = persona.as_ref().borrow();
    linear_layout_form(vec![
        (" Фамилия:", persona.get_family()),
        ("     Имя:", persona.get_name()),
        ("Отчество:", persona.get_surname()),
        ("  E-mail:", persona.get_email()),
    ])
}
