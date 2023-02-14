use std::{cell::RefCell, io, path::Path, rc::Rc};

use serde::{Deserialize, Serialize};

use super::{attached_file::AttachedFile, data_container::DataContainer, Identity, Represent};

pub type LetterContainer = DataContainer<LetterRepr>;
pub type Letter = Rc<RefCell<LetterRepr>>;
pub type DateTime = chrono::DateTime<chrono::Local>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LetterRepr {
    time: DateTime,
    topic: String,
    text: String,
    attachment: Vec<AttachedFile>,
}
impl LetterRepr {
    pub fn new() -> Self {
        let now = chrono::Local::now();
        Self {
            time: now,
            topic: String::new(),
            text: String::new(),
            attachment: vec![],
        }
    }

    pub fn get_time(&self) -> &DateTime {
        &self.time
    }

    pub fn get_topic(&self) -> &str {
        &self.topic
    }

    pub fn set_topic(&mut self, topic: impl ToString) {
        self.topic = topic.to_string()
    }

    pub fn set_text(&mut self, text: impl ToString) {
        self.text = text.to_string()
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn attachment_info(&self) -> String {
        let info = self
            .attachment
            .iter()
            .map(|attached_file| {
                format!(
                    "[{} ({} байт)]",
                    attached_file.get_filename(),
                    attached_file.get_size()
                )
            })
            .collect::<Vec<_>>();
        if info.is_empty() {
            "[Вложений нет]".to_string()
        } else {
            info.join(" ")
        }
    }

    pub fn add_attachment_from_path(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let attached_file = AttachedFile::from_path(path)?;
        self.attachment.push(attached_file);
        Ok(())
    }

    pub fn clear_attachment(&mut self) {
        self.attachment.clear();
    }
}

impl Represent for LetterRepr {
    fn identity(&self) -> Identity {
        self.time.to_rfc3339()
    }
}

pub fn new_letter() -> Letter {
    Rc::new(RefCell::new(LetterRepr::new()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_handler::data_container::DataContainer;

    #[test]
    fn test_new_letter_in_data_container() {
        let mut container = DataContainer::<LetterRepr>::new();
        let letter = new_letter();
        container.insert_or_update(letter);
        let time = container.idendities().next().unwrap();
        let letter = container.all_representations().next().unwrap();
        assert_eq!(time, &letter.borrow().identity());
    }
}
