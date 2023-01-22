use std::{fs, sync::mpsc};

use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    view::{Scrollable, ViewWrapper},
    views::{Dialog, DialogFocus, LinearLayout, ResizedView, ScrollView, TextArea, TextView},
    wrap_impl, View,
};
use lettre::message::{header::ContentType, Attachment};

use crate::{
    controller::ControllerSignal,
    ui::{
        dialogs::{open_file::OpenFileDialog, SetData},
        utils::{dismiss, text_entry_full_width},
    },
};

pub struct LetterForm {
    view: Dialog,
    name: uuid::Uuid,
    controller_tx: mpsc::Sender<ControllerSignal>,
}

impl LetterForm {
    pub fn new(name: uuid::Uuid, controller_tx: &mpsc::Sender<ControllerSignal>) -> Self {
        Self {
            view: init_dialog(),
            name,
            controller_tx: controller_tx.clone(),
        }
    }

    pub fn set_filename(&mut self, filename: &str) {
        let filebody = fs::read(filename).unwrap();
        let content_type = ContentType::parse("application/txt").unwrap();
        let attachment = Attachment::new(filename.to_owned()).body(filebody, content_type);
        eprintln!(
            "Loaded: '{}'",
            String::from_utf8_lossy(&attachment.formatted())
        );
    }
}

impl LetterForm {
    const INTRO: &str = "email_form_intro";
    const TEXT: &str = "email_form_text";
    const OUTRO: &str = "email_form_outro";

    fn get_area(&mut self, index: usize) -> &mut TextArea {
        let scroll = self
            .view
            .get_content_mut()
            .downcast_mut::<ScrollView<LinearLayout>>()
            .unwrap();
        let entry = scroll
            .get_inner_mut()
            .get_child_mut(index)
            .unwrap()
            .downcast_mut::<LinearLayout>()
            .unwrap();
        let widget = entry
            .get_child_mut(2)
            .unwrap()
            .downcast_mut::<ResizedView<TextArea>>()
            .unwrap()
            .get_inner_mut();
        widget
    }

    fn event_submit(&mut self) -> EventResult {
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
        self.controller_tx
            .send(ControllerSignal::SendLetter)
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
    fn set_data(&mut self, data: &[String]) {
        for x in data {
            self.set_filename(x);
        }
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

fn init_dialog() -> Dialog {
    Dialog::around(init_form().scrollable())
        .title("Редактируем письмо")
        .button("OK", |_| {})
        .button("Cancel", |_| {})
        .button("Add file", |_| {})
        .button("Remove file", |_| {})
        .button("Send", |_| {})
}

fn init_form() -> impl View {
    LinearLayout::vertical()
        .child(text_entry_full_width("     Тема:", LetterForm::INTRO))
        .child(text_entry_full_width("Сообщение:", LetterForm::TEXT))
        .child(text_entry_full_width("  Подпись:", LetterForm::OUTRO))
        .child(TextView::new("Вложения"))
}
