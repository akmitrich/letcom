use std::sync::mpsc;

use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    view::{Scrollable, ViewWrapper},
    views::{Dialog, DialogFocus, LinearLayout, ResizedView, ScrollView, TextArea, TextView},
    wrap_impl, View,
};

use crate::{controller::ControllerSignal, ui::utils::text_entry_full_width};

pub struct EmailForm {
    view: Dialog,
    controller_tx: mpsc::Sender<ControllerSignal>,
}

impl EmailForm {
    pub fn new(controller_tx: &mpsc::Sender<ControllerSignal>) -> Self {
        let mut form = Self {
            view: init_dialog(),
            controller_tx: controller_tx.clone(),
        };
        form.populate_form();
        form
    }
}

impl EmailForm {
    const INTRO: &str = "email_form_intro";
    const TEXT: &str = "email_form_text";
    const OUTRO: &str = "email_form_outro";

    fn populate_form(&mut self) {
        self.get_area(2)
            .set_content("С уважением,\nАлександр Калашников.");
    }

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
        self.dismiss()
    }

    fn event_cancel(&mut self) -> EventResult {
        self.dismiss()
    }

    fn event_send(&mut self) -> EventResult {
        self.controller_tx
            .send(ControllerSignal::SendEmail)
            .unwrap();
        self.dismiss()
    }

    fn dismiss(&self) -> EventResult {
        EventResult::with_cb(|c| {
            c.pop_layer();
        })
    }
}

impl ViewWrapper for EmailForm {
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
                        DialogFocus::Button(0) => self.event_submit(),
                        DialogFocus::Button(1) => self.event_cancel(),
                        DialogFocus::Button(2) => self.event_send(),
                        _ => EventResult::Ignored,
                    }
                } else {
                    EventResult::Ignored
                }
            }
            Event::Key(Key::Enter) => match self.view.focus() {
                DialogFocus::Button(0) => self.event_submit(),
                DialogFocus::Button(1) => self.event_cancel(),
                DialogFocus::Button(2) => self.event_send(),
                _ => self
                    .with_view_mut(|v| v.on_event(event))
                    .unwrap_or(EventResult::Ignored),
            },
            _ => self
                .with_view_mut(|v| v.on_event(event))
                .unwrap_or(EventResult::Ignored),
        }
    }
}

fn init_dialog() -> Dialog {
    Dialog::around(init_form().scrollable())
        .button("OK", |_| {})
        .button("Cancel", |_| {})
        .button("Send", |_| {})
}

fn init_form() -> impl View {
    LinearLayout::vertical()
        .child(text_entry_full_width("     Тема:", EmailForm::INTRO))
        .child(text_entry_full_width("Сообщение:", EmailForm::TEXT))
        .child(text_entry_full_width("  Подпись:", EmailForm::OUTRO))
        .child(TextView::new("Вложения"))
}
