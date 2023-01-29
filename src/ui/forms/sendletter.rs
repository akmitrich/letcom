use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    view::{Scrollable, ViewWrapper},
    views::{Dialog, DialogFocus, LinearLayout, Panel, SelectView, TextView},
    wrap_impl, View,
};

use crate::{
    controller::letter::Letter,
    ui::utils::{dismiss, text_entry_full_width},
};

pub struct SendLetterForm {
    view: Dialog,
    letter: Letter,
    addresses: Vec<String>,
}

impl SendLetterForm {
    pub fn new(letter: Letter, addresses: Vec<String>) -> Self {
        Self {
            view: init_dialog(&letter, &addresses),
            letter,
            addresses,
        }
    }

    fn button_event(&self, _n: usize) -> EventResult {
        match _n {
            1 => self.event_cancel(),
            _ => EventResult::Ignored,
        }
    }

    fn event_cancel(&self) -> EventResult {
        dismiss()
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
    let mut select = SelectView::new();
    for a in addresses {
        select.add_item_str(a);
    }
    select
}

fn init_letter_panel(letter: &Letter) -> impl View {
    LinearLayout::vertical()
        .child(text_entry_full_width(" Тема:", &letter.topic))
        .child(text_entry_full_width("Текст:", &letter.text))
        .child(TextView::new(letter.attachment_description()))
        .scrollable()
}
