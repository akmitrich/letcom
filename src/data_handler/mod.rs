use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use self::{letter::LetterContainer, persona::PersonaContainer, tag::TagContainer};

pub mod attached_file;
pub mod data_container;
pub mod letter;
pub mod persona;
pub mod tag;

pub type Identity = String;

pub trait Represent {
    fn identity(&self) -> Identity;
}

#[derive(Debug)]
pub struct DataHandler {
    people: PersonaContainer,
    tags: TagContainer,
    letters: LetterContainer,
}

impl DataHandler {
    pub fn restore() -> Self {
        Self {
            people: Default::default(),
            tags: Default::default(),
            letters: Default::default(),
        }
    }
}

pub fn make_ref<'a, T>(x: &'a Rc<RefCell<T>>) -> impl Deref<Target = T> + 'a {
    x.borrow()
}

pub fn make_mut<'a, T>(x: &'a Rc<RefCell<T>>) -> impl DerefMut<Target = T> + 'a {
    x.borrow_mut()
}
