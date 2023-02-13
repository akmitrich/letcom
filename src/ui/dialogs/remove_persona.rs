use std::sync::mpsc;

use cursive::{
    views::{Dialog, TextView},
    View,
};

use crate::{
    controller::ControllerSignal,
    data_handler::{make_ref, persona::Persona, Represent},
};

pub fn remove_persona_dialog(
    persona: Persona,
    controller_tx: &mpsc::Sender<ControllerSignal>,
) -> impl View {
    let tx = controller_tx.clone();
    let persona_identity = make_ref(&persona).identity();
    Dialog::around(TextView::new(format!(
        "Вы уверены, что хотите удалить\n{:?}?",
        persona_identity
    )))
    .title(format!("Удаляем {}", persona_identity))
    .button("Yes", move |c| {
        tx.send(ControllerSignal::RemovePersona(persona.clone()))
            .unwrap();
        c.pop_layer();
    })
    .button("No", |c| {
        c.pop_layer();
    })
    .button("Cancel", |c| {
        c.pop_layer();
    })
}
