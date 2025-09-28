#[cfg(not(feature = "test-mocks"))]
use crate::{GitTypeError, Result};
#[cfg(feature = "test-mocks")]
use crate::Result;
use super::AppDataProvider;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[cfg(not(feature = "test-mocks"))]
mod real_impl {
    use super::*;

    pub struct FileStorage;

    impl AppDataProvider for FileStorage {}

    impl FileStorage {

        /// Read and deserialize JSON from a file
        pub fn read_json<T>(&self, file_path: &Path) -> Result<Option<T>>
        where
            T: for<'de> Deserialize<'de>,
        {
            if !file_path.exists() {
                return Ok(None);
            }

            let contents = std::fs::read_to_string(file_path)?;
            let data: T = serde_json::from_str(&contents).map_err(|e| {
                GitTypeError::ExtractionFailed(format!(
                    "Failed to parse JSON file {}: {}",
                    file_path.display(),
                    e
                ))
            })?;
            Ok(Some(data))
        }

        /// Serialize and write JSON to a file
        pub fn write_json<T>(&self, file_path: &Path, data: &T) -> Result<()>
        where
            T: Serialize,
        {
            // Ensure parent directory exists
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let contents = serde_json::to_string_pretty(data).map_err(|e| {
                GitTypeError::ExtractionFailed(format!(
                    "Failed to serialize data for {}: {}",
                    file_path.display(),
                    e
                ))
            })?;
            std::fs::write(file_path, contents)?;
            Ok(())
        }

        /// Delete a file if it exists
        pub fn delete_file(&self, file_path: &Path) -> Result<()> {
            if file_path.exists() {
                std::fs::remove_file(file_path)?;
            }
            Ok(())
        }

        /// Check if a file exists
        pub fn file_exists(&self, file_path: &Path) -> bool {
            file_path.exists()
        }
    }
}

#[cfg(feature = "test-mocks")]
mod mock_impl {
    use super::*;

    pub struct FileStorage;

    impl AppDataProvider for FileStorage {}

    impl FileStorage {

        pub fn read_json<T>(&self, _file_path: &Path) -> Result<Option<T>>
        where
            T: for<'de> Deserialize<'de>,
        {
            // For test mocks, just return None (no cache)
            Ok(None)
        }

        pub fn write_json<T>(&self, _file_path: &Path, _data: &T) -> Result<()>
        where
            T: Serialize,
        {
            // For test mocks, just succeed
            Ok(())
        }

        pub fn delete_file(&self, _file_path: &Path) -> Result<()> {
            Ok(())
        }

        pub fn file_exists(&self, _file_path: &Path) -> bool {
            false
        }
    }
}

#[cfg(not(feature = "test-mocks"))]
pub use real_impl::FileStorage;

#[cfg(feature = "test-mocks")]
pub use mock_impl::FileStorage;
