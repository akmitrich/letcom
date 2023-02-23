use crate::data_handler::{letter::Letter, persona::Persona, tag::Tag, Identity};

#[derive(Debug)]
pub enum ControllerSignal {
    Noop,
    Log(String),
    OpenSettings,
    SaveSettings,
    NewTag,
    CompleteEditTag { key: Identity, tag: Tag },
    NewLetter,
    EditLetter(Letter),
    CompleteEditLetter { key: Identity, letter: Letter },
    OpenLetterToSend(Letter),
    SendEmail { letter: Letter, to: Vec<String> },
    ImportPersona(Vec<Persona>),
    SelectPersona,
    EditPersona(Persona),
    CompleteEditPersona { key: Identity, persona: Persona },
    RemovePersonaAlert(Persona),
    RemovePersona(Persona),
    Quit,
}
