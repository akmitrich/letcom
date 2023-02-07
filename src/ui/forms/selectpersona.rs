use std::sync::mpsc;

use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    view::ViewWrapper,
    views::{Dialog, DialogFocus, SelectView},
    wrap_impl, View,
};

use crate::{
    controller::ControllerSignal,
    data_handler::{persona::Persona, Represent},
    ui::utils::dismiss,
};

pub struct SelectPersonaForm {
    view: Dialog,
    ui_tx: mpsc::Sender<ControllerSignal>,
}

impl SelectPersonaForm {
    pub fn new(persona: Vec<Persona>, ui_tx: &mpsc::Sender<ControllerSignal>) -> Self {
        Self {
            view: init_dialog(persona),
            ui_tx: ui_tx.clone(),
        }
    }

    fn button_event(&self, n: usize) -> EventResult {
        match n {
            0 => dismiss(),
            1 => self.event_edit(),
            2 => self.event_remove(),
            _ => EventResult::Ignored,
        }
    }

    fn event_edit(&self) -> EventResult {
        if let Some(selected_persona) = self.get_selected_persona() {
            self.ui_tx
                .send(ControllerSignal::EditPersona(selected_persona))
                .unwrap();
            dismiss()
        } else {
            self.no_selection_info("редактирования");
            EventResult::consumed()
        }
    }

    fn event_remove(&self) -> EventResult {
        if let Some(selected_persona) = self.get_selected_persona() {
            self.ui_tx
                .send(ControllerSignal::RemovePersonaAlert(selected_persona))
                .unwrap();
            dismiss()
        } else {
            self.no_selection_info("удаления");
            EventResult::consumed()
        }
    }

    fn event_close(&self) -> EventResult {
        dismiss()
    }

    fn no_selection_info(&self, action: &str) {
        self.ui_tx
            .send(ControllerSignal::Log(format!(
                "Для {} надо выбрать персону.",
                action
            )))
            .unwrap();
    }

    fn get_selected_persona(&self) -> Option<Persona> {
        let view = self
            .view
            .get_content()
            .downcast_ref::<SelectView<Persona>>()
            .unwrap();
        view.selection().map(|p| p.as_ref().clone())
    }
}

impl ViewWrapper for SelectPersonaForm {
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
            Event::Key(Key::Esc) => self.event_close(),
            _ => self
                .with_view_mut(|v| v.on_event(event))
                .unwrap_or(EventResult::Ignored),
        }
    }
}

fn init_dialog(persona: Vec<Persona>) -> Dialog {
    Dialog::around(init_view(persona))
        .button("Close", |_| {})
        .button("Edit", |_| {})
        .button("Remove", |_| {})
}

fn init_view(persona: Vec<Persona>) -> impl View {
    let mut select = SelectView::new().popup();
    select.set_autojump(true);
    for p in persona {
        let label = p.read().unwrap().identity();
        select.add_item(label, p);
    }
    select
}
