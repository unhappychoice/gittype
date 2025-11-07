use serde::{Deserialize, Serialize};
use shaku::Interface;

use std::path::{Path, PathBuf};

use crate::Result;

use super::AppDataProvider;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub is_file: bool,
}

pub trait FileStorageInterface: Interface + std::fmt::Debug {
    fn delete_file(&self, file_path: &Path) -> Result<()>;
    fn file_exists(&self, file_path: &Path) -> bool;
    fn walk_directory(&self, path: &Path) -> Result<Vec<FileEntry>>;
    fn read_to_string(&self, file_path: &Path) -> Result<String>;
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    fn write(&self, file_path: &Path, contents: &[u8]) -> Result<()>;
    fn metadata(&self, file_path: &Path) -> Result<std::fs::Metadata>;
    fn read_dir(&self, path: &Path) -> Result<std::fs::ReadDir>;
    fn remove_dir_all(&self, path: &Path) -> Result<()>;
    fn get_app_data_dir(&self) -> Result<PathBuf>;
}

#[cfg(not(feature = "test-mocks"))]
mod real_impl {
    use super::*;

    use crate::GitTypeError;

    #[derive(Debug, Clone, shaku::Component)]
    #[shaku(interface = FileStorageInterface)]
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
    }

    impl FileStorageInterface for FileStorage {
        fn delete_file(&self, file_path: &Path) -> Result<()> {
            if file_path.exists() {
                std::fs::remove_file(file_path)?;
            }
            Ok(())
        }

        fn file_exists(&self, file_path: &Path) -> bool {
            file_path.exists()
        }

        fn walk_directory(&self, path: &Path) -> Result<Vec<FileEntry>> {
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

        fn read_to_string(&self, file_path: &Path) -> Result<String> {
            std::fs::read_to_string(file_path).map_err(|e| e.into())
        }

        fn create_dir_all(&self, path: &Path) -> Result<()> {
            std::fs::create_dir_all(path).map_err(|e| e.into())
        }

        fn write(&self, file_path: &Path, contents: &[u8]) -> Result<()> {
            // Ensure parent directory exists
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(file_path, contents).map_err(|e| e.into())
        }

        fn metadata(&self, file_path: &Path) -> Result<std::fs::Metadata> {
            std::fs::metadata(file_path).map_err(|e| e.into())
        }

        fn read_dir(&self, path: &Path) -> Result<std::fs::ReadDir> {
            std::fs::read_dir(path).map_err(|e| e.into())
        }

        fn remove_dir_all(&self, path: &Path) -> Result<()> {
            std::fs::remove_dir_all(path).map_err(|e| e.into())
        }

        fn get_app_data_dir(&self) -> Result<PathBuf> {
            <Self as AppDataProvider>::get_app_data_dir()
        }
    }
}

#[cfg(feature = "test-mocks")]
mod mock_impl {
    use super::*;

    use std::collections::HashMap;

    use crate::GitTypeError;

    #[derive(Debug, Clone, shaku::Component)]
    #[shaku(interface = FileStorageInterface)]
    pub struct FileStorage {
        #[shaku(default)]
        pub files: Vec<FileEntry>,
        #[shaku(default)]
        file_contents: HashMap<PathBuf, String>,
    }

    impl FileStorageInterface for FileStorage {
        fn delete_file(&self, _file_path: &Path) -> Result<()> {
            Ok(())
        }

        fn file_exists(&self, file_path: &Path) -> bool {
            self.file_contents.contains_key(file_path)
        }

        fn walk_directory(&self, path: &Path) -> Result<Vec<FileEntry>> {
            // For mock implementation, simulate real behavior for non-existent paths
            if path.to_str() == Some("/nonexistent/path") {
                return Err(GitTypeError::ExtractionFailed(format!(
                    "Path does not exist: {}",
                    path.display()
                )));
            }
            Ok(self.files.clone())
        }

        fn read_to_string(&self, file_path: &Path) -> Result<String> {
            self.file_contents.get(file_path).cloned().ok_or_else(|| {
                GitTypeError::ExtractionFailed(format!(
                    "Mock file not found: {}",
                    file_path.display()
                ))
            })
        }

        fn create_dir_all(&self, _path: &Path) -> Result<()> {
            Ok(())
        }

        fn write(&self, _file_path: &Path, _contents: &[u8]) -> Result<()> {
            Ok(())
        }

        fn metadata(&self, file_path: &Path) -> Result<std::fs::Metadata> {
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

        fn read_dir(&self, _path: &Path) -> Result<std::fs::ReadDir> {
            Err(GitTypeError::ExtractionFailed(
                "Mock read_dir not implemented".to_string(),
            ))
        }

        fn remove_dir_all(&self, _path: &Path) -> Result<()> {
            Ok(())
        }

        fn get_app_data_dir(&self) -> Result<PathBuf> {
            <Self as AppDataProvider>::get_app_data_dir()
        }
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
    }
}

#[cfg(not(feature = "test-mocks"))]
pub use real_impl::FileStorage;

#[cfg(feature = "test-mocks")]
pub use mock_impl::FileStorage;
