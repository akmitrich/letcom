use std::sync::mpsc::{self, Sender};

use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    view::ViewWrapper,
    views::{Dialog, DialogFocus, LinearLayout, SelectView, TextView},
    wrap_impl, View,
};

use crate::{
    controller::ControllerSignal,
    data_handler::{make_ref, tag::Tag},
    ui::utils::{dismiss, get_view_from_dialog, no_selection_info},
};

pub struct SelectTagForm {
    view: Dialog,
    controller_tx: Sender<ControllerSignal>,
}

impl SelectTagForm {
    pub fn new(tags: Vec<Tag>, controller_tx: &mpsc::Sender<ControllerSignal>) -> Self {
        Self {
            view: init_view(tags),
            controller_tx: controller_tx.clone(),
        }
    }
}

impl SelectTagForm {
    fn button_event(&mut self, index: usize) -> EventResult {
        match index {
            0 => dismiss(),
            1 => self.event_edit(),
            2 => self.event_remove(),
            _ => EventResult::consumed(),
        }
    }

    fn event_edit(&mut self) -> EventResult {
        if let Some(selected) = self.get_selected_tag() {
            self.controller_tx
                .send(ControllerSignal::EditTag(selected))
                .unwrap();
            dismiss()
        } else {
            no_selection_info(&self.controller_tx, "редактирования", "метку");
            EventResult::consumed()
        }
    }

    fn event_remove(&mut self) -> EventResult {
        if let Some(selected) = self.get_selected_tag() {
            self.controller_tx
                .send(ControllerSignal::RemoveTagAlert(selected))
                .unwrap();
            dismiss()
        } else {
            no_selection_info(&self.controller_tx, "удаления", "метку");
            EventResult::consumed()
        }
    }

    fn event_cancel(&mut self) -> EventResult {
        dismiss()
    }
}

impl SelectTagForm {
    fn get_selected_tag(&self) -> Option<Tag> {
        self.get_select_view()
            .selection()
            .map(|t| t.as_ref().clone())
    }

    fn get_select_view(&self) -> &SelectView<Tag> {
        const SELECT_VIEW_INDEX: usize = 1;
        get_view_from_dialog(&self.view, SELECT_VIEW_INDEX)
    }
}

impl ViewWrapper for SelectTagForm {
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

fn init_view(tags: Vec<Tag>) -> Dialog {
    Dialog::around(init_dialog(tags))
        .button("Close", |_| {})
        .button("Edit", |_| {})
        .button("Remove", |_| {})
}

fn init_dialog(tags: Vec<Tag>) -> impl View {
    let mut select = SelectView::<Tag>::new().popup();
    for tag in tags {
        let label = make_ref(&tag).label();
        select.add_item(label, tag);
    }
    LinearLayout::vertical()
        .child(TextView::new("Выберите метку:"))
        .child(select)
}
