pub trait SetData {
    fn set_data(&mut self, data: impl Iterator<Item = String>);
}

pub mod open_file;
pub mod remove_persona;
