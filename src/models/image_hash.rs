use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Assumes string to be base64 encoded hash bytes
/// we might change this in the future to store and handle directly the hash bytes (similar to Algonaut's HashDigest struct)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageHash(pub String);

impl ImageHash {
    pub fn bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<ImageHash> {
        Ok(ImageHash(String::from_utf8(bytes)?))
    }

    pub fn as_str(&self) -> String {
        self.0.clone()
    }

    pub fn as_api_id(&self) -> String {
        self.0.clone()
    }

    // TODO add in core only
    // pub fn as_api_url(&self, image_api: &dyn ImageApi) -> String {
    //     image_api.image_url(&self.as_api_id())
    // }
}
