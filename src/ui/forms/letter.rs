use std::sync::mpsc;

use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    view::{Scrollable, ViewWrapper},
    views::{Dialog, DialogFocus, LinearLayout, ScrollView, TextArea, TextView},
    wrap_impl, View,
};

use crate::{
    controller::ControllerSignal,
    data_handler::letter::Letter,
    ui::{
        dialogs::{open_file::OpenFileDialog, SetData},
        utils::{dismiss, get_area_from, linear_layout_form},
    },
};

pub struct LetterForm {
    view: Dialog,
    name: uuid::Uuid,
    letter: Letter,
    controller_tx: mpsc::Sender<ControllerSignal>,
}

impl LetterForm {
    pub fn new(
        name: uuid::Uuid,
        letter: Letter,
        controller_tx: &mpsc::Sender<ControllerSignal>,
    ) -> Self {
        let mut form = Self {
            view: init_dialog(&letter),
            name,
            letter,
            controller_tx: controller_tx.clone(),
        };
        form.update_attachments();
        form
    }

    pub fn set_filename(&mut self, filename: &str) {
        let filename = filename.trim();
        if let Err(e) = self.letter.borrow_mut().add_attachment_from_path(filename) {
            self.controller_tx
                .send(ControllerSignal::Log(format!(
                    "Не удалось присоединить файл: {:?}\nОшибка: {}",
                    filename, e
                )))
                .unwrap();
        }
        self.update_attachments();
    }
}

impl LetterForm {
    fn get_area(&self, n: usize) -> &TextArea {
        get_area_from(&self.view, n)
    }

    fn get_attachment_view(&mut self) -> &mut TextView {
        self.view
            .get_content_mut()
            .downcast_mut::<ScrollView<LinearLayout>>()
            .unwrap()
            .get_inner_mut()
            .get_child_mut(2)
            .unwrap()
            .downcast_mut::<TextView>()
            .unwrap()
    }

    fn update_attachments(&mut self) {
        let info = self.letter.borrow().attachment_info();
        self.get_attachment_view().set_content(info);
    }

    fn save_letter(&mut self) {
        let topic = self.get_area(0).get_content();
        let text = self.get_area(1).get_content();
        let mut letter = self.letter.borrow_mut();
        letter.set_topic(topic);
        letter.set_text(text);
    }

    fn event_submit(&mut self) -> EventResult {
        self.save_letter();
        dismiss()
    }

    fn event_cancel(&mut self) -> EventResult {
        dismiss()
    }

    fn event_open_file(&mut self) -> EventResult {
        let parent_name = self.name;
        EventResult::with_cb_once(move |c| {
            c.add_layer(OpenFileDialog::<Self>::new(parent_name));
        })
    }

    fn event_send(&mut self) -> EventResult {
        self.save_letter();
        self.controller_tx
            .send(ControllerSignal::OpenLetterToSend(self.letter.clone()))
            .unwrap();
        dismiss()
    }

    fn button_event(&mut self, button: usize) -> EventResult {
        match button {
            0 => self.event_submit(),
            1 => self.event_cancel(),
            2 => self.event_open_file(),
            4 => self.event_send(),
            _ => EventResult::Ignored,
        }
    }
}

impl SetData for LetterForm {
    fn set_data(&mut self, data: String) {
        self.set_filename(&data)
    }
}

impl ViewWrapper for LetterForm {
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

fn init_dialog(letter: &Letter) -> Dialog {
    Dialog::around(init_form(letter))
        .title("Редактируем письмо")
        .button("OK", |_| {})
        .button("Cancel", |_| {})
        .button("Add file", |_| {})
        .button("Remove file", |_| {})
        .button("Send", |_| {})
}

fn init_form(letter: &Letter) -> impl View {
    let letter = letter.as_ref().borrow();
    linear_layout_form(vec![
        ("Тема:", letter.get_topic()),
        ("Сообщение:", letter.get_text()),
    ])
    .child(TextView::new(letter.attachment_info()))
    .scrollable()
}
