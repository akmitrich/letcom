use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachedFile {
    content_bytes: Vec<u8>,
    filename: String,
    content_type: String,
}

impl AttachedFile {
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let bytes = fs::read(path.as_ref())?;
        let filename = path.as_ref().file_name().unwrap().to_str().unwrap();
        Ok(Self {
            content_bytes: bytes,
            filename: filename.to_owned(),
            content_type: "application/octet-stream".to_owned(),
        })
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
    }

    pub fn get_size(&self) -> usize {
        self.content_bytes.len()
    }
}
