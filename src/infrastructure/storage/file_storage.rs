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

    #[derive(Debug, Clone)]
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

        /// Read file contents as a string
        pub fn read_to_string(&self, file_path: &Path) -> Result<String> {
            std::fs::read_to_string(file_path).map_err(|e| e.into())
        }

        /// Create directory and all parent directories
        pub fn create_dir_all(&self, path: &Path) -> Result<()> {
            std::fs::create_dir_all(path).map_err(|e| e.into())
        }

        /// Write string contents to a file
        pub fn write(&self, file_path: &Path, contents: impl AsRef<[u8]>) -> Result<()> {
            // Ensure parent directory exists
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(file_path, contents).map_err(|e| e.into())
        }

        /// Get file metadata
        pub fn metadata(&self, file_path: &Path) -> Result<std::fs::Metadata> {
            std::fs::metadata(file_path).map_err(|e| e.into())
        }

        /// Read directory entries
        pub fn read_dir(&self, path: &Path) -> Result<std::fs::ReadDir> {
            std::fs::read_dir(path).map_err(|e| e.into())
        }

        /// Remove a directory and all its contents
        pub fn remove_dir_all(&self, path: &Path) -> Result<()> {
            std::fs::remove_dir_all(path).map_err(|e| e.into())
        }

        /// Get the application data directory (.gittype directory)
        pub fn get_app_data_dir(&self) -> Result<PathBuf> {
            <Self as AppDataProvider>::get_app_data_dir()
        }
    }
}

#[cfg(feature = "test-mocks")]
mod mock_impl {
    use super::*;
    use crate::GitTypeError;

    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct FileStorage {
        pub files: Vec<FileEntry>,
        file_contents: HashMap<PathBuf, String>,
    }

    impl AppDataProvider for FileStorage {}

    impl Default for FileStorage {
        fn default() -> Self {
            Self::new()
        }
    }

    impl FileStorage {
        pub fn new() -> Self {
            Self {
                files: Vec::new(),
                file_contents: HashMap::new(),
            }
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

        pub fn set_file_content<P: Into<PathBuf>>(&mut self, path: P, content: String) {
            self.file_contents.insert(path.into(), content);
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

        pub fn file_exists(&self, file_path: &Path) -> bool {
            self.file_contents.contains_key(file_path)
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

        pub fn read_to_string(&self, file_path: &Path) -> Result<String> {
            self.file_contents.get(file_path).cloned().ok_or_else(|| {
                GitTypeError::ExtractionFailed(format!(
                    "Mock file not found: {}",
                    file_path.display()
                ))
            })
        }

        pub fn create_dir_all(&self, _path: &Path) -> Result<()> {
            Ok(())
        }

        pub fn write(&self, _file_path: &Path, _contents: impl AsRef<[u8]>) -> Result<()> {
            Ok(())
        }

        pub fn metadata(&self, file_path: &Path) -> Result<std::fs::Metadata> {
            self.file_contents
                .get(file_path)
                .map(|_| {
                    // Return a fake metadata (this is a hack, but better than real filesystem access)
                    Err(GitTypeError::ExtractionFailed(
                        "Mock metadata not available".to_string(),
                    ))
                })
                .unwrap_or_else(|| {
                    Err(GitTypeError::ExtractionFailed(format!(
                        "Mock file not found: {}",
                        file_path.display()
                    )))
                })
        }

        pub fn read_dir(&self, _path: &Path) -> Result<std::fs::ReadDir> {
            Err(GitTypeError::ExtractionFailed(
                "Mock read_dir not implemented".to_string(),
            ))
        }

        pub fn remove_dir_all(&self, _path: &Path) -> Result<()> {
            Ok(())
        }

        pub fn get_app_data_dir(&self) -> Result<PathBuf> {
            <Self as AppDataProvider>::get_app_data_dir()
        }
    }
}

#[cfg(not(feature = "test-mocks"))]
pub use real_impl::FileStorage;

#[cfg(feature = "test-mocks")]
pub use mock_impl::FileStorage;
