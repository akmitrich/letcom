use cursive::{
    view::Resizable,
    views::{DummyView, LinearLayout, TextArea, TextView},
};

pub fn text_entry_full_width(label: &str, content: &str) -> LinearLayout {
    LinearLayout::horizontal()
        .child(TextView::new(label))
        .child(DummyView)
        .child(TextArea::new().content(content).full_width())
}
