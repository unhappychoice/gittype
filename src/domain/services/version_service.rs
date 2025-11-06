use crate::domain::repositories::version_repository::VersionRepositoryInterface;
use crate::{GitTypeError, Result};
use shaku::Interface;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type VersionCheckFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(bool, String, String)>> + Send + 'a>>;

pub trait VersionServiceInterface: Interface {
    fn check(&self) -> VersionCheckFuture<'_>;
    fn check_with_version(&self, current_version: &str) -> VersionCheckFuture<'_>;
}

#[derive(shaku::Component)]
#[shaku(interface = VersionServiceInterface)]
pub struct VersionService {
    #[shaku(inject)]
    repository: Arc<dyn VersionRepositoryInterface>,
}

impl VersionServiceInterface for VersionService {
    fn check(&self) -> Pin<Box<dyn Future<Output = Result<(bool, String, String)>> + Send + '_>> {
        Box::pin(async move {
            let current_version = env!("CARGO_PKG_VERSION").to_string();
            self.check_with_version(&current_version).await
        })
    }

    fn check_with_version(
        &self,
        current_version: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(bool, String, String)>> + Send + '_>> {
        let current_version = current_version.to_string();
        Box::pin(async move {
            let latest_version = self.repository.fetch_latest_version().await?;
            let has_update = VersionService::is_version_newer(&latest_version, &current_version);
            Ok((has_update, current_version, latest_version))
        })
    }
}

impl VersionService {
    #[cfg(feature = "test-mocks")]
    pub fn new_for_test() -> Result<Self> {
        use crate::domain::repositories::version_repository::VersionRepository;
        Ok(Self {
            repository: Arc::new(VersionRepository::new_for_test()?),
        })
    }

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
