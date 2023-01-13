use cursive::{views::TextView, Cursive};

pub struct Ui {
    pub siv: Cursive,
}

impl Ui {
    pub fn new() -> Self {
        let mut ui = Self {
            siv: Cursive::new(),
        };
        ui.init_menu();
        ui.init_view();
        ui
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}

impl Ui //private
{
    fn init_menu(&mut self) {}

    fn init_view(&mut self) {
        self.siv
            .add_layer(TextView::new("Hello World!\nPress q to quit."));
        self.siv.add_global_callback('q', Cursive::quit);
    }
}
