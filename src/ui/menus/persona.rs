use std::{
    fs,
    io::{self, BufRead},
    sync::mpsc,
};

use cursive::menu::Tree;

use crate::{controller::ControllerSignal, data_handler::persona::import_persona, ui::UiEvent};

pub fn persona_menu(
    controller_tx: &mpsc::Sender<ControllerSignal>,
    ui_tx: &mpsc::Sender<UiEvent>,
) -> Tree {
    let tree = Tree::new();
    let info_tx = ui_tx.clone();
    let import_tx = controller_tx.clone();
    tree.delimiter().leaf("Import 'persona.tsv'", move |_| {
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
            info_tx
                .send(UiEvent::PresentInfo(format!(
                    "Открытие файла 'persona.tsv' завершилось провалом!"
                )))
                .unwrap();
        }
    })
}
