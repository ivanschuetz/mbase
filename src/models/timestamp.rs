use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Unix timestamp (seconds)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timestamp(pub u64);

impl From<DateTime<Utc>> for Timestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        Timestamp(dt.timestamp() as u64)
    }
}
