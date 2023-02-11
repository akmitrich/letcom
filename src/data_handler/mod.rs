pub mod attached_file;
pub mod data_container;
pub mod letter;
pub mod persona;
pub mod tag;

pub type Identity = String;

pub trait Represent {
    fn identity(&self) -> Identity;
}
