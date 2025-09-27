#[cfg(not(feature = "test-mocks"))]
use crate::{GitTypeError, Result};
#[cfg(feature = "test-mocks")]
use crate::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GitHubRelease {
    pub tag_name: String,
}

#[cfg(not(feature = "test-mocks"))]
mod real_impl {
    use super::*;

    pub struct GitHubApiClient {
        client: reqwest::Client,
    }

    impl GitHubApiClient {
        pub fn new() -> Result<Self> {
            let client = reqwest::Client::builder()
                .user_agent("gittype")
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .map_err(|e| GitTypeError::ExtractionFailed(format!("Failed to create HTTP client: {}", e)))?;

            Ok(Self { client })
        }

        pub async fn fetch_latest_release(&self) -> Result<GitHubRelease> {
            let url = "https://api.github.com/repos/unhappychoice/gittype/releases/latest";
            let response = self
                .client
                .get(url)
                .send()
                .await
                .map_err(|e| GitTypeError::ExtractionFailed(format!("Failed to fetch release: {}", e)))?;

            if !response.status().is_success() {
                return Err(GitTypeError::ExtractionFailed(format!(
                    "GitHub API request failed with status: {}",
                    response.status()
                )));
            }

            let release: GitHubRelease = response
                .json()
                .await
                .map_err(|e| GitTypeError::ExtractionFailed(format!("Failed to parse JSON: {}", e)))?;

            Ok(release)
        }
    }
}

#[cfg(feature = "test-mocks")]
mod mock_impl {
    use super::*;

    pub struct GitHubApiClient;

    impl GitHubApiClient {
        pub fn new() -> Result<Self> {
            Ok(Self)
        }

        pub async fn fetch_latest_release(&self) -> Result<GitHubRelease> {
            Ok(GitHubRelease {
                tag_name: "v1.0.0".to_string(),
            })
        }
    }
}

#[cfg(not(feature = "test-mocks"))]
pub use real_impl::GitHubApiClient;

#[cfg(feature = "test-mocks")]
pub use mock_impl::GitHubApiClient;
