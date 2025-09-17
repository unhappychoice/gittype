use chrono::{Duration, Utc};
use gittype::version::{VersionCache, VersionCacheEntry, VersionChecker};

#[test]
fn cache_valid_when_recent_and_version_matches() {
    let frequency_hours = 24;
    let entry = VersionCacheEntry {
        latest_version: "9.9.9".to_string(),
        current_version: VersionChecker::CURRENT_VERSION.to_string(),
        update_available: true,
        last_checked: Utc::now() - Duration::hours(1),
    };

    assert!(VersionCache::is_cache_valid(&entry, frequency_hours));
}

#[test]
fn cache_invalid_when_stale() {
    let frequency_hours = 24;
    let entry = VersionCacheEntry {
        latest_version: "9.9.9".to_string(),
        current_version: VersionChecker::CURRENT_VERSION.to_string(),
        update_available: true,
        last_checked: Utc::now() - Duration::hours(frequency_hours as i64 + 1),
    };

    assert!(!VersionCache::is_cache_valid(&entry, frequency_hours));
}

#[test]
fn cache_invalid_when_version_mismatch() {
    let frequency_hours = 24;
    let entry = VersionCacheEntry {
        latest_version: "9.9.9".to_string(),
        current_version: format!("{}-dev", VersionChecker::CURRENT_VERSION),
        update_available: true,
        last_checked: Utc::now(),
    };

    assert!(!VersionCache::is_cache_valid(&entry, frequency_hours));
}
