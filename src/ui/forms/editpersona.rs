use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    view::{Scrollable, ViewWrapper},
    views::{Dialog, DialogFocus, LinearLayout, TextArea},
    wrap_impl, View,
};

use crate::{
    data_handler::persona::Persona,
    ui::utils::{dismiss, get_area_from, text_entry_full_width},
};

pub struct EditPersonaForm {
    view: Dialog,
    persona: Persona,
}

impl EditPersonaForm {
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
        let family = self.get_area(0).get_content().to_string();
        let name = self.get_area(1).get_content().to_string();
        let surname = self.get_area(2).get_content().to_string();
        let email = self.get_area(3).get_content().to_string();
        let mut persona = self.persona.write().unwrap();
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

    fn get_area(&self, n: usize) -> &TextArea {
        get_area_from(&self.view, n)
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
            persona.read().unwrap().identity()
        ))
        .button("Ok", |_| {})
        .button("Cancel", |_| {})
}

fn init_view(persona: &Persona) -> impl View {
    LinearLayout::vertical()
        .child(text_entry_full_width(
            "Фамилия:",
            persona.read().unwrap().get_family(),
        ))
        .child(text_entry_full_width(
            "Имя:",
            persona.read().unwrap().get_name(),
        ))
        .child(text_entry_full_width(
            "Отчество:",
            persona.read().unwrap().get_surname(),
        ))
        .child(text_entry_full_width(
            "E-mail:",
            persona.read().unwrap().get_email(),
        ))
        .scrollable()
}
