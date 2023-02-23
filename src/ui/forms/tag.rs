use std::{mem::take, sync::mpsc};

use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    reexports::ahash::HashSet,
    view::{Scrollable, ViewWrapper},
    views::{
        Checkbox, Dialog, DialogFocus, LinearLayout, ListChild, ListView, ResizedView, ScrollView,
        TextArea,
    },
    wrap_impl, View,
};

use crate::{
    controller::ControllerSignal,
    data_handler::{make_mut, make_ref, tag::Tag, Identity, Represent},
    ui::utils::{dismiss, text_entry_full_width},
};

pub struct TagForm {
    view: Dialog,
    key: Identity,
    tag: Tag,
    controller_tx: mpsc::Sender<ControllerSignal>,
}

impl TagForm {
    pub fn new(
        key: Identity,
        tag: Tag,
        persona_list: &[Identity],
        controller_tx: &mpsc::Sender<ControllerSignal>,
    ) -> Self {
        Self {
            view: init_dialog(&tag, persona_list),
            key,
            tag,
            controller_tx: controller_tx.clone(),
        }
    }
}

impl TagForm {
    fn event_submit(&mut self) -> EventResult {
        let label = self.get_tag_label();
        let checked_persona = self.get_checked_persona();
        let mut tag = make_mut(&self.tag);
        tag.set_label(label);
        tag.set_persona_ids(checked_persona);
        self.controller_tx
            .send(ControllerSignal::CompleteEditTag {
                key: take(&mut self.key),
                tag: self.tag.clone(),
            })
            .unwrap();
        dismiss()
    }

    fn event_cancel(&self) -> EventResult {
        dismiss()
    }

    fn button_event(&mut self, index: usize) -> EventResult {
        match index {
            0 => self.event_submit(),
            1 => self.event_cancel(),
            _ => EventResult::Ignored,
        }
    }

    fn get_tag_label(&self) -> Identity {
        self.get_label_area().get_content().to_owned()
    }

    fn get_checked_persona(&self) -> Vec<Identity> {
        let mut result = vec![];
        for item in self.get_persona_list_view().children() {
            if let ListChild::Row(label, view) = item {
                if let Some(check) = view.downcast_ref::<Checkbox>() {
                    if check.is_checked() {
                        result.push(label.to_owned());
                    }
                }
            }
        }
        result
    }

    fn get_label_area(&self) -> &TextArea {
        self.view
            .get_content()
            .downcast_ref::<LinearLayout>()
            .unwrap()
            .get_child(0)
            .unwrap()
            .downcast_ref::<LinearLayout>()
            .unwrap()
            .get_child(2)
            .unwrap()
            .downcast_ref::<ResizedView<TextArea>>()
            .unwrap()
            .get_inner()
    }

    fn get_persona_list_view(&self) -> &ListView {
        self.view
            .get_content()
            .downcast_ref::<LinearLayout>()
            .unwrap()
            .get_child(1)
            .unwrap()
            .downcast_ref::<ScrollView<ListView>>()
            .unwrap()
            .get_inner()
    }
}

impl ViewWrapper for TagForm {
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

fn init_dialog(tag: &Tag, persona_list: &[Identity]) -> Dialog {
    Dialog::around(init_view(tag, persona_list))
        .title(format!("Редактируем метку {}", make_ref(tag).identity()))
        .button("Ok", |_| {})
        .button("Cancel", |_| {})
}

fn init_view(tag: &Tag, persona_list: &[Identity]) -> impl View {
    LinearLayout::vertical()
        .child(text_entry_full_width("Метка:", &make_ref(tag).identity()))
        .child(init_list(tag, persona_list))
}

fn init_list(tag: &Tag, persona_list: &[Identity]) -> impl View {
    let mut view = ListView::new();
    let tag = make_ref(tag);
    let need_to_check = tag.persona_ids().collect::<HashSet<_>>();
    for persona in persona_list {
        let check_box = Checkbox::new();
        view.add_child(
            &persona,
            if need_to_check.contains(persona) {
                check_box.checked()
            } else {
                check_box.unchecked()
            },
        );
    }
    view.scrollable()
}
