use crate::controller::Controller;

pub struct App {
    controller: Controller,
}

impl App {
    pub fn new() -> Self {
        Self {
            controller: Controller::new(),
        }
    }

    pub fn go(mut self) {
        self.controller.run();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
