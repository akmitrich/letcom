use std::sync::mpsc;

use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    view::{Scrollable, ViewWrapper},
    views::{
        Checkbox, Dialog, DialogFocus, LinearLayout, ListChild, ListView, Panel, ResizedView,
        ScrollView, TextArea,
    },
    wrap_impl, View,
};

use crate::{
    controller::ControllerSignal,
    data_handler::{letter::Letter, make_mut, make_ref, persona::Persona, Represent},
    ui::utils::dismiss,
};

use super::letter::letter_view;

pub struct SendLetterForm {
    view: Dialog,
    people: Vec<Persona>,
    letter: Letter,
    controller_tx: mpsc::Sender<ControllerSignal>,
}

impl SendLetterForm {
    pub fn new(
        letter: Letter,
        people: Vec<Persona>,
        controller_tx: &mpsc::Sender<ControllerSignal>,
    ) -> Self {
        let controller_tx = controller_tx.clone();
        Self {
            view: init_dialog(&letter, &people),
            people,
            letter,
            controller_tx,
        }
    }

    fn button_event(&mut self, n: usize) -> EventResult {
        match n {
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
        let topic = self.get_letter_area(0).get_content();
        let text = self.get_letter_area(1).get_content();
        let mut letter = make_mut(&self.letter);
        letter.set_topic(topic);
        letter.set_text(text);
    }

    fn get_chosen_addresses(&self) -> Vec<String> {
        let mut chosen = vec![];
        let list_view = self.get_list_view();
        for (index, list_entry) in list_view.children().iter().enumerate() {
            if let ListChild::Row(_, check) = list_entry {
                if let Some(check) = check.downcast_ref::<Checkbox>() {
                    if check.is_checked() {
                        let email_string = make_ref(self.people.get(index).unwrap())
                            .get_email()
                            .to_string();
                        chosen.push(email_string);
                    }
                }
            }
        }
        chosen
    }

    fn get_list_view(&self) -> &ListView {
        self.get_panel(0)
            .downcast_ref::<Panel<ScrollView<ListView>>>()
            .unwrap()
            .get_inner()
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

    fn get_panel(&self, panel_index: usize) -> &dyn View {
        self.view
            .get_content()
            .downcast_ref::<LinearLayout>()
            .unwrap()
            .get_child(panel_index)
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

fn init_dialog(letter: &Letter, people: &[Persona]) -> Dialog {
    Dialog::around(init_view(letter, people))
        .title("Отправка письма")
        .button("SEND", |_| {})
        .button("Cancel", |_| {})
}

fn init_view(letter: &Letter, people: &[Persona]) -> impl View {
    LinearLayout::horizontal()
        .child(Panel::new(init_address_panel(people)))
        .child(Panel::new(init_letter_panel(letter)))
}

fn init_address_panel(people: &[Persona]) -> impl View {
    let mut select = ListView::new();
    for persona in people {
        let checkbox = Checkbox::new().unchecked();
        let label = make_ref(persona).identity();
        select.add_child(&label, checkbox);
    }
    select.scrollable()
}

fn init_letter_panel(letter: &Letter) -> impl View {
    letter_view(letter)
}
