use super::AppDataProvider;
#[cfg(feature = "test-mocks")]
use crate::Result;
#[cfg(not(feature = "test-mocks"))]
use crate::{GitTypeError, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[cfg(not(feature = "test-mocks"))]
mod real_impl {
    use super::*;
    use flate2::{read::GzDecoder, write::GzEncoder, Compression};
    use std::fs;
    use std::io::{Read, Write};

    #[derive(Debug, Clone, Default)]
    pub struct CompressedFileStorage;

    impl AppDataProvider for CompressedFileStorage {}

    impl CompressedFileStorage {
        /// Save compressed binary data to a file
        pub fn save<T: Serialize>(&self, file_path: &Path, data: &T) -> Result<()> {
            // Ensure parent directory exists
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let binary_data = bincode::serde::encode_to_vec(data, bincode::config::standard())
                .map_err(|e| {
                    GitTypeError::ExtractionFailed(format!("Failed to serialize data: {}", e))
                })?;

            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&binary_data).map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to compress data: {}", e))
            })?;

            let compressed_data = encoder.finish().map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to finish compression: {}", e))
            })?;

            fs::write(file_path, compressed_data).map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to save file: {}", e))
            })?;

            Ok(())
        }

        /// Load and decompress binary data from a file
        pub fn load<T: for<'de> Deserialize<'de>>(&self, file_path: &Path) -> Result<Option<T>> {
            if !file_path.exists() {
                return Ok(None);
            }

            let compressed_data = fs::read(file_path).map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to read file: {}", e))
            })?;

            let mut decoder = GzDecoder::new(&compressed_data[..]);
            let mut binary_data = Vec::new();

            decoder.read_to_end(&mut binary_data).map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to decompress data: {}", e))
            })?;

            let (data, _) =
                bincode::serde::decode_from_slice(&binary_data, bincode::config::standard())
                    .map_err(|e| {
                        GitTypeError::ExtractionFailed(format!("Failed to deserialize data: {}", e))
                    })?;

            Ok(Some(data))
        }

        /// Delete a file if it exists
        pub fn delete_file(&self, file_path: &Path) -> Result<()> {
            if file_path.exists() {
                std::fs::remove_file(file_path)?;
            }
            Ok(())
        }

        /// Get file size if it exists
        pub fn get_file_size(&self, file_path: &Path) -> Option<u64> {
            file_path.metadata().ok().map(|m| m.len())
        }

        /// List all files in a directory (for testing compatibility)
        pub fn list_files_in_dir(&self, dir_path: &Path) -> Vec<PathBuf> {
            if !dir_path.exists() {
                return Vec::new();
            }

            std::fs::read_dir(dir_path)
                .unwrap_or_else(|_| std::fs::read_dir(".").unwrap())
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(|path| path.is_file())
                .collect()
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
    use crate::GitTypeError;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    #[derive(Debug, Clone)]
    pub struct CompressedFileStorage {
        test_storage: Arc<Mutex<HashMap<PathBuf, Vec<u8>>>>,
    }

    impl Default for CompressedFileStorage {
        fn default() -> Self {
            Self {
                test_storage: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }

    impl AppDataProvider for CompressedFileStorage {}

    impl CompressedFileStorage {
        pub fn save<T: Serialize>(&self, file_path: &Path, data: &T) -> Result<()> {
            use flate2::{write::GzEncoder, Compression};
            use std::io::Write;

            let binary_data = bincode::serde::encode_to_vec(data, bincode::config::standard())
                .map_err(|e| {
                    GitTypeError::ExtractionFailed(format!("Failed to serialize data: {}", e))
                })?;

            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&binary_data).map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to compress data: {}", e))
            })?;

            let compressed_data = encoder.finish().map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to finish compression: {}", e))
            })?;

            let mut storage = self.test_storage.lock().unwrap();
            storage.insert(file_path.to_path_buf(), compressed_data);
            Ok(())
        }

        pub fn load<T: for<'de> Deserialize<'de>>(&self, file_path: &Path) -> Result<Option<T>> {
            use flate2::read::GzDecoder;
            use std::io::Read;

            let storage = self.test_storage.lock().unwrap();
            let compressed_data = match storage.get(file_path) {
                Some(data) => data,
                None => return Ok(None),
            };

            let mut decoder = GzDecoder::new(&compressed_data[..]);
            let mut binary_data = Vec::new();

            decoder.read_to_end(&mut binary_data).map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to decompress data: {}", e))
            })?;

            let (data, _) =
                bincode::serde::decode_from_slice(&binary_data, bincode::config::standard())
                    .map_err(|e| {
                        GitTypeError::ExtractionFailed(format!("Failed to deserialize data: {}", e))
                    })?;

            Ok(Some(data))
        }

        pub fn delete_file(&self, file_path: &Path) -> Result<()> {
            let mut storage = self.test_storage.lock().unwrap();
            storage.remove(file_path);
            Ok(())
        }

        pub fn file_exists(&self, file_path: &Path) -> bool {
            let storage = self.test_storage.lock().unwrap();
            storage.contains_key(file_path)
        }

        /// Test-only method to get all stored file paths in a directory
        pub fn list_files_in_dir(&self, dir_path: &Path) -> Vec<PathBuf> {
            let storage = self.test_storage.lock().unwrap();
            storage
                .keys()
                .filter(|path| path.parent() == Some(dir_path))
                .cloned()
                .collect()
        }

        /// Test-only method to get file size
        pub fn get_file_size(&self, file_path: &Path) -> Option<u64> {
            let storage = self.test_storage.lock().unwrap();
            storage.get(file_path).map(|data| data.len() as u64)
        }
    }
}

#[cfg(not(feature = "test-mocks"))]
pub use real_impl::CompressedFileStorage;

#[cfg(feature = "test-mocks")]
pub use mock_impl::CompressedFileStorage;
