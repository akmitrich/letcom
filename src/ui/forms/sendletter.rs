use cursive::{
    view::ViewWrapper,
    views::{Dialog, DummyView},
    wrap_impl,
};

pub struct SendLetterForm {
    view: Dialog,
}

impl SendLetterForm {
    pub fn new() -> Self {
        Self {
            view: Dialog::around(DummyView),
        }
    }
}

impl ViewWrapper for SendLetterForm {
    wrap_impl!(self.view: Dialog);
}
