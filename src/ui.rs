use std::sync::mpsc;

use cursive::{
    event::{Event, Key},
    views::TextView,
    Cursive, CursiveRunner,
};

use crate::controller::{self, ControllerSignal};

pub struct Ui {
    runner: CursiveRunner<Cursive>,
    controller_tx: mpsc::Sender<ControllerSignal>,
    rx: mpsc::Receiver<UiEvent>,
    tx: mpsc::Sender<UiEvent>,
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
        init_menu(&mut runner, &controller_tx);
        init_view(&mut runner);
        runner.refresh();
        Self {
            runner,
            rx,
            tx: tx.clone(),
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
            match event {
                Noop => todo!(),
                SettingsForm(settings) => self.runner.add_layer(
                    forms::settings::SettingsForm::new(settings, &self.controller_tx),
                ),
                any => eprintln!("Unexpected UI event {:?}", any),
            }
        }
    }
}

fn init_menu(siv: &mut Cursive, controller_tx: &mpsc::Sender<ControllerSignal>) {
    let menu = siv.menubar();
    let tx = controller_tx.clone();
    menu.add_subtree("Email", menus::email::email_menu(controller_tx));
    menu.add_leaf("Quit", move |_| tx.send(ControllerSignal::Quit).unwrap());
    siv.add_global_callback(Key::Esc, |c| c.select_menubar());
    siv.set_autohide_menu(false);
}

fn init_view(siv: &mut Cursive) {
    siv.add_layer(TextView::new("Hello World!\nPress Ctrl-q to quit."));
    siv.add_global_callback(Event::CtrlChar('q'), Cursive::quit);
}

#[derive(Debug)]
pub enum UiEvent {
    Noop,
    StepNext,
    SettingsForm(controller::settings::Settings),
}

mod forms;
mod menus;
mod utils;
