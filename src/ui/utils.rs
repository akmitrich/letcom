use cursive::{
    event::EventResult,
    view::{Resizable, Scrollable},
    views::{Dialog, DummyView, LinearLayout, ResizedView, ScrollView, TextArea, TextView},
    View,
};

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

pub fn get_area_from(dialog: &Dialog, entry_index: usize) -> &TextArea {
    const TEXT_AREA_IN_ENTRY: usize = 2;
    dialog
        .get_content()
        .downcast_ref::<ScrollView<LinearLayout>>()
        .unwrap()
        .get_inner()
        .get_child(entry_index)
        .unwrap()
        .downcast_ref::<LinearLayout>()
        .unwrap()
        .get_child(TEXT_AREA_IN_ENTRY)
        .unwrap()
        .downcast_ref::<ResizedView<TextArea>>()
        .unwrap()
        .get_inner()
}

pub fn dismiss() -> EventResult {
    EventResult::with_cb(|c| {
        c.pop_layer();
    })
}
