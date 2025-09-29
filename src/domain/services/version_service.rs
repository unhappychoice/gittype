use crate::domain::repositories::VersionRepository;
use crate::{GitTypeError, Result};

pub struct VersionService {
    repository: VersionRepository,
}

impl VersionService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            repository: VersionRepository::new()?,
        })
    }

    /// Check if a new version is available and return both the result and versions
    pub async fn check(&self) -> Result<(bool, String, String)> {
        let current_version = env!("CARGO_PKG_VERSION").to_string();
        self.check_with_version(&current_version).await
    }

    /// Check if a new version is available with a custom current version
    pub async fn check_with_version(
        &self,
        current_version: &str,
    ) -> Result<(bool, String, String)> {
        let current_version = current_version.to_string();
        let latest_version = self.repository.fetch_latest_version().await?;
        let has_update = Self::is_version_newer(&latest_version, &current_version);

        Ok((has_update, current_version, latest_version))
    }

    /// Compare two version strings and determine if the first is newer
    fn is_version_newer(latest: &str, current: &str) -> bool {
        let latest_parts = Self::parse_version(latest);
        let current_parts = Self::parse_version(current);

        match (latest_parts, current_parts) {
            (Ok(latest), Ok(current)) => {
                for (l, c) in latest.iter().zip(current.iter()) {
                    if l > c {
                        return true;
                    } else if l < c {
                        return false;
                    }
                }
                latest.len() > current.len()
            }
            _ => false,
        }
    }

    /// Parse a version string into a vector of numeric parts
    fn parse_version(version: &str) -> Result<Vec<u32>> {
        version
            .split('.')
            .map(|part| {
                part.parse::<u32>().map_err(|e| {
                    GitTypeError::ExtractionFailed(format!("Invalid version format: {}", e))
                })
            })
            .collect()
    }
}
