use std::sync::mpsc;

use cursive::menu::Tree;

use crate::{
    controller::{self, ControllerSignal},
    ui,
};

pub fn email_menu(controller_tx: &mpsc::Sender<ControllerSignal>) -> Tree {
    let form_tx = controller_tx.clone();
    let tree = Tree::new().leaf("Form", move |c| {
        let form = ui::forms::email::EmailForm::new(&form_tx);
        c.add_layer(form);
    });
    let settings_tx = controller_tx.clone();
    tree.leaf("Settings...", move |c| {
        let settings = controller::settings::Settings::load();
        let form = ui::forms::settings::SettingsForm::new(settings, &settings_tx);
        c.add_layer(form);
    })
}
