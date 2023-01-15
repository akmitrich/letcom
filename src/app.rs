use std::sync::mpsc;

use crate::{
    controller::{Controller, ControllerSignal},
    ui::{Ui, UiEvent},
};

pub struct App {
    controller: Controller,
    ui: Ui,
}

impl App {
    pub fn new() -> Self {
        let (controller_tx, controller_rx) = mpsc::channel::<ControllerSignal>();
        let (ui_tx, ui_rx) = mpsc::channel::<UiEvent>();
        Self {
            controller: Controller::new(controller_rx, &controller_tx, &ui_tx),
            ui: Ui::new(ui_rx, &ui_tx, &controller_tx),
        }
    }

    pub fn go(mut self) {
        self.controller.run(&mut self.ui);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
