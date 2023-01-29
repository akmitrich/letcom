use std::collections::BTreeMap;

use lettre::message::SinglePart;

#[derive(Debug, Clone)]
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

    pub fn attachment_description(&self) -> String {
        let mut description = vec![];
        for (a, b) in self.attachment.iter() {
            description.push(format!("['{}' ({} байт)]", a, b.raw_body().len()));
        }
        if description.is_empty() {
            "Вложений нет.".to_owned()
        } else {
            format!("Вложения: {}.", description.join(", "))
        }
    }
}
