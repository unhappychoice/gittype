use gittype::infrastructure::storage::app_data_provider::AppDataProvider;
use gittype::infrastructure::storage::file_storage::FileStorage;

struct TestProvider;
impl AppDataProvider for TestProvider {}

#[test]
fn app_data_provider_get_app_data_dir() {
    let result = TestProvider::get_app_data_dir();
    assert!(result.is_ok());
}

#[test]
fn app_data_provider_returns_valid_path() {
    let result = TestProvider::get_app_data_dir();
    assert!(result.is_ok());
    let path = result.unwrap();
    assert!(!path.as_os_str().is_empty());
}

#[cfg(feature = "test-mocks")]
mod mock_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn app_data_provider_returns_test_path() {
        let result = TestProvider::get_app_data_dir();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/tmp/test"));
    }
}

#[test]
fn file_storage_implements_app_data_provider() {
    let file_storage = FileStorage::new();
    let result = file_storage.get_app_data_dir();
    assert!(result.is_ok());
}

#[test]
fn app_data_provider_path_is_consistent() {
    let path1 = TestProvider::get_app_data_dir().unwrap();
    let path2 = TestProvider::get_app_data_dir().unwrap();
    assert_eq!(path1, path2);
}
