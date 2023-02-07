pub mod data_container;
pub mod persona;
pub mod tag;
pub trait Represent {
    fn identity(&self) -> String;
}
