use std::{
    fs,
    io::{self, BufRead},
    sync::mpsc,
};

use cursive::menu::Tree;

use crate::{controller::ControllerSignal, data_handler::persona::import_persona};

pub fn persona_menu(controller_tx: &mpsc::Sender<ControllerSignal>) -> Tree {
    let mut tree = Tree::new();
    let select_tx = controller_tx.clone();
    tree.add_leaf("Select...", move |_| {
        select_tx.send(ControllerSignal::SelectPersona).unwrap()
    });
    let import_tx = controller_tx.clone();
    tree.add_delimiter();
    tree.add_leaf("Import 'persona.tsv'", move |_| {
        if let Ok(f) = fs::File::open("persona.tsv") {
            let import = io::BufReader::new(f)
                .lines()
                .skip(1)
                .filter_map(|x| x.ok())
                .filter_map(|data| import_persona(&data))
                .collect::<Vec<_>>();
            import_tx
                .send(ControllerSignal::ImportPersona(import))
                .unwrap();
        } else {
            import_tx
                .send(ControllerSignal::Log(format!(
                    "Открытие файла 'persona.tsv' завершилось провалом!"
                )))
                .unwrap();
        }
    });
    tree
}
