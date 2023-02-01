use std::sync::mpsc;

use crate::ui::{Ui, UiEvent};

use self::{
    letter::{create_new_letter, Letter},
    settings::{load_settings, Settings},
};

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
        let settings = load_settings();
        Self {
            ui_tx: ui_tx.clone(),
            settings,
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
                SaveSettings => self.save_settings(),
                NewLetter => self.new_letter(),
                OpenLetterToSend(l) => self.open_letter_to_send(l),
                SendEmail {
                    letter,
                    to: addresses,
                } => self.send_email(letter, addresses),
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

    fn save_settings(&mut self) {
        self.settings.read().unwrap().save();
    }

    fn new_letter(&mut self) {
        let letter = create_new_letter();
        self.ui_tx.send(UiEvent::LetterForm(letter)).unwrap();
    }

    fn open_letter_to_send(&mut self, letter: Letter) {
        let addresses = vec!["ak.mitrich@mail.ru".to_owned()];
        self.ui_tx
            .send(UiEvent::SendForm { letter, addresses })
            .unwrap();
    }

    fn send_email(&mut self, letter: Letter, addresses: Vec<String>) {
        self.ui_tx
            .send(UiEvent::PresentInfo(format!(
                "To: {:?}\n{:?}\n{:?}\n{:?}",
                addresses,
                letter.read().unwrap().topic,
                letter.read().unwrap().text,
                letter.read().unwrap().attachment_info()
            )))
            .unwrap();
    }
}

#[derive(Debug)]
pub enum ControllerSignal {
    Noop,
    OpenSettings,
    SaveSettings,
    NewLetter,
    OpenLetterToSend(Letter),
    SendEmail { letter: Letter, to: Vec<String> },
    Quit,
}

pub mod letter;
pub mod settings;
