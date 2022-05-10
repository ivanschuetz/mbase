use algonaut::core::{to_app_address, Address};
use serde::{Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

// TODO consider smart initializer: return error if id is 0 (invalid dao/app id)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct DaoAppId(pub u64);

impl FromStr for DaoAppId {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DaoAppId(s.parse()?))
    }
}
impl ToString for DaoAppId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl DaoAppId {
    pub fn bytes(&self) -> [u8; 8] {
        // note: matches to_le_bytes() in DaoId::from()
        self.0.to_le_bytes()
    }

    pub fn address(&self) -> Address {
        to_app_address(self.0)
    }
}

impl From<[u8; 8]> for DaoAppId {
    fn from(slice: [u8; 8]) -> Self {
        // note: matches to_le_bytes() in DaoId::bytes()
        DaoAppId(u64::from_le_bytes(slice))
    }
}

impl TryFrom<&[u8]> for DaoAppId {
    type Error = anyhow::Error;
    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let array: [u8; 8] = slice.try_into()?;
        Ok(array.into())
    }
}
