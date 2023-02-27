use std::sync::mpsc;

use cursive::{
    event::EventResult,
    view::{Resizable, Scrollable},
    views::{Dialog, DummyView, LinearLayout, ResizedView, ScrollView, TextArea, TextView},
    View,
};

use crate::controller::ControllerSignal;

pub fn text_entry_full_width(label: &str, content: &str) -> LinearLayout {
    LinearLayout::horizontal()
        .child(TextView::new(label))
        .child(DummyView)
        .child(TextArea::new().content(content).full_width())
}

pub fn linear_layout_form(entries: Vec<(&str, &str)>) -> LinearLayout {
    let mut layout = LinearLayout::vertical();
    for (label, content) in entries {
        layout.add_child(text_entry_full_width(label, content));
    }
    layout
}

pub fn form_view(entries: Vec<(&str, &str)>) -> impl View {
    linear_layout_form(entries).scrollable()
}

pub fn get_from_layout<V: View>(layout: &LinearLayout, index: usize) -> &V {
    layout
        .get_child(index)
        .unwrap()
        .downcast_ref::<V>()
        .unwrap()
}

pub fn get_area_from_form(dialog: &Dialog, entry_index: usize) -> &TextArea {
    const TEXT_AREA_IN_ENTRY: usize = 2;
    let main_layout = dialog
        .get_content()
        .downcast_ref::<ScrollView<LinearLayout>>()
        .unwrap()
        .get_inner();
    let entry_layout = get_from_layout::<LinearLayout>(&main_layout, entry_index);
    get_from_layout::<ResizedView<TextArea>>(entry_layout, TEXT_AREA_IN_ENTRY).get_inner()
}

pub fn get_text_from_form_entry(dialog: &Dialog, entry_index: usize) -> &str {
    get_area_from_form(dialog, entry_index).get_content()
}

pub fn get_view_from_dialog<V: View>(dialog: &Dialog, view_index: usize) -> &V {
    get_from_layout::<V>(
        dialog.get_content().downcast_ref::<LinearLayout>().unwrap(),
        view_index,
    )
}

pub fn no_selection_info(
    controller_tx: &mpsc::Sender<ControllerSignal>,
    action: &str,
    object: &str,
) {
    controller_tx
        .send(ControllerSignal::Log(format!(
            "Для выполнения {action} требуется выбрать {object}."
        )))
        .unwrap();
}

pub fn dismiss() -> EventResult {
    EventResult::with_cb(|c| {
        c.pop_layer();
    })
}
