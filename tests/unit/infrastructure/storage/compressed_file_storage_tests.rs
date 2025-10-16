use gittype::infrastructure::storage::compressed_file_storage::CompressedFileStorage;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TestData {
    name: String,
    value: i32,
}

#[test]
fn compressed_file_storage_new() {
    let _storage = CompressedFileStorage::new();
    // Test passes if construction succeeds
}

#[test]
fn compressed_file_storage_default() {
    let _storage = CompressedFileStorage::default();
    // Test passes if construction succeeds
}

#[cfg(feature = "test-mocks")]
mod mock_tests {
    use super::*;

    #[test]
    fn compressed_file_storage_save_and_load() {
        let storage = CompressedFileStorage::new();
        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };
        let path = Path::new("/test/data.bin");

        // Save data
        let save_result = storage.save(path, &data);
        assert!(save_result.is_ok());

        // Load data
        let load_result: Result<Option<TestData>, _> = storage.load(path);
        assert!(load_result.is_ok());
        let loaded = load_result.unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap(), data);
    }

    #[test]
    fn compressed_file_storage_load_nonexistent_file() {
        let storage = CompressedFileStorage::new();
        let path = Path::new("/test/nonexistent.bin");

        let load_result: Result<Option<TestData>, _> = storage.load(path);
        assert!(load_result.is_ok());
        assert!(load_result.unwrap().is_none());
    }

    #[test]
    fn compressed_file_storage_delete_file() {
        let storage = CompressedFileStorage::new();
        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };
        let path = Path::new("/test/data.bin");

        // Save and verify exists
        storage.save(path, &data).unwrap();
        assert!(storage.file_exists(path));

        // Delete and verify doesn't exist
        let delete_result = storage.delete_file(path);
        assert!(delete_result.is_ok());
        assert!(!storage.file_exists(path));
    }

    #[test]
    fn compressed_file_storage_file_exists() {
        let storage = CompressedFileStorage::new();
        let path = Path::new("/test/data.bin");

        // Initially doesn't exist
        assert!(!storage.file_exists(path));

        // After saving, exists
        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };
        storage.save(path, &data).unwrap();
        assert!(storage.file_exists(path));
    }

    #[test]
    fn compressed_file_storage_get_file_size() {
        let storage = CompressedFileStorage::new();
        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };
        let path = Path::new("/test/data.bin");

        // Nonexistent file has no size
        assert!(storage.get_file_size(path).is_none());

        // After saving, has size
        storage.save(path, &data).unwrap();
        let size = storage.get_file_size(path);
        assert!(size.is_some());
        assert!(size.unwrap() > 0);
    }

    #[test]
    fn compressed_file_storage_list_files_in_dir() {
        let storage = CompressedFileStorage::new();
        let dir_path = Path::new("/test");
        let file1_path = dir_path.join("file1.bin");
        let file2_path = dir_path.join("file2.bin");

        // Initially empty
        let files = storage.list_files_in_dir(dir_path);
        assert_eq!(files.len(), 0);

        // Add files
        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };
        storage.save(&file1_path, &data).unwrap();
        storage.save(&file2_path, &data).unwrap();

        // Now has files
        let files = storage.list_files_in_dir(dir_path);
        assert_eq!(files.len(), 2);
        assert!(files.contains(&file1_path));
        assert!(files.contains(&file2_path));
    }

    #[test]
    fn compressed_file_storage_save_multiple_times() {
        let storage = CompressedFileStorage::new();
        let path = Path::new("/test/data.bin");

        // Save first version
        let data1 = TestData {
            name: "first".to_string(),
            value: 1,
        };
        storage.save(path, &data1).unwrap();

        // Save second version (overwrites)
        let data2 = TestData {
            name: "second".to_string(),
            value: 2,
        };
        storage.save(path, &data2).unwrap();

        // Load should return second version
        let loaded: Option<TestData> = storage.load(path).unwrap();
        assert_eq!(loaded.unwrap(), data2);
    }

    #[test]
    fn compressed_file_storage_clone() {
        let storage = CompressedFileStorage::new();
        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };
        let path = Path::new("/test/data.bin");

        // Save in original
        storage.save(path, &data).unwrap();

        // Clone should share same storage
        let cloned = storage.clone();
        assert!(cloned.file_exists(path));

        let loaded: Option<TestData> = cloned.load(path).unwrap();
        assert_eq!(loaded.unwrap(), data);
    }
}
