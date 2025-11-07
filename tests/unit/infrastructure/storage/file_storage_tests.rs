use gittype::infrastructure::storage::file_storage::{
    FileEntry, FileStorage, FileStorageInterface,
};
use std::path::PathBuf;

#[test]
fn file_entry_creation() {
    let entry = FileEntry {
        path: PathBuf::from("/test/file.txt"),
        is_file: true,
    };

    assert_eq!(entry.path, PathBuf::from("/test/file.txt"));
    assert!(entry.is_file);
}

#[test]
fn file_entry_directory() {
    let entry = FileEntry {
        path: PathBuf::from("/test/dir"),
        is_file: false,
    };

    assert_eq!(entry.path, PathBuf::from("/test/dir"));
    assert!(!entry.is_file);
}

#[test]
fn file_entry_clone() {
    let entry = FileEntry {
        path: PathBuf::from("/test/file.txt"),
        is_file: true,
    };

    let cloned = entry.clone();
    assert_eq!(entry.path, cloned.path);
    assert_eq!(entry.is_file, cloned.is_file);
}

#[test]
fn file_storage_new() {
    let _storage = FileStorage::new();
    // Test passes if construction succeeds
}

#[test]
fn file_storage_default() {
    let _storage = FileStorage::default();
    // Test passes if construction succeeds
}

#[cfg(feature = "test-mocks")]
mod mock_tests {
    use super::*;

    #[test]
    fn file_storage_add_file() {
        let mut storage = FileStorage::new();
        storage.add_file("/test/file.txt");

        assert_eq!(storage.files.len(), 1);
        assert_eq!(storage.files[0].path, PathBuf::from("/test/file.txt"));
        assert!(storage.files[0].is_file);
    }

    #[test]
    fn file_storage_add_directory() {
        let mut storage = FileStorage::new();
        storage.add_directory("/test/dir");

        assert_eq!(storage.files.len(), 1);
        assert_eq!(storage.files[0].path, PathBuf::from("/test/dir"));
        assert!(!storage.files[0].is_file);
    }

    #[test]
    fn file_storage_read_json_returns_none() {
        use serde::Deserialize;
        use std::path::Path;

        #[derive(Deserialize)]
        struct TestData {
            #[allow(dead_code)]
            value: String,
        }

        let storage = FileStorage::new();
        let result: Result<Option<TestData>, _> = storage.read_json(Path::new("/test/file.json"));

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn file_storage_write_json_succeeds() {
        use serde::Serialize;
        use std::path::Path;

        #[derive(Serialize)]
        struct TestData {
            value: String,
        }

        let storage = FileStorage::new();
        let data = TestData {
            value: "test".to_string(),
        };
        let result = storage.write_json(Path::new("/test/file.json"), &data);

        assert!(result.is_ok());
    }

    #[test]
    fn file_storage_delete_file_succeeds() {
        use std::path::Path;

        let storage = FileStorage::new();
        let result = storage.delete_file(Path::new("/test/file.txt"));

        assert!(result.is_ok());
    }

    #[test]
    fn file_storage_file_exists_returns_false() {
        use std::path::Path;

        let storage = FileStorage::new();
        let exists = storage.file_exists(Path::new("/test/file.txt"));

        assert!(!exists);
    }

    #[test]
    fn file_storage_walk_directory_returns_files() {
        use std::path::Path;

        let mut storage = FileStorage::new();
        storage.add_file("/test/file1.txt");
        storage.add_file("/test/file2.txt");

        let result = storage.walk_directory(Path::new("/test"));

        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn file_storage_walk_nonexistent_directory_returns_error() {
        use std::path::Path;

        let storage = FileStorage::new();
        let result = storage.walk_directory(Path::new("/nonexistent/path"));

        assert!(result.is_err());
    }
}
