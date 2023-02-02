use std::sync::mpsc;

use crate::{
    data_handler::persona::{Persona, PersonaContainer},
    ui::{Ui, UiEvent},
};

use self::{
    letter::{create_new_letter, Letter},
    settings::{load_settings, Settings},
};

pub struct Controller {
    ui_tx: mpsc::Sender<UiEvent>,
    settings: Settings,
    persona_container: PersonaContainer,
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
            persona_container: Default::default(),
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
                OpenLetterToSend(letter) => self.open_letter_to_send(letter),
                SendEmail { letter, to } => self.send_email(letter, to),
                ImportPersona(p) => self.import_persona(p),
                Quit => return false,
                any => eprintln!("Unexpected controller signal: {:?}", any),
            }
        }
        true
    }

    fn open_settings(&self) {
        self.ui_tx
            .send(UiEvent::SettingsForm(self.settings.clone()))
            .unwrap();
    }

    fn save_settings(&self) {
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

    fn send_email(&mut self, letter: Letter, to: Vec<String>) {
        self.ui_tx
            .send(UiEvent::PresentInfo(format!(
                "To: {:?}\n{:?}\n{:?}\n{:?}",
                to,
                letter.read().unwrap().topic,
                letter.read().unwrap().text,
                letter.read().unwrap().attachment_info()
            )))
            .unwrap();
    }

    fn import_persona(&mut self, persona: Vec<Persona>) {
        let count = persona.len();
        for persona in persona {
            self.persona_container.update_persona(persona);
        }
        self.ui_tx
            .send(UiEvent::PresentInfo(format!(
                "Импортировано {count} персон. Теперь у нас {} персон.",
                self.persona_container.len()
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
    ImportPersona(Vec<Persona>),
    Quit,
}

pub mod letter;
pub mod settings;
