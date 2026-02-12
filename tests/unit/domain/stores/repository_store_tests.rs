use gittype::domain::stores::{RepositoryStore, RepositoryStoreInterface};
use std::path::PathBuf;

use crate::fixtures::models::git_repository;

fn create_store() -> RepositoryStore {
    RepositoryStore::new_for_test()
}

// --- git_repository field ---

#[test]
fn test_get_repository_returns_none_by_default() {
    let store = create_store();
    assert!(store.get_repository().is_none());
}

#[test]
fn test_set_and_get_repository() {
    let store = create_store();
    let repo = git_repository::build();

    store.set_repository(repo.clone());

    let result = store.get_repository().unwrap();
    assert_eq!(result.user_name, repo.user_name);
    assert_eq!(result.repository_name, repo.repository_name);
}

#[test]
fn test_clear_repository_removes_value() {
    let store = create_store();
    store.set_repository(git_repository::build());

    store.clear_repository();

    assert!(store.get_repository().is_none());
}

#[test]
fn test_clear_repository_on_empty_is_noop() {
    let store = create_store();
    store.clear_repository();
    assert!(store.get_repository().is_none());
}

// --- repo_spec field ---

#[test]
fn test_get_repo_spec_returns_none_by_default() {
    let store = create_store();
    assert!(store.get_repo_spec().is_none());
}

#[test]
fn test_set_and_get_repo_spec() {
    let store = create_store();
    store.set_repo_spec("owner/repo".to_string());

    assert_eq!(store.get_repo_spec().unwrap(), "owner/repo");
}

#[test]
fn test_set_repo_spec_overwrites_previous() {
    let store = create_store();
    store.set_repo_spec("first/repo".to_string());
    store.set_repo_spec("second/repo".to_string());

    assert_eq!(store.get_repo_spec().unwrap(), "second/repo");
}

// --- repo_path field ---

#[test]
fn test_get_repo_path_returns_none_by_default() {
    let store = create_store();
    assert!(store.get_repo_path().is_none());
}

#[test]
fn test_set_and_get_repo_path() {
    let store = create_store();
    let path = PathBuf::from("/tmp/test-repo");
    store.set_repo_path(path.clone());

    assert_eq!(store.get_repo_path().unwrap(), path);
}

// --- extraction_options field ---

#[test]
fn test_get_extraction_options_returns_none_by_default() {
    let store = create_store();
    assert!(store.get_extraction_options().is_none());
}

#[test]
fn test_set_and_get_extraction_options() {
    use gittype::domain::models::ExtractionOptions;

    let store = create_store();
    let options = ExtractionOptions::default();
    store.set_extraction_options(options);

    let result = store.get_extraction_options().unwrap();
    assert!(!result.include_patterns.is_empty());
}

// --- clear all ---

#[test]
fn test_clear_resets_all_fields() {
    let store = create_store();
    store.set_repository(git_repository::build());
    store.set_repo_spec("owner/repo".to_string());
    store.set_repo_path(PathBuf::from("/tmp/test"));
    store.set_extraction_options(gittype::domain::models::ExtractionOptions::default());

    store.clear();

    assert!(store.get_repository().is_none());
    assert!(store.get_repo_spec().is_none());
    assert!(store.get_repo_path().is_none());
    assert!(store.get_extraction_options().is_none());
}

#[test]
fn test_clear_on_empty_store_is_noop() {
    let store = create_store();
    store.clear();

    assert!(store.get_repository().is_none());
    assert!(store.get_repo_spec().is_none());
    assert!(store.get_repo_path().is_none());
    assert!(store.get_extraction_options().is_none());
}
