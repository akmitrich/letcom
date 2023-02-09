use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use lettre::message::SinglePart;

pub type Letter = Rc<RefCell<LetterRepr>>;

#[derive(Debug)]
pub struct LetterRepr {
    pub topic: String,
    pub text: String,
    pub attachment: BTreeMap<String, SinglePart>,
}

impl LetterRepr {
    pub fn new() -> Self {
        Self {
            topic: "".into(),
            text: "".into(),
            attachment: BTreeMap::new(),
        }
    }

    pub fn attachment_info(&self) -> String {
        let mut info = vec![];
        for (a, b) in self.attachment.iter() {
            info.push(format!("['{}' ({} байт)]", a, b.raw_body().len()));
        }
        if info.is_empty() {
            "Вложений нет.".to_owned()
        } else {
            format!("Вложения: {}.", info.join(", "))
        }
    }
}

pub fn create_new_letter() -> Letter {
    Rc::new(RefCell::new(LetterRepr::new()))
}
