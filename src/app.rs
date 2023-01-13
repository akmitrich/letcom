use crate::ui;
use cursive::CursiveExt;

pub struct App {
    ui: ui::Ui,
}

impl App {
    pub fn new() -> Self {
        Self { ui: ui::Ui::new() }
    }

    pub fn go(mut self) {
        self.ui.siv.run();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
