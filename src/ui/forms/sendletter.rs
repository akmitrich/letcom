use std::sync::mpsc;

use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    view::{Scrollable, ViewWrapper},
    views::{
        Checkbox, Dialog, DialogFocus, LinearLayout, ListChild, ListView, Panel, ResizedView,
        ScrollView, TextArea, TextView,
    },
    wrap_impl, View,
};

use crate::{
    controller::{letter::Letter, ControllerSignal},
    ui::utils::{dismiss, text_entry_full_width},
};

pub struct SendLetterForm {
    view: Dialog,
    letter: Letter,
    controller_tx: mpsc::Sender<ControllerSignal>,
}

impl SendLetterForm {
    pub fn new(
        letter: Letter,
        addresses: Vec<String>,
        controller_tx: &mpsc::Sender<ControllerSignal>,
    ) -> Self {
        let controller_tx = controller_tx.clone();
        Self {
            view: init_dialog(&letter, &addresses),
            letter,
            controller_tx,
        }
    }

    fn button_event(&mut self, _n: usize) -> EventResult {
        match _n {
            0 => self.do_send(),
            1 => self.event_cancel(),
            _ => EventResult::Ignored,
        }
    }

    fn do_send(&mut self) -> EventResult {
        self.update_letter();
        self.controller_tx
            .send(ControllerSignal::SendEmail {
                letter: self.letter.clone(),
                to: self.get_chosen_addresses(),
            })
            .unwrap();
        dismiss()
    }

    fn event_cancel(&self) -> EventResult {
        dismiss()
    }

    fn update_letter(&mut self) {
        let topic = self.get_letter_area(0).get_content().to_string();
        let text = self.get_letter_area(1).get_content().to_string();
        self.letter.write().unwrap().topic = topic;
        self.letter.write().unwrap().text = text;
    }

    fn get_chosen_addresses(&self) -> Vec<String> {
        let mut chosen = vec![];
        let list_view = self.get_list_view();
        for c in list_view.children() {
            if let ListChild::Row(label, check) = c {
                if let Some(check) = check.downcast_ref::<Checkbox>() {
                    if check.is_checked() {
                        chosen.push(label.to_owned());
                    }
                }
            }
        }
        chosen
    }

    fn get_list_view(&self) -> &ListView {
        self.get_panel(0)
            .downcast_ref::<Panel<ListView>>()
            .unwrap()
            .get_inner()
    }

    fn get_letter_area(&self, n: usize) -> &TextArea {
        self.get_panel(1)
            .downcast_ref::<Panel<ScrollView<LinearLayout>>>()
            .unwrap()
            .get_inner()
            .get_inner()
            .get_child(n)
            .unwrap()
            .downcast_ref::<LinearLayout>()
            .unwrap()
            .get_child(2)
            .unwrap()
            .downcast_ref::<ResizedView<TextArea>>()
            .unwrap()
            .get_inner()
    }

    fn get_panel(&self, n: usize) -> &dyn View {
        self.view
            .get_content()
            .downcast_ref::<LinearLayout>()
            .unwrap()
            .get_child(n)
            .unwrap()
    }
}

impl ViewWrapper for SendLetterForm {
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

fn init_dialog(letter: &Letter, addresses: &[String]) -> Dialog {
    Dialog::around(init_view(letter, addresses))
        .title("Отправка письма")
        .button("SEND", |_| {})
        .button("Cancel", |_| {})
}

fn init_view(letter: &Letter, addresses: &[String]) -> impl View {
    LinearLayout::horizontal()
        .child(Panel::new(init_address_panel(addresses)))
        .child(Panel::new(init_letter_panel(letter)))
}

fn init_address_panel(addresses: &[String]) -> impl View {
    let mut select = ListView::new();
    for a in addresses {
        let checkbox = Checkbox::new().unchecked();
        select.add_child(a, checkbox);
    }
    select
}

fn init_letter_panel(letter: &Letter) -> impl View {
    LinearLayout::vertical()
        .child(text_entry_full_width(
            " Тема:",
            &letter.read().unwrap().topic,
        ))
        .child(text_entry_full_width(
            "Текст:",
            &letter.read().unwrap().text,
        ))
        .child(TextView::new(letter.read().unwrap().attachment_info()))
        .scrollable()
}
