use cursive::CursiveExt;

use crate::ui;

pub struct Controller {
    ui: ui::Ui,
}

impl Controller {
    pub fn new() -> Self {
        Self { ui: ui::Ui::new() }
    }

    pub fn run(&mut self) {
        self.ui.siv.run()
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}
