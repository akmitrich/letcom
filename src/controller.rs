use std::sync::mpsc;

use crate::ui::{Ui, UiEvent};

use self::settings::Settings;

pub struct Controller {
    ui_tx: mpsc::Sender<UiEvent>,
    settings: Settings,
    rx: mpsc::Receiver<ControllerSignal>,
    tx: mpsc::Sender<ControllerSignal>,
}

impl Controller {
    pub fn new(
        rx: mpsc::Receiver<ControllerSignal>,
        tx: &mpsc::Sender<ControllerSignal>,
        ui_tx: &mpsc::Sender<UiEvent>,
    ) -> Self {
        Self {
            ui_tx: ui_tx.clone(),
            settings: Settings::load(),
            rx,
            tx: tx.clone(),
        }
    }

    pub fn run(&mut self, ui: &mut Ui) {
        loop {
            if self.process_signals() {
                break;
            }
            ui.step_next();
        }
    }
}

impl Controller {
    fn process_signals(&mut self) -> bool {
        use ControllerSignal::*;
        if let Ok(signal) = self.rx.try_recv() {
            match signal {
                EditSettings => {
                    self.ui_tx
                        .send(UiEvent::SettingsForm(self.settings.clone()))
                        .unwrap();
                }
                UpdateSettings(s) => {
                    self.settings = s;
                    self.settings.save();
                }
                SendEmail => {
                    eprintln!("Email processing...");
                }
                Quit => return true,
                any => eprintln!("Unexpected controller signal: {:?}", any),
            }
        }
        false
    }
}

#[derive(Debug)]
pub enum ControllerSignal {
    Noop,
    EditSettings,
    UpdateSettings(Settings),
    SendEmail,
    Quit,
}

pub mod settings;
