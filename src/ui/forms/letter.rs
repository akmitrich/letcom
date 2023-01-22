use std::{fs, sync::mpsc};

use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    view::{Scrollable, ViewWrapper},
    views::{Dialog, DialogFocus, LinearLayout, ResizedView, ScrollView, TextArea, TextView},
    wrap_impl, View,
};
use lettre::message::{header::ContentType, Attachment};

use crate::{
    controller::{letter::Letter, ControllerSignal},
    ui::{
        dialogs::{open_file::OpenFileDialog, SetData},
        utils::{dismiss, text_entry_full_width},
        UiEvent,
    },
};

pub struct LetterForm {
    view: Dialog,
    name: uuid::Uuid,
    letter: Letter,
    controller_tx: mpsc::Sender<ControllerSignal>,
    ui_tx: mpsc::Sender<UiEvent>,
}

impl LetterForm {
    pub fn new(
        name: uuid::Uuid,
        letter: Letter,
        controller_tx: &mpsc::Sender<ControllerSignal>,
        ui_tx: &mpsc::Sender<UiEvent>,
    ) -> Self {
        let mut form = Self {
            view: init_dialog(&letter),
            name,
            letter,
            controller_tx: controller_tx.clone(),
            ui_tx: ui_tx.clone(),
        };
        form.update_attachments();
        form
    }

    pub fn set_filename(&mut self, filename: &str) {
        let filename = filename.trim();
        match fs::read(filename) {
            Ok(filebody) => {
                let content_type = ContentType::parse("application/txt").unwrap();
                let attachment = Attachment::new(filename.to_owned()).body(filebody, content_type);
                let size = attachment.raw_body().len();
                self.letter
                    .attachment
                    .insert(filename.to_owned(), attachment);
                self.ui_tx
                    .send(UiEvent::PresentInfo(format!(
                        "Загружен файл: '{}'\n({} байт)",
                        filename.to_owned(),
                        size
                    )))
                    .unwrap();
            }
            Err(ref read_error) => self
                .ui_tx
                .send(UiEvent::PresentInfo(format!(
                    "Случилось непредвиденное:\n{}",
                    read_error
                )))
                .unwrap(),
        }
        self.update_attachments();
    }
}

impl LetterForm {
    fn get_area(&mut self, index: usize) -> &TextArea {
        let scroll = self
            .view
            .get_content()
            .downcast_ref::<ScrollView<LinearLayout>>()
            .unwrap();
        let entry = scroll
            .get_inner()
            .get_child(index)
            .unwrap()
            .downcast_ref::<LinearLayout>()
            .unwrap();
        let widget = entry
            .get_child(2)
            .unwrap()
            .downcast_ref::<ResizedView<TextArea>>()
            .unwrap()
            .get_inner();
        widget
    }

    fn update_attachments(&mut self) {
        let text = self
            .view
            .get_content_mut()
            .downcast_mut::<ScrollView<LinearLayout>>()
            .unwrap()
            .get_inner_mut()
            .get_child_mut(2)
            .unwrap()
            .downcast_mut::<TextView>()
            .unwrap();
        let mut info = vec![];
        for (a, b) in self.letter.attachment.iter() {
            info.push(format!("['{}' ({} байт)]", a, b.raw_body().len()));
        }
        if info.is_empty() {
            text.set_content("Вложения");
        } else {
            text.set_content(format!("Вложения: {}.", info.join(", ")));
        }
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

fn init_dialog(letter: &Letter) -> Dialog {
    Dialog::around(init_form(letter).scrollable())
        .title("Редактируем письмо")
        .button("OK", |_| {})
        .button("Cancel", |_| {})
        .button("Add file", |_| {})
        .button("Remove file", |_| {})
        .button("Send", |_| {})
}

fn init_form(letter: &Letter) -> impl View {
    LinearLayout::vertical()
        .child(text_entry_full_width("     Тема:", &letter.topic))
        .child(text_entry_full_width("Сообщение:", &letter.text))
        .child(TextView::new("Вложения"))
}
