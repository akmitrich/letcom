use std::{
    cell::RefCell,
    collections::BTreeMap,
    fs,
    io::{self, Write},
    path::Path,
    rc::Rc,
};

use serde::{de::DeserializeOwned, Serialize};

use super::{make_ref, Identity, Represent};

#[derive(Debug)]
pub struct DataContainer<Repr> {
    container: BTreeMap<Identity, Rc<RefCell<Repr>>>,
}

impl<Repr> DataContainer<Repr> {
    pub fn new() -> Self {
        Self {
            container: BTreeMap::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.container.len()
    }

    pub fn idendities(&self) -> impl Iterator<Item = &Identity> + '_ {
        self.container.keys()
    }

    pub fn all_representations(&self) -> impl Iterator<Item = Rc<RefCell<Repr>>> + '_ {
        self.container.values().cloned()
    }
}

impl<Repr> Default for DataContainer<Repr> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Repr: Represent> DataContainer<Repr> {
    pub fn insert_or_update(&mut self, repr: Rc<RefCell<Repr>>) {
        let repr_id = make_ref(&repr).identity();
        self.update_identity(repr_id, repr);
    }

    pub fn update_identity(&mut self, identity: impl AsRef<str>, repr: Rc<RefCell<Repr>>) {
        let key = identity.as_ref();
        let repr_id = make_ref(&repr).identity();
        if repr_id != key {
            self.container.remove(key);
        }
        self.container
            .entry(repr_id)
            .and_modify(|old| *old = Rc::clone(&repr))
            .or_insert(repr);
    }

    pub fn remove_identity(&mut self, identity: impl AsRef<str>) -> Option<Rc<RefCell<Repr>>> {
        self.container.remove(identity.as_ref())
    }

    pub fn remove_representation(&mut self, repr: Rc<RefCell<Repr>>) -> Option<Rc<RefCell<Repr>>> {
        self.remove_identity(make_ref(&repr).identity())
    }
}

impl<Repr: Serialize> DataContainer<Repr> {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.container.values().collect::<Vec<_>>()).unwrap()
    }

    pub fn finalize(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let mut file = fs::File::create(path)?;
        write!(file, "{}", self.to_json())?;
        Ok(())
    }
}

impl<Repr: DeserializeOwned + Represent> DataContainer<Repr> {
    pub fn from_json(json: impl AsRef<str>) -> serde_json::Result<Self> {
        let data: Vec<Rc<RefCell<Repr>>> = serde_json::from_str(json.as_ref())?;
        Ok(Self {
            container: data
                .into_iter()
                .map(|x| (make_ref(&x).identity(), Rc::clone(&x)))
                .collect(),
        })
    }

    pub fn restore(path: impl AsRef<Path>) -> io::Result<Self> {
        let json = fs::read_to_string(path)?;
        DataContainer::from_json(&json)
            .map_err(|json_err| io::Error::new(io::ErrorKind::InvalidData, json_err))
    }
}
