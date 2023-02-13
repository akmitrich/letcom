use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub mod attached_file;
pub mod data_container;
pub mod handler;
pub mod letter;
pub mod persona;
pub mod tag;

pub type Identity = String;

pub trait Represent {
    fn identity(&self) -> Identity;
}

pub fn make_ref<'a, T>(x: &'a Rc<RefCell<T>>) -> impl Deref<Target = T> + 'a {
    x.borrow()
}

pub fn make_mut<'a, T>(x: &'a Rc<RefCell<T>>) -> impl DerefMut<Target = T> + 'a {
    x.borrow_mut()
}
