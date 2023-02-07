use std::sync::mpsc;

use crate::{
    data_handler::{
        data_container::restore,
        persona::{restore_persona_container, Persona, PersonaContainer},
        tag::TagContainer,
        Represent,
    },
    ui::Ui,
};

use self::{
    letter::{create_new_letter, Letter},
    settings::{load_settings, Settings},
};

const PERSONA_CONTAINER_PATH: &str = "persona.json";
const TAG_CONTAINER_PATH: &str = "tag.json";

pub struct Controller {
    ui: Ui,
    settings: Settings,
    persona_container: PersonaContainer,
    tag_container: TagContainer,
    rx: mpsc::Receiver<ControllerSignal>,
    tx: mpsc::Sender<ControllerSignal>,
}

impl Controller {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let ui = Ui::new(&tx);
        let settings = load_settings();
        let persona_container =
            restore_persona_container(PERSONA_CONTAINER_PATH).unwrap_or_default();
        let tag_container = restore(TAG_CONTAINER_PATH).unwrap_or_default();
        Self {
            ui,
            settings,
            persona_container,
            tag_container,
            rx,
            tx,
        }
    }

    pub fn run(&mut self) {
        while self.process_signals() {
            self.ui.step_next();
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
                Log(info) => self.log(info),
                OpenSettings => self.open_settings(),
                SaveSettings => self.save_settings(),
                NewLetter => self.new_letter(),
                OpenLetterToSend(letter) => self.open_letter_to_send(letter),
                SendEmail { letter, to } => self.send_email(letter, to),
                ImportPersona(p) => self.import_persona(p),
                SelectPersona => self.select_persona(),
                EditPersona(p) => self.edit_persona(p),
                RemovePersonaAlert(p) => self.remove_persona_alert(p),
                RemovePersona(p) => self.remove_persona(p),
                Quit => return self.finalize(),
                any => eprintln!("Unexpected controller signal: {:?}", any),
            }
        }
        true
    }

    fn log(&mut self, info: impl AsRef<str>) {
        self.ui.present_info(info);
    }

    fn open_settings(&mut self) {
        self.ui.settings_form(self.settings.clone());
    }

    fn save_settings(&self) {
        self.settings.read().unwrap().save();
    }

    fn new_letter(&mut self) {
        let letter = create_new_letter();
        self.ui.letter_form(letter);
    }

    fn open_letter_to_send(&mut self, letter: Letter) {
        let addresses = vec!["ak.mitrich@mail.ru".to_owned()];
        self.ui.send_letter_form(letter, addresses);
    }

    fn send_email(&mut self, letter: Letter, to: Vec<String>) {
        self.tx
            .send(ControllerSignal::Log(format!(
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
            let identity = persona.read().unwrap().identity();
            self.persona_container.update(identity, persona);
        }
        self.tx
            .send(ControllerSignal::Log(format!(
                "Импортировано {count} персон. Теперь у нас {} персон.",
                self.persona_container.size()
            )))
            .unwrap();
    }

    fn select_persona(&mut self) {
        self.ui
            .select_persona_form(self.persona_container.all_representations().collect())
    }

    fn edit_persona(&mut self, persona: Persona) {
        self.ui.edit_persona_form(persona);
    }

    fn remove_persona_alert(&mut self, persona: Persona) {
        self.ui.remove_persona_dialog(persona);
    }

    fn remove_persona(&mut self, persona: Persona) {
        self.persona_container.remove_representation(persona);
    }

    fn finalize(&mut self) -> bool {
        self.persona_container.finalize(PERSONA_CONTAINER_PATH);
        self.tag_container.finalize(TAG_CONTAINER_PATH);
        false
    }
}

#[derive(Debug)]
pub enum ControllerSignal {
    Noop,
    Log(String),
    OpenSettings,
    SaveSettings,
    NewLetter,
    OpenLetterToSend(Letter),
    SendEmail { letter: Letter, to: Vec<String> },
    ImportPersona(Vec<Persona>),
    SelectPersona,
    EditPersona(Persona),
    RemovePersonaAlert(Persona),
    RemovePersona(Persona),
    Quit,
}

pub mod letter;
pub mod settings;
