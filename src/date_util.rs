use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDateTime, Timelike, Utc};
use std::convert::TryInto;

use crate::models::timestamp::Timestamp;

pub fn timestamp_seconds_to_date(timestamp: u64) -> Result<DateTime<Utc>> {
    // i64::MAX is in the year 2262, where if this program still exists and is regularly updated, the dependencies should require suitable types.
    // until then we don't expect this to fail (under normal circumstances).
    let timestamp_i64 = timestamp.try_into()?;
    Ok(DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(timestamp_i64, 0),
        Utc,
    ))
}

pub trait DateTimeExt {
    fn to_timestap(&self) -> Timestamp;

    // sets the time component (hour/min/sec/nanosec) to 0
    fn zero_time(&mut self) -> Result<DateTime<Utc>>;
}

impl DateTimeExt for DateTime<Utc> {
    fn to_timestap(&self) -> Timestamp {
        Timestamp(self.timestamp() as u64)
    }

    fn zero_time(&mut self) -> Result<DateTime<Utc>> {
        Ok(self
            .with_hour(0)
            .ok_or_else(|| anyhow!("Couldn't reset hour"))?
            .with_minute(0)
            .ok_or_else(|| anyhow!("Couldn't reset min"))?
            .with_second(0)
            .ok_or_else(|| anyhow!("Couldn't reset sec"))?
            .with_nanosecond(0)
            .ok_or_else(|| anyhow!("Couldn't reset nanosec"))?)
    }
}
