use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};

pub type Persona = Arc<RwLock<PersonaRepr>>;

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

    pub fn identity(&self) -> String {
        format!("{} {} {}", self.family, self.name, self.surname)
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}

#[derive(Debug, Default)]
pub struct PersonaContainer {
    container: BTreeMap<String, Persona>,
}

impl PersonaContainer {
    pub fn len(&self) -> usize {
        self.container.len()
    }

    pub fn all_identities(&self) -> Vec<&String> {
        self.container.keys().collect()
    }

    pub fn all_persona(&self) -> impl Iterator<Item = Persona> + '_ {
        self.container.values().cloned()
    }

    pub fn update_persona(&mut self, persona: Persona) {
        let key = persona.read().unwrap().identity();
        self.container
            .entry(key)
            .and_modify(|old_persona| *old_persona = Arc::clone(&persona))
            .or_insert(persona);
    }

    pub fn from_json(json: &str) -> Option<Self> {
        Some(Self {
            container: serde_json::from_str(json).ok()?,
        })
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.container).unwrap()
    }
}

pub fn new_persona() -> Persona {
    Arc::new(RwLock::new(PersonaRepr::new("", "", "", "")))
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
    Some(Arc::new(RwLock::new(PersonaRepr::new(
        data.get(FAMILY)?.trim(),
        data.get(NAME)?.trim(),
        data.get(SURNAME)?.trim(),
        data.get(EMAIL1)?.trim(),
    ))))
}

pub fn restore_persona(path: impl AsRef<Path>) -> Option<PersonaContainer> {
    let json = fs::read_to_string(path).ok()?;
    PersonaContainer::from_json(&json)
}
