use std::sync::mpsc;

use cursive::{
    view::Scrollable,
    views::{Dialog, LinearLayout, TextView},
    View,
};

use crate::controller::ControllerSignal;

pub struct EmailForm {
    controller_tx: mpsc::Sender<ControllerSignal>,
}

impl EmailForm {
    const INTRO: &str = "email_dialog_intro";
    const TEXT: &str = "email_dialog_text";
    const OUTRO: &str = "email_dialog_outro";

    pub fn new(controller_tx: &mpsc::Sender<ControllerSignal>) -> Self {
        Self {
            controller_tx: controller_tx.clone(),
        }
    }

    pub fn dialog(&self) -> Dialog {
        let tx = self.controller_tx.clone();
        Dialog::around(email_form_layout())
            .button("OK", |c| {
                c.pop_layer();
            })
            .button("Cancel", |c| {
                c.pop_layer();
            })
            .button("Send", move |c| {
                tx.send(ControllerSignal::SendEmail).unwrap();
                c.pop_layer();
            })
    }
}

fn email_form_layout() -> impl View {
    LinearLayout::vertical()
        .child(TextView::new("Письмо"))
        .scrollable()
}
