use std::{fs, sync::mpsc};

use crate::ui;

pub struct Controller {
    ui: ui::Ui,
    rx: mpsc::Receiver<ControllerSignal>,
}

impl Controller {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            ui: ui::Ui::new(&tx),
            rx,
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.process_signals() {
                break;
            }
            self.ui.step_next();
        }
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}

impl Controller {
    fn process_signals(&mut self) -> bool {
        use ControllerSignal::*;
        match self.rx.try_recv() {
            Ok(signal) => match signal {
                SendEmail => {
                    let contents = "Send email processing.";
                    fs::write("log.txt", contents).unwrap();
                }
                Quit => return true,
            },
            Err(_e) => {}
        }
        false
    }
}

pub enum ControllerSignal {
    SendEmail,
    Quit,
}
