use std::sync::mpsc;

use cursive::{
    event::{Event, Key},
    view::Nameable,
    views::{Dialog, TextView},
    Cursive, CursiveRunner,
};

use crate::controller::{letter::Letter, settings::Settings, ControllerSignal};

pub struct Ui {
    runner: CursiveRunner<Cursive>,
    controller_tx: mpsc::Sender<ControllerSignal>,
    rx: mpsc::Receiver<UiEvent>,
    _tx: mpsc::Sender<UiEvent>,
}

impl Ui {
    pub fn new(
        rx: mpsc::Receiver<UiEvent>,
        tx: &mpsc::Sender<UiEvent>,
        controller_tx: &mpsc::Sender<ControllerSignal>,
    ) -> Self {
        let ncurses = cursive::backends::curses::n::Backend::init().unwrap();
        let mut runner = CursiveRunner::new(Cursive::default(), ncurses);
        let controller_tx = controller_tx.clone();
        init_menu(&mut runner, &controller_tx, tx);
        init_view(&mut runner);
        runner.refresh();
        Self {
            runner,
            rx,
            _tx: tx.clone(),
            controller_tx,
        }
    }

    pub fn step_next(&mut self) {
        if self.runner.is_running() {
            self.process_messages();
            self.runner.step();
            self.runner.refresh();
            if self.need_to_stop() {
                self.controller_tx.send(ControllerSignal::Quit).unwrap();
            }
        }
    }

    pub fn need_to_stop(&self) -> bool {
        !self.runner.is_running()
    }
}

impl Ui {
    fn process_messages(&mut self) {
        use UiEvent::*;
        if let Ok(event) = self.rx.try_recv() {
            #[allow(unreachable_patterns)]
            match event {
                Noop => self.runner.refresh(),
                SettingsForm(settings) => self.settings_form(settings),
                LetterForm(letter) => self.letter_form(letter),
                PresentInfo(ref info) => self.present_info(info),
                any => eprintln!("Unexpected UI event {:?}", any),
            }
        }
    }

    fn settings_form(&mut self, settings: Settings) {
        self.runner.add_layer(forms::settings::SettingsForm::new(
            settings,
            &self.controller_tx,
        ))
    }

    fn letter_form(&mut self, letter: Letter) {
        let letter_form_id = uuid::Uuid::new_v4();
        self.runner.add_layer(
            forms::letter::LetterForm::new(letter_form_id, letter, &self.controller_tx, &self._tx)
                .with_name(letter_form_id.to_string()),
        );
    }

    fn present_info(&mut self, info: &str) {
        self.runner.add_layer(Dialog::info(info))
    }
}

fn init_menu(
    siv: &mut Cursive,
    controller_tx: &mpsc::Sender<ControllerSignal>,
    ui_tx: &mpsc::Sender<UiEvent>,
) {
    let menu = siv.menubar();
    let tx = controller_tx.clone();
    menu.add_subtree("Email", menus::email::email_menu(controller_tx, ui_tx));
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

#[derive(Debug)]
pub enum UiEvent {
    Noop,
    SettingsForm(Settings),
    LetterForm(Letter),
    PresentInfo(String),
}

mod dialogs;
mod forms;
mod menus;
mod utils;
