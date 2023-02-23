use std::sync::mpsc;

use cursive::menu::Tree;

use crate::controller::ControllerSignal;

pub fn tag_menu(controller_tx: &mpsc::Sender<ControllerSignal>) -> Tree {
    let mut tree = Tree::new();
    let new_tx = controller_tx.clone();
    tree.add_leaf("New tag", move |_| {
        new_tx.send(ControllerSignal::NewTag).unwrap();
    });
    tree
}
