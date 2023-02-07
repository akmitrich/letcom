use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use super::{data_container::DataContainer, persona::Persona, Represent};

pub type TagContainer = DataContainer<TagRepr>;
pub type Tag = Arc<RwLock<TagRepr>>;
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
    Arc::new(RwLock::new(TagRepr::new(label)))
}
