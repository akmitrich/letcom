use cursive::{
    view::ViewWrapper,
    views::{Dialog, TextView},
    wrap_impl,
};

pub struct SelectPersonaForm {
    view: Dialog,
}

impl SelectPersonaForm {
    pub fn new() -> Self {
        Self {
            view: Dialog::around(TextView::new("Здесь выбирают персону.")),
        }
    }
}

impl ViewWrapper for SelectPersonaForm {
    wrap_impl!(self.view: Dialog);
}
