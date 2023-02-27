use std::sync::mpsc::{self, Sender};

use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    view::ViewWrapper,
    views::{Dialog, DialogFocus},
    wrap_impl, View,
};

use crate::{
    controller::ControllerSignal,
    data_handler::{make_mut, make_ref, persona::Persona, Identity, Represent},
    ui::utils::{dismiss, form_view, get_text_from_form_entry},
};

pub struct EditPersonaForm {
    view: Dialog,
    key: Identity,
    persona: Persona,
    controller_tx: mpsc::Sender<ControllerSignal>,
}

impl EditPersonaForm {
    const FAMILY_INDEX: usize = 0;
    const NAME_INDEX: usize = 1;
    const SURNAME_INDEX: usize = 2;
    const EMAIL_INDEX: usize = 3;

    pub fn new(key: Identity, persona: Persona, controller_tx: &Sender<ControllerSignal>) -> Self {
        Self {
            view: init_dialog(&persona),
            key,
            persona,
            controller_tx: controller_tx.clone(),
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
        let family = get_text_from_form_entry(&self.view, Self::FAMILY_INDEX);
        let name = get_text_from_form_entry(&self.view, Self::NAME_INDEX);
        let surname = get_text_from_form_entry(&self.view, Self::SURNAME_INDEX);
        let email = get_text_from_form_entry(&self.view, Self::EMAIL_INDEX);
        let mut persona = make_mut(&self.persona);
        persona.set_family(family);
        persona.set_name(name);
        persona.set_surname(surname);
        persona.set_email(email);
        drop(persona);
        self.controller_tx
            .send(ControllerSignal::CompleteEditPersona {
                key: self.key.clone(),
                persona: self.persona.clone(),
            })
            .unwrap();
        dismiss()
    }

    fn event_cancel(&self) -> EventResult {
        dismiss()
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
        .title(format!("Редактируем {}", make_ref(persona).identity()))
        .button("Ok", |_| {})
        .button("Cancel", |_| {})
}

fn init_view(persona: &Persona) -> impl View {
    let persona = make_ref(persona);
    form_view(vec![
        (" Фамилия:", persona.get_family()),
        ("     Имя:", persona.get_name()),
        ("Отчество:", persona.get_surname()),
        ("  E-mail:", persona.get_email()),
    ])
}
