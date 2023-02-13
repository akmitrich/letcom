use std::sync::mpsc;

use cursive::{
    event::{Event, Key},
    view::Nameable,
    views::{Dialog, TextView},
    Cursive, CursiveRunner,
};

use crate::{
    controller::{settings::Settings, ControllerSignal},
    data_handler::{letter::Letter, persona::Persona, Identity},
};

pub struct Ui {
    runner: CursiveRunner<Cursive>,
    controller_tx: mpsc::Sender<ControllerSignal>,
}

impl Ui {
    pub fn new(controller_tx: &mpsc::Sender<ControllerSignal>) -> Self {
        let ncurses = cursive::backends::curses::n::Backend::init().unwrap();
        let mut runner = CursiveRunner::new(Cursive::default(), ncurses);
        let controller_tx = controller_tx.clone();
        init_menu(&mut runner, &controller_tx);
        init_view(&mut runner);
        runner.refresh();
        Self {
            runner,
            controller_tx,
        }
    }

    pub fn step_next(&mut self) {
        if self.runner.is_running() {
            self.runner.step();
            self.runner.refresh();
        } else {
            self.controller_tx.send(ControllerSignal::Quit).unwrap();
        }
    }

    pub fn remove_persona_dialog(&mut self, persona: Persona) {
        self.runner
            .add_layer(dialogs::remove_persona::remove_persona_dialog(
                persona,
                &self.controller_tx,
            ));
    }
}

impl Ui {
    pub(crate) fn settings_form(&mut self, settings: Settings) {
        self.runner.add_layer(forms::settings::SettingsForm::new(
            settings,
            &self.controller_tx,
        ))
    }

    pub(crate) fn letter_form(&mut self, letter: Letter) {
        let letter_form_id = uuid::Uuid::new_v4();
        self.runner.add_layer(
            forms::letter::LetterForm::new(letter_form_id, letter, &self.controller_tx)
                .with_name(letter_form_id.to_string()),
        );
    }

    pub(crate) fn send_letter_form(&mut self, letter: Letter, people: Vec<Persona>) {
        self.runner
            .add_layer(forms::sendletter::SendLetterForm::new(
                letter,
                people,
                &self.controller_tx,
            ));
    }

    pub(crate) fn select_persona_form(&mut self, persona: Vec<Persona>) {
        if persona.is_empty() {
            self.controller_tx
                .send(ControllerSignal::Log(
                    "В данный момент у меня в памяти никого нет.\nДобавьте персон!".into(),
                ))
                .unwrap();
        } else {
            self.runner
                .add_layer(forms::selectpersona::SelectPersonaForm::new(
                    persona,
                    &self.controller_tx,
                ));
        }
    }

    pub(crate) fn edit_persona_form(&mut self, key: Identity, persona: Persona) {
        self.runner
            .add_layer(forms::editpersona::EditPersonaForm::new(
                key,
                persona,
                &self.controller_tx,
            ));
    }

    pub(crate) fn present_info(&mut self, info: impl AsRef<str>) {
        self.runner.add_layer(Dialog::info(info.as_ref()));
    }
}

fn init_menu(siv: &mut Cursive, controller_tx: &mpsc::Sender<ControllerSignal>) {
    let menu = siv.menubar();
    menu.add_subtree("Persona", menus::persona::persona_menu(controller_tx));
    menu.add_subtree("Email", menus::email::email_menu(controller_tx));
    let tx = controller_tx.clone();
    menu.add_leaf("Quit", move |_| tx.send(ControllerSignal::Quit).unwrap());
    siv.add_global_callback(Key::Esc, |c| c.select_menubar());
    siv.set_autohide_menu(false);
}

fn init_view(siv: &mut Cursive) {
    siv.add_layer(TextView::new(
        "Вас приветствует составитель писем!\nPress Ctrl-q to quit.",
    ));
    siv.add_global_callback(Event::CtrlChar('q'), Cursive::quit);
}

mod dialogs;
mod forms;
mod menus;
mod utils;
