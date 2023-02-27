use std::marker::PhantomData;

use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    view::ViewWrapper,
    views::{Dialog, DialogFocus},
    wrap_impl, View,
};

use crate::ui::utils::{dismiss, form_view, get_area_from_form};

use super::SetData;

pub struct OpenFileDialog<P> {
    view: Dialog,
    parent_name: String,
    ph: PhantomData<P>,
}

impl<P> OpenFileDialog<P> {
    pub fn new(parent_name: String) -> Self {
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
                let filename = self.get_filename();
                let parent_name = self.parent_name.to_string();
                EventResult::with_cb_once(move |c| {
                    let parent_name = parent_name.to_string();
                    if let Some(mut parent) = c.find_name::<P>(&parent_name) {
                        parent.set_data(filename);
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
    fn get_filename(&self) -> String {
        get_area_from_form(&self.view, 0).get_content().to_string()
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
    Dialog::around(create_view())
        .title("Открываем файл")
        .button("Ok", |_| {})
        .button("Cancel", |_| {})
}

fn create_view() -> impl View {
    form_view(vec![("Имя файла:", "Cargo.toml")])
}
