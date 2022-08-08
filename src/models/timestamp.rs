use std::convert::TryInto;

use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

/// Unix timestamp (seconds)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timestamp(pub u64);

impl From<DateTime<Utc>> for Timestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        Timestamp(dt.timestamp() as u64)
    }
}

impl Timestamp {
    pub fn to_date(&self) -> Result<DateTime<Utc>> {
        let timestamp_i64 = self.0.try_into()?;
        let naive = NaiveDateTime::from_timestamp(timestamp_i64, 0);
        Ok(DateTime::from_utc(naive, Utc))
    }

    pub fn now() -> Timestamp {
        Utc::now().into()
    }
}
