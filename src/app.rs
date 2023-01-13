use cursive::{views::TextView, Cursive, CursiveExt};

pub struct App {
    siv: Cursive,
}

impl App {
    pub fn new() -> Self {
        Self {
            siv: Cursive::new(),
        }
    }

    pub fn go(mut self) {
        self.siv
            .add_layer(TextView::new("Hello World!\nPress q to quit."));
        self.siv.add_global_callback('q', Cursive::quit);
        self.siv.run();
    }
}
