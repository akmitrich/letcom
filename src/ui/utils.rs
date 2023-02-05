use cursive::{
    event::EventResult,
    view::Resizable,
    views::{Dialog, DummyView, LinearLayout, ResizedView, ScrollView, TextArea, TextView},
};

pub fn text_entry_full_width(label: &str, content: &str) -> LinearLayout {
    LinearLayout::horizontal()
        .child(TextView::new(label))
        .child(DummyView)
        .child(TextArea::new().content(content).full_width())
}

pub fn dismiss() -> EventResult {
    EventResult::with_cb(|c| {
        c.pop_layer();
    })
}

pub fn get_area_from(dialog: &Dialog, n: usize) -> &TextArea {
    dialog
        .get_content()
        .downcast_ref::<ScrollView<LinearLayout>>()
        .unwrap()
        .get_inner()
        .get_child(n)
        .unwrap()
        .downcast_ref::<LinearLayout>()
        .unwrap()
        .get_child(2)
        .unwrap()
        .downcast_ref::<ResizedView<TextArea>>()
        .unwrap()
        .get_inner()
}
