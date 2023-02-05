use std::marker::PhantomData;

use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    view::{Scrollable, ViewWrapper},
    views::{Dialog, DialogFocus, LinearLayout, ResizedView, ScrollView, TextArea},
    wrap_impl, View,
};

use crate::ui::utils::{dismiss, text_entry_full_width};

use super::SetData;

pub struct OpenFileDialog<P> {
    view: Dialog,
    parent_name: uuid::Uuid,
    ph: PhantomData<P>,
}

impl<P> OpenFileDialog<P> {
    pub fn new(parent_name: uuid::Uuid) -> Self {
        Self {
            view: init_dialog(),
            parent_name,
            ph: PhantomData,
        }
    }
}

impl<P: SetData + ViewWrapper> OpenFileDialog<P> {
    fn button_event(&mut self, button: usize) -> EventResult {
        match button {
            0 => {
                let filename = self.get_filenames().collect::<Vec<_>>();
                let parent_name = self.parent_name;
                EventResult::with_cb_once(move |c| {
                    let parent_name = parent_name.to_string();
                    if let Some(mut parent) = c.find_name::<P>(&parent_name) {
                        parent.set_data(filename.into_iter());
                    } else {
                        panic!("Unable to find parent window");
                    }
                    c.pop_layer();
                })
            }
            1 => dismiss(),
            _ => EventResult::Ignored,
        }
    }
}

impl<P> OpenFileDialog<P> {
    fn get_filenames(&self) -> impl Iterator<Item = String> + '_ {
        [self
            .view
            .get_content()
            .downcast_ref::<ScrollView<LinearLayout>>()
            .unwrap()
            .get_inner()
            .get_child(2)
            .unwrap()
            .downcast_ref::<ResizedView<TextArea>>()
            .unwrap()
            .get_inner()
            .get_content()
            .to_string()]
        .into_iter()
    }
}

impl<P: ViewWrapper + SetData + 'static> ViewWrapper for OpenFileDialog<P> {
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
            Event::Key(Key::Esc) => dismiss(),
            _ => self
                .with_view_mut(|v| v.on_event(event))
                .unwrap_or(EventResult::Ignored),
        }
    }
}

fn init_dialog() -> Dialog {
    Dialog::around(create_view().scrollable())
        .title("Открываем файл")
        .button("Ok", |_| {})
        .button("Cancel", |_| {})
}

fn create_view() -> impl View {
    text_entry_full_width("Имя файла:", "Cargo.toml")
}
