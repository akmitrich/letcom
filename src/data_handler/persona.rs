use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

pub type Persona = Arc<RwLock<PersonaRepr>>;

#[derive(Debug)]
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

    pub fn update_persona(&mut self, persona: Persona) {
        let key = persona.read().unwrap().identity();
        self.container
            .entry(key)
            .and_modify(|old_persona| *old_persona = Arc::clone(&persona))
            .or_insert(persona);
    }
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
