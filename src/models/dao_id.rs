use super::dao_app_id::DaoAppId;
use serde::{Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct DaoId(pub DaoAppId);
impl DaoId {
    pub fn bytes(&self) -> [u8; 8] {
        self.0.bytes()
    }
}

impl FromStr for DaoId {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let app_id: DaoAppId = s.parse()?;
        Ok(DaoId(app_id))
    }
}
impl ToString for DaoId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl TryFrom<&[u8]> for DaoId {
    type Error = anyhow::Error;
    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let app_id: DaoAppId = slice.try_into()?;
        Ok(DaoId(app_id))
    }
}
