use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachedFile {
    content_bytes: Vec<u8>,
    filename: String,
    content_type: String,
}
