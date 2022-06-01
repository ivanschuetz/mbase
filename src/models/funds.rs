use super::asset_amount::AssetAmount;
use crate::{
    checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedMulOther, CheckedSub},
    util::decimal_util::AsDecimal,
};
use anyhow::Result;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FundsAmount(pub AssetAmount);

impl Display for FundsAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val())
    }
}

impl FundsAmount {
    pub fn new(amount: u64) -> FundsAmount {
        FundsAmount(AssetAmount(amount))
    }

    pub fn as_decimal(&self) -> Decimal {
        self.0 .0.as_decimal()
    }

    pub fn val(&self) -> u64 {
        self.0 .0
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl CheckedAdd for FundsAmount {
    fn add(&self, o: &Self) -> Result<Self> {
        Ok(FundsAmount(self.0.add(&o.0)?))
    }
}

impl CheckedSub for FundsAmount {
    fn sub(&self, o: &Self) -> Result<Self> {
        Ok(FundsAmount(self.0.sub(&o.0)?))
    }
}

impl CheckedMul for FundsAmount {
    fn mul(&self, o: &Self) -> Result<Self> {
        Ok(FundsAmount(<AssetAmount as CheckedMul>::mul(
            &self.0, &o.0,
        )?))
    }
}

impl CheckedDiv for FundsAmount {
    fn div(&self, o: &Self) -> Result<Self> {
        Ok(FundsAmount(self.0.div(&o.0)?))
    }
}

impl CheckedMulOther<u64> for FundsAmount {
    fn mul(self, rhs: u64) -> Result<Self> {
        Ok(FundsAmount(self.0.mul(rhs)?))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FundsAssetId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Funds {
    pub asset_id: FundsAssetId,
    pub amount: FundsAmount,
}
