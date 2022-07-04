use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cid(pub String);

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct CidUrl(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Nft {
    pub url: String,
    pub asset_id: u64,
}
