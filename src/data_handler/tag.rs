use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use super::{data_container::DataContainer, persona::Persona, Represent};

pub type TagContainer = DataContainer<TagRepr>;
pub type Tag = Rc<RefCell<TagRepr>>;
#[derive(Debug, Serialize, Deserialize)]
pub struct TagRepr {
    label: String,
    persona: Vec<Persona>,
}

impl TagRepr {
    pub fn new(label: impl ToString) -> Self {
        Self {
            label: label.to_string(),
            persona: Vec::new(),
        }
    }
}

impl Represent for TagRepr {
    fn identity(&self) -> String {
        self.label.to_owned()
    }
}

pub fn new_tag(label: impl ToString) -> Tag {
    Rc::new(RefCell::new(TagRepr::new(label)))
}
