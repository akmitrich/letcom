use std::sync::mpsc;

use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    view::{Scrollable, ViewWrapper},
    views::{Dialog, DialogFocus, LinearLayout, ResizedView, ScrollView, TextArea},
    wrap_impl, View,
};

use crate::{
    controller::{settings::Settings, ControllerSignal},
    ui::utils::{dismiss, text_entry_full_width},
};

pub struct SettingsForm {
    view: Dialog,
    settings: Settings,
    controller_tx: mpsc::Sender<ControllerSignal>,
}

impl SettingsForm {
    pub fn new(settings: Settings, controller_tx: &mpsc::Sender<ControllerSignal>) -> Self {
        Self {
            view: init_dialog(&settings),
            settings,
            controller_tx: controller_tx.clone(),
        }
    }
}

impl SettingsForm {
    fn event_submit(&mut self) -> EventResult {
        self.update_settings();
        self.controller_tx
            .send(ControllerSignal::UpdateSettings(self.settings.clone()))
            .unwrap();
        dismiss()
    }

    fn event_cancel(&mut self) -> EventResult {
        dismiss()
    }
}

impl SettingsForm {
    //update logic impl block
    const SMTP_RELAY_I: usize = 0;
    const SMTP_USER_I: usize = 1;
    const SMTP_PASSWORD_I: usize = 2;
    const LETTER_FROM_I: usize = 3;
    const PLURAL_TITLE_I: usize = 4;
    const SINGLE_GREET_I: usize = 5;

    fn update_settings(&mut self) {
        self.update_smtp_relay();
        self.update_smtp_user();
        self.update_smtp_password();
        self.update_letter_from();
        self.update_plural_title();
        self.update_single_greet();
    }

    fn update_smtp_relay(&mut self) {
        self.settings.smtp_relay = self.get_data(Self::SMTP_RELAY_I);
    }

    fn update_smtp_user(&mut self) {
        self.settings.smtp_user = self.get_data(Self::SMTP_USER_I);
    }

    fn update_smtp_password(&mut self) {
        self.settings.smtp_password = self.get_data(Self::SMTP_PASSWORD_I);
    }
    fn update_letter_from(&mut self) {
        self.settings.letter_from = self.get_data(Self::LETTER_FROM_I);
    }
    fn update_plural_title(&mut self) {
        self.settings.plural_title = self.get_data(Self::PLURAL_TITLE_I);
    }
    fn update_single_greet(&mut self) {
        self.settings.single_greet = self.get_data(Self::SINGLE_GREET_I);
    }

    fn get_data(&self, index: usize) -> String {
        self.get_area(index).get_content().into()
    }

    fn get_area(&self, index: usize) -> &TextArea {
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
}

impl ViewWrapper for SettingsForm {
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
                        _ => EventResult::Ignored,
                    }
                } else {
                    EventResult::Ignored
                }
            }
            Event::Key(Key::Enter) => match self.view.focus() {
                DialogFocus::Button(0) => self.event_submit(),
                DialogFocus::Button(1) => self.event_cancel(),
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

fn init_dialog(settings: &Settings) -> Dialog {
    Dialog::around(init_form(settings))
        .button("OK", |_| {})
        .button("Cancel", |_| {})
}

fn init_form(settings: &Settings) -> impl View {
    LinearLayout::vertical()
        .child(text_entry_full_width("SMTP-сервер:", &settings.smtp_relay))
        .child(text_entry_full_width(
            "SMTP-пользователь:",
            &settings.smtp_user,
        ))
        .child(text_entry_full_width(
            "SMTP-пароль:",
            &settings.smtp_password,
        ))
        .child(text_entry_full_width("Отправитель:", &settings.letter_from))
        .child(text_entry_full_width("Обращение:", &settings.plural_title))
        .child(text_entry_full_width(
            "Приветствие:",
            &settings.single_greet,
        ))
        .scrollable()
}
