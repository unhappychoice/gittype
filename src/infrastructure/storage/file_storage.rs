use super::AppDataProvider;
#[cfg(feature = "test-mocks")]
use crate::Result;
#[cfg(not(feature = "test-mocks"))]
use crate::{GitTypeError, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub is_file: bool,
}

#[cfg(not(feature = "test-mocks"))]
mod real_impl {
    use super::*;

    pub struct FileStorage;

    impl AppDataProvider for FileStorage {}

    impl Default for FileStorage {
        fn default() -> Self {
            Self::new()
        }
    }

    impl FileStorage {
        pub fn new() -> Self {
            Self
        }

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

        /// Walk directory and return file entries
        pub fn walk_directory(&self, path: &Path) -> Result<Vec<FileEntry>> {
            use ignore::WalkBuilder;

            if !path.exists() {
                return Err(GitTypeError::ExtractionFailed(format!(
                    "Path does not exist: {}",
                    path.display()
                )));
            }

            let walker = WalkBuilder::new(path)
                .hidden(false)
                .git_ignore(true)
                .git_global(true)
                .git_exclude(true)
                .build();

            walker
                .map(|entry| {
                    entry
                        .map_err(|e| GitTypeError::ExtractionFailed(format!("Walk error: {}", e)))
                        .map(|entry| FileEntry {
                            path: entry.path().to_path_buf(),
                            is_file: entry.path().is_file(),
                        })
                })
                .collect()
        }
    }
}

#[cfg(feature = "test-mocks")]
mod mock_impl {
    use super::*;
    use crate::GitTypeError;

    pub struct FileStorage {
        pub files: Vec<FileEntry>,
    }

    impl AppDataProvider for FileStorage {}

    impl Default for FileStorage {
        fn default() -> Self {
            Self::new()
        }
    }

    impl FileStorage {
        pub fn new() -> Self {
            Self { files: Vec::new() }
        }

        pub fn add_file<P: Into<PathBuf>>(&mut self, path: P) {
            self.files.push(FileEntry {
                path: path.into(),
                is_file: true,
            });
        }

        pub fn add_directory<P: Into<PathBuf>>(&mut self, path: P) {
            self.files.push(FileEntry {
                path: path.into(),
                is_file: false,
            });
        }

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

        pub fn walk_directory(&self, path: &Path) -> Result<Vec<FileEntry>> {
            // For mock implementation, simulate real behavior for non-existent paths
            if path.to_str() == Some("/nonexistent/path") {
                return Err(GitTypeError::ExtractionFailed(format!(
                    "Path does not exist: {}",
                    path.display()
                )));
            }
            Ok(self.files.clone())
        }
    }
}

#[cfg(not(feature = "test-mocks"))]
pub use real_impl::FileStorage;

#[cfg(feature = "test-mocks")]
pub use mock_impl::FileStorage;
