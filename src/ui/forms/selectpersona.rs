use std::sync::mpsc;

use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    view::ViewWrapper,
    views::{Dialog, DialogFocus, LinearLayout, SelectView, TextView},
    wrap_impl, View,
};

use crate::{
    controller::ControllerSignal,
    data_handler::{make_ref, persona::Persona, Represent},
    ui::utils::{dismiss, get_view_from_dialog, no_selection_info},
};

pub struct SelectPersonaForm {
    view: Dialog,
    controller_tx: mpsc::Sender<ControllerSignal>,
}

impl SelectPersonaForm {
    pub fn new(persona: Vec<Persona>, ui_tx: &mpsc::Sender<ControllerSignal>) -> Self {
        Self {
            view: init_dialog(persona),
            controller_tx: ui_tx.clone(),
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
            self.controller_tx
                .send(ControllerSignal::EditPersona(selected_persona))
                .unwrap();
            dismiss()
        } else {
            no_selection_info(&self.controller_tx, "редактирования", "персону");
            EventResult::consumed()
        }
    }

    fn event_remove(&self) -> EventResult {
        if let Some(selected_persona) = self.get_selected_persona() {
            self.controller_tx
                .send(ControllerSignal::RemovePersonaAlert(selected_persona))
                .unwrap();
            dismiss()
        } else {
            no_selection_info(&self.controller_tx, "удаления", "персону");
            EventResult::consumed()
        }
    }

    fn event_close(&self) -> EventResult {
        dismiss()
    }

    fn get_selected_persona(&self) -> Option<Persona> {
        self.get_select_view()
            .selection()
            .map(|p| p.as_ref().clone())
    }

    fn get_select_view(&self) -> &SelectView<Persona> {
        const SELECT_VIEW_INDEX: usize = 1;
        get_view_from_dialog(&self.view, SELECT_VIEW_INDEX)
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

fn init_view(persona_list: Vec<Persona>) -> impl View {
    let mut select = SelectView::new().popup();
    select.set_autojump(true);
    for persona in persona_list {
        let label = make_ref(&persona).identity();
        select.add_item(label, persona);
    }
    LinearLayout::vertical()
        .child(TextView::new("Выберите персону:"))
        .child(select)
}
