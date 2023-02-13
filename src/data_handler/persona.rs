use std::{cell::RefCell, fs, path::Path, rc::Rc};

use serde::{Deserialize, Serialize};

use super::{data_container::DataContainer, Identity, Represent};

pub type Persona = Rc<RefCell<PersonaRepr>>;
pub type PersonaContainer = DataContainer<PersonaRepr>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonaRepr {
    family: String,
    name: String,
    surname: String,
    email: String,
}

impl PersonaRepr {
    pub fn new(family: &str, name: &str, surname: &str, email: &str) -> Self {
        Self {
            family: family.into(),
            name: name.into(),
            surname: surname.into(),
            email: email.into(),
        }
    }

    pub fn get_family(&self) -> &str {
        &self.family
    }

    pub fn set_family(&mut self, family: String) {
        self.family = family;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_surname(&self) -> &str {
        &self.surname
    }

    pub fn set_surname(&mut self, surname: String) {
        self.surname = surname;
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }
}

impl Represent for PersonaRepr {
    fn identity(&self) -> Identity {
        format!("{} {} {}", self.family, self.name, self.surname)
    }
}

pub fn new_persona() -> Persona {
    Rc::new(RefCell::new(PersonaRepr::new("", "", "", "")))
}

pub fn import_persona(data: &str) -> Option<Persona> {
    const FAMILY: usize = 0;
    const NAME: usize = 1;
    const SURNAME: usize = 2;
    const _POSITION: usize = 4;
    const _DEGREE: usize = 5;
    const _BIRTHDATE: usize = 9;
    const _SHT_SOVM: usize = 12;
    const _STAVKA: usize = 13;
    const _PHONE: usize = 14;
    const EMAIL1: usize = 15;
    const _EMAIL2: usize = 16;
    const _EMAIL3: usize = 17;
    let data = data.split('\t').collect::<Vec<_>>();
    Some(Rc::new(RefCell::new(PersonaRepr::new(
        data.get(FAMILY)?.trim(),
        data.get(NAME)?.trim(),
        data.get(SURNAME)?.trim(),
        data.get(EMAIL1)?.trim(),
    ))))
}

pub fn restore_persona_container(path: impl AsRef<Path>) -> Option<PersonaContainer> {
    let json = fs::read_to_string(path).ok()?;
    PersonaContainer::from_json(&json)
}
