use crate::checked::{CheckedAdd, CheckedMulOther, CheckedSub};

use super::asset_amount::AssetAmount;
use anyhow::Result;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::Display;

/// An amount of shares (DAO ASA)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareAmount(pub AssetAmount);

impl ShareAmount {
    pub fn new(amount: u64) -> ShareAmount {
        ShareAmount(AssetAmount(amount))
    }

    pub fn as_decimal(&self) -> Decimal {
        self.0.as_decimal()
    }

    pub fn val(&self) -> u64 {
        self.0 .0
    }
}

impl Display for ShareAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val())
    }
}

impl PartialOrd for ShareAmount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl From<AssetAmount> for ShareAmount {
    fn from(amount: AssetAmount) -> Self {
        ShareAmount(amount)
    }
}

impl CheckedAdd for ShareAmount {
    fn add(&self, o: &Self) -> Result<Self> {
        Ok(ShareAmount(self.0.add(&o.0)?))
    }
}

impl CheckedSub for ShareAmount {
    fn sub(&self, o: &Self) -> Result<Self> {
        Ok(ShareAmount(self.0.sub(&o.0)?))
    }
}

impl CheckedMulOther<u64> for ShareAmount {
    fn mul(self, rhs: u64) -> Result<Self> {
        Ok(ShareAmount(self.0.mul(rhs)?))
    }
}
