use std::{
    collections::BTreeMap,
    fs,
    io::Write,
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

    pub fn remove(&mut self, persona: Persona) {
        self.container.remove(&persona.read().unwrap().identity());
    }

    pub fn finalize_persona_container(&self, path: impl AsRef<Path>) {
        if let Ok(mut file) = fs::File::create(path) {
            write!(file, "{}", self.to_json()).unwrap()
        };
    }

    pub fn from_json(json: &str) -> Option<Self> {
        let persona: Vec<Persona> = serde_json::from_str(json).ok()?;
        Some(Self {
            container: persona
                .into_iter()
                .map(|person| (person.clone().read().unwrap().identity(), person))
                .collect(),
        })
    }

    pub fn to_json(&self) -> String {
        let persona = self.container.values().collect::<Vec<_>>();
        serde_json::to_string(&persona).unwrap()
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

pub fn restore_persona_container(path: impl AsRef<Path>) -> Option<PersonaContainer> {
    let json = fs::read_to_string(path).ok()?;
    PersonaContainer::from_json(&json)
}
