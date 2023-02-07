use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::Path,
    sync::{Arc, RwLock},
};

use serde::{de::DeserializeOwned, Serialize};

use super::Represent;

#[derive(Debug)]
pub struct DataContainer<Repr> {
    container: BTreeMap<String, Arc<RwLock<Repr>>>,
}

impl<Repr> DataContainer<Repr> {
    pub fn new() -> Self {
        Self {
            container: BTreeMap::new(),
        }
    }
}

impl<Repr> Default for DataContainer<Repr> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Repr: Represent> DataContainer<Repr> {
    pub fn update(&mut self, identity: impl AsRef<str>, repr: Arc<RwLock<Repr>>) {
        let key = identity.as_ref();
        let repr_id = repr.read().unwrap().identity();
        if repr_id != key {
            self.container.remove(key);
        }
        self.container
            .entry(repr_id)
            .and_modify(|old| *old = Arc::clone(&repr))
            .or_insert(repr);
    }

    pub fn remove_identity(&mut self, identity: impl AsRef<str>) -> Option<Arc<RwLock<Repr>>> {
        self.container.remove(identity.as_ref())
    }

    pub fn remove_representation(&mut self, repr: Arc<RwLock<Repr>>) -> Option<Arc<RwLock<Repr>>> {
        self.remove_identity(repr.read().unwrap().identity())
    }
}

impl<Repr: Serialize> DataContainer<Repr> {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.container.values().collect::<Vec<_>>()).unwrap()
    }

    pub fn finalize(&self, path: impl AsRef<Path>) {
        if let Ok(mut file) = fs::File::create(path) {
            write!(file, "{}", self.to_json()).unwrap_or_default();
        }
    }
}

impl<Repr: DeserializeOwned + Represent> DataContainer<Repr> {
    pub fn from_json(json: impl AsRef<str>) -> Option<Self> {
        let data: Vec<Arc<RwLock<Repr>>> = serde_json::from_str(json.as_ref()).ok()?;
        Some(Self {
            container: data
                .into_iter()
                .map(|x| (x.read().unwrap().identity(), Arc::clone(&x)))
                .collect(),
        })
    }
}
pub fn restore<Repr>(path: impl AsRef<Path>) -> Option<DataContainer<Repr>>
where
    Repr: DeserializeOwned + Represent,
{
    let json = fs::read_to_string(path).ok()?;
    DataContainer::from_json(&json)
}
