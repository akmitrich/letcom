use std::sync::mpsc;

use cursive::menu::Tree;

use crate::{controller::ControllerSignal, ui};

pub fn email_menu(controller_tx: &mpsc::Sender<ControllerSignal>) -> Tree {
    let tx = controller_tx.clone();
    Tree::new().leaf("Form", move |c| {
        let form = ui::forms::email::EmailForm::new(&tx);
        c.add_layer(form.dialog());
    })
}
