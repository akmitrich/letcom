use std::sync::mpsc;

use cursive::menu::Tree;

use crate::{
    controller::{letter::Letter, ControllerSignal},
    ui::UiEvent,
};

pub fn email_menu(
    controller_tx: &mpsc::Sender<ControllerSignal>,
    ui_tx: &mpsc::Sender<UiEvent>,
) -> Tree {
    let new_letter_tx = ui_tx.clone();
    let tree = Tree::new()
        .leaf("New Letter", move |_| {
            new_letter_tx
                .send(UiEvent::LetterForm(Letter::new()))
                .unwrap();
        })
        .delimiter();
    let settings_tx = controller_tx.clone();
    tree.leaf("Settings...", move |_| {
        settings_tx.send(ControllerSignal::OpenSettings).unwrap();
    })
}
