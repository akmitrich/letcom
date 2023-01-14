use std::sync::mpsc;

use cursive::{event::Key, views::TextView, Cursive, CursiveRunner};

use crate::controller::ControllerSignal;

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

fn init_menu(siv: &mut Cursive, controller_tx: &mpsc::Sender<ControllerSignal>) {
    let menu = siv.menubar();
    let tx = controller_tx.clone();
    menu.add_subtree("Email", menus::email::email_menu(controller_tx));
    menu.add_leaf("Quit", move |_| tx.send(ControllerSignal::Quit).unwrap());
    siv.add_global_callback(Key::Esc, |c| c.select_menubar());
    siv.set_autohide_menu(false);
}

fn init_view(siv: &mut Cursive) {
    siv.add_layer(TextView::new("Hello World!\nPress q to quit."));
    siv.add_global_callback('q', Cursive::quit);
}

impl Ui {
    fn process_messages(&mut self) {}
}

mod forms;
mod menus;
