pub use signals::ControllerSignal;
use std::sync::mpsc;

use crate::{
    data_handler::{
        handler::DataHandler,
        letter::{new_letter, Letter},
        make_ref,
        persona::Persona,
        tag::{new_tag, Tag},
        Identity, Represent,
    },
    ui::Ui,
};

use self::settings::{load_settings, Settings};

pub struct Controller {
    ui: Ui,
    settings: Settings,
    data_handler: DataHandler,
    rx: mpsc::Receiver<ControllerSignal>,
    tx: mpsc::Sender<ControllerSignal>,
}

impl Controller {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let ui = Ui::new(&tx);
        let settings = load_settings();
        Self {
            ui,
            settings,
            data_handler: DataHandler::new(),
            rx,
            tx,
        }
    }

    pub fn run(&mut self) {
        while self.process_signals() {
            self.ui.step_next();
        }
    }

    fn get_people(&mut self) -> Vec<Persona> {
        self.data_handler
            .get_people()
            .all_representations()
            .collect()
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
                NewTag => self.new_tag(),
                CompleteEditTag { key, tag } => self.complete_edit_tag(key, tag),
                NewLetter => self.new_letter(),
                EditLetter(letter) => self.edit_letter(letter),
                CompleteEditLetter { key, letter } => self.complete_edit_letter(key, letter),
                OpenLetterToSend(letter) => self.open_letter_to_send(letter),
                SendEmail { letter, to } => self.send_email(letter, to),
                ImportPersona(p) => self.import_persona(p),
                SelectPersona => self.select_persona(),
                EditPersona(p) => self.edit_persona(p),
                CompleteEditPersona { key, persona } => self.complete_edit_persona(key, persona),
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
        self.ui.settings_form(self.settings.clone()); //clone smart pointer to settings
    }

    fn save_settings(&self) {
        make_ref(&self.settings).save();
    }

    fn new_tag(&mut self) {
        let new_tag = new_tag("New");
        let people = self
            .data_handler
            .get_people()
            .idendities()
            .cloned()
            .collect::<Vec<_>>();
        self.ui
            .tag_form(make_ref(&new_tag).identity(), new_tag.clone(), &people);
    }

    fn complete_edit_tag(&mut self, key: Identity, tag: Tag) {
        self.data_handler.get_tags_mut().update_identity(key, tag);
    }

    fn new_letter(&mut self) {
        let letter = new_letter();
        self.tx.send(ControllerSignal::EditLetter(letter)).unwrap();
    }

    fn edit_letter(&mut self, letter: Letter) {
        let key = make_ref(&letter).identity();
        self.ui.letter_form(key, letter);
    }

    fn complete_edit_letter(&mut self, key: Identity, _letter: Letter) {
        self.tx
            .send(ControllerSignal::Log(format!("Complete window:\n{}", key)))
            .unwrap()
    }

    fn open_letter_to_send(&mut self, letter: Letter) {
        let people = self.get_people();
        self.ui.send_letter_form(letter, people);
    }

    fn send_email(&mut self, letter: Letter, to: Vec<String>) {
        let letter = make_ref(&letter);
        self.tx
            .send(ControllerSignal::Log(format!(
                "To: {:?}\n{:?}\n{:?}\n{:?}",
                to,
                letter.get_topic(),
                letter.get_text(),
                letter.attachment_info()
            )))
            .unwrap();
    }

    fn import_persona(&mut self, persona: Vec<Persona>) {
        let count = persona.len();
        for persona in persona {
            self.data_handler.get_people_mut().insert_or_update(persona);
        }
        self.tx
            .send(ControllerSignal::Log(format!(
                "Импортировано {count} персон. Теперь у нас {} персон.",
                self.data_handler.get_people().size()
            )))
            .unwrap();
    }

    fn select_persona(&mut self) {
        self.ui.select_persona_form(
            self.data_handler
                .get_people()
                .all_representations()
                .collect(),
        )
    }

    fn edit_persona(&mut self, persona: Persona) {
        let key = make_ref(&persona).identity();
        self.ui.edit_persona_form(key, persona);
    }

    fn complete_edit_persona(&mut self, key: Identity, persona: Persona) {
        self.data_handler
            .get_people_mut()
            .update_identity(key, persona);
    }

    fn remove_persona_alert(&mut self, persona: Persona) {
        self.ui.remove_persona_dialog(persona);
    }

    fn remove_persona(&mut self, persona: Persona) {
        self.data_handler
            .get_people_mut()
            .remove_representation(persona);
    }

    fn finalize(&mut self) -> bool {
        self.data_handler.finalize();
        false
    }
}

pub mod settings;
pub mod signals;
