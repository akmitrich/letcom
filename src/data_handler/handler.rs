use std::io;

use super::{letter::LetterContainer, persona::PersonaContainer, tag::TagContainer};

const PERSONA_CONTAINER_PATH: &str = "persona.json";
const TAG_CONTAINER_PATH: &str = "tag.json";

#[derive(Debug)]
pub struct DataHandler {
    people: Option<PersonaContainer>,
    tags: Option<TagContainer>,
    _letters: Option<LetterContainer>,
}

impl DataHandler {
    pub fn new() -> Self {
        Self {
            people: None,
            tags: None,
            _letters: None,
        }
    }

    pub fn get_people(&mut self) -> &PersonaContainer {
        match self.people {
            Some(ref people) => people,
            None => {
                self.people = Some(Self::restore_people().unwrap_or_default());
                self.get_people()
            }
        }
    }

    pub fn get_people_mut(&mut self) -> &mut PersonaContainer {
        match self.people {
            Some(ref mut people) => people,
            None => {
                self.people = Some(Self::restore_people().unwrap_or_default());
                self.get_people_mut()
            }
        }
    }

    pub fn get_tags(&mut self) -> &TagContainer {
        match self.tags {
            Some(ref tags) => tags,
            None => {
                self.tags = Some(Self::restore_tags().unwrap_or_default());
                self.get_tags()
            }
        }
    }

    pub fn get_tags_mut(&mut self) -> &mut TagContainer {
        match self.tags {
            Some(ref mut tags) => tags,
            None => {
                self.tags = Some(Self::restore_tags().unwrap_or_default());
                self.get_tags_mut()
            }
        }
    }

    fn restore_people() -> io::Result<PersonaContainer> {
        PersonaContainer::restore(PERSONA_CONTAINER_PATH)
    }

    fn restore_tags() -> io::Result<TagContainer> {
        TagContainer::restore(TAG_CONTAINER_PATH)
    }

    fn _restore_letters() -> io::Result<LetterContainer> {
        todo!()
    }

    pub fn finalize(&mut self) {
        if let Some(ref people) = self.people {
            people.finalize(PERSONA_CONTAINER_PATH).unwrap();
        }
        if let Some(ref tags) = self.tags {
            tags.finalize(TAG_CONTAINER_PATH).unwrap();
        }
    }
}
