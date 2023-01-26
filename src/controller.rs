use std::sync::mpsc;

use crate::ui::{Ui, UiEvent};

use self::{letter::Letter, settings::Settings};

pub struct Controller {
    ui_tx: mpsc::Sender<UiEvent>,
    settings: Settings,
    rx: mpsc::Receiver<ControllerSignal>,
    _tx: mpsc::Sender<ControllerSignal>,
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
            _tx: tx.clone(),
        }
    }

    pub fn run(&mut self, ui: &mut Ui) {
        while self.process_signals() {
            ui.step_next();
        }
    }
}

impl Controller {
    fn process_signals(&mut self) -> bool {
        use ControllerSignal::*;
        if let Ok(signal) = self.rx.try_recv() {
            #[allow(unreachable_patterns)]
            match signal {
                Noop => {}
                OpenSettings => self.open_settings(),
                UpdateSettings(s) => self.update_settings(s),
                AppendLetter(l) => self.append_letter(l),
                Quit => return false,
                any => eprintln!("Unexpected controller signal: {:?}", any),
            }
        }
        true
    }

    fn open_settings(&mut self) {
        self.ui_tx
            .send(UiEvent::SettingsForm(self.settings.clone()))
            .unwrap();
    }

    fn update_settings(&mut self, s: Settings) {
        self.settings = s;
        self.settings.save();
    }

    fn append_letter(&mut self, letter: Letter) {}
}

#[derive(Debug)]
pub enum ControllerSignal {
    Noop,
    OpenSettings,
    UpdateSettings(Settings),
    AppendLetter(Letter),
    Quit,
}

pub mod letter;
pub mod settings;
