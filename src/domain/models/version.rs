use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionCacheEntry {
    pub latest_version: String,
    pub current_version: String,
    pub update_available: bool,
    pub last_checked: DateTime<Utc>,
}
