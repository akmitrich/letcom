use std::collections::BTreeMap;

use lettre::message::SinglePart;

#[derive(Debug)]
pub struct Letter {
    pub topic: String,
    pub text: String,
    pub attachment: BTreeMap<String, SinglePart>,
}

impl Letter {
    pub fn new() -> Self {
        Self {
            topic: "".into(),
            text: "".into(),
            attachment: BTreeMap::new(),
        }
    }
}
