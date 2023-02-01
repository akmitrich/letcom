use std::sync::mpsc;

use cursive::menu::Tree;

use crate::{controller::ControllerSignal, ui::UiEvent};

pub fn email_menu(
    controller_tx: &mpsc::Sender<ControllerSignal>,
    _ui_tx: &mpsc::Sender<UiEvent>,
) -> Tree {
    use ControllerSignal::*;
    let new_letter_tx = controller_tx.clone();
    let tree = Tree::new()
        .leaf("New Letter", move |_| {
            new_letter_tx.send(NewLetter).unwrap();
        })
        .delimiter();
    let settings_tx = controller_tx.clone();
    tree.leaf("Settings...", move |_| {
        settings_tx.send(OpenSettings).unwrap();
    })
}
