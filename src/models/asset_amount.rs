use crate::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedMulOther, CheckedSub};
use crate::util::decimal_util::AsDecimal;
use anyhow::{anyhow, Result};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::cmp::PartialOrd;
use std::fmt::Display;

/// An amount of assets (ASA)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct AssetAmount(pub u64);

impl AssetAmount {
    pub fn as_decimal(&self) -> Decimal {
        self.0.as_decimal()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

impl Display for AssetAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<u64> for AssetAmount {
    fn eq(&self, other: &u64) -> bool {
        &self.0 == other
    }
}

impl PartialOrd<u64> for AssetAmount {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl CheckedAdd for AssetAmount {
    fn add(&self, v: &Self) -> Result<Self> {
        Ok(AssetAmount(
            self.0
                .checked_add(v.0)
                .ok_or(anyhow!("Failed: {self} + {v}".to_owned()))?,
        ))
    }
}

impl CheckedSub for AssetAmount {
    fn sub(&self, v: &Self) -> Result<Self> {
        Ok(AssetAmount(
            self.0
                .checked_sub(v.0)
                .ok_or(anyhow!("Failed: {self} - {v}".to_owned()))?,
        ))
    }
}

impl CheckedMul for AssetAmount {
    fn mul(&self, v: &Self) -> Result<Self> {
        Ok(AssetAmount(
            self.0
                .checked_mul(v.0)
                .ok_or(anyhow!("Failed: {self} * {v}".to_owned()))?,
        ))
    }
}

impl CheckedDiv for AssetAmount {
    fn div(&self, v: &Self) -> Result<Self> {
        Ok(AssetAmount(
            self.0
                .checked_div(v.0)
                .ok_or(anyhow!("Failed: {self} / {v}".to_owned()))?,
        ))
    }
}

impl CheckedMulOther<u64> for AssetAmount {
    fn mul(self, rhs: u64) -> Result<Self> {
        Ok(AssetAmount(
            self.0
                .checked_mul(rhs)
                .ok_or(anyhow!("Failed: {self} * {v}".to_owned()))?,
        ))
    }
}
