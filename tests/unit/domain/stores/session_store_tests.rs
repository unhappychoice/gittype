use gittype::domain::stores::{SessionStore, SessionStoreInterface};

fn create_store() -> SessionStore {
    SessionStore::new_for_test()
}

// --- loading_completed ---

#[test]
fn test_loading_completed_is_false_by_default() {
    let store = create_store();
    assert!(!store.is_loading_completed());
}

#[test]
fn test_set_loading_completed_to_true() {
    let store = create_store();
    store.set_loading_completed(true);
    assert!(store.is_loading_completed());
}

#[test]
fn test_set_loading_completed_to_false() {
    let store = create_store();
    store.set_loading_completed(true);
    store.set_loading_completed(false);
    assert!(!store.is_loading_completed());
}

// --- loading_failed ---

#[test]
fn test_loading_failed_is_false_by_default() {
    let store = create_store();
    assert!(!store.is_loading_failed());
}

#[test]
fn test_set_loading_failed_to_true() {
    let store = create_store();
    store.set_loading_failed(true);
    assert!(store.is_loading_failed());
}

#[test]
fn test_set_loading_failed_to_false() {
    let store = create_store();
    store.set_loading_failed(true);
    store.set_loading_failed(false);
    assert!(!store.is_loading_failed());
}

// --- error_message ---

#[test]
fn test_error_message_is_none_by_default() {
    let store = create_store();
    assert!(store.get_error_message().is_none());
}

#[test]
fn test_set_and_get_error_message() {
    let store = create_store();
    store.set_error_message("Something went wrong".to_string());

    assert_eq!(store.get_error_message().unwrap(), "Something went wrong");
}

#[test]
fn test_set_error_message_overwrites_previous() {
    let store = create_store();
    store.set_error_message("First error".to_string());
    store.set_error_message("Second error".to_string());

    assert_eq!(store.get_error_message().unwrap(), "Second error");
}

#[test]
fn test_clear_error_message() {
    let store = create_store();
    store.set_error_message("Error".to_string());
    store.clear_error_message();

    assert!(store.get_error_message().is_none());
}

#[test]
fn test_clear_error_message_on_none_is_noop() {
    let store = create_store();
    store.clear_error_message();
    assert!(store.get_error_message().is_none());
}

// --- clear all ---

#[test]
fn test_clear_resets_all_fields() {
    let store = create_store();
    store.set_loading_completed(true);
    store.set_loading_failed(true);
    store.set_error_message("Error".to_string());

    store.clear();

    assert!(!store.is_loading_completed());
    assert!(!store.is_loading_failed());
    assert!(store.get_error_message().is_none());
}

#[test]
fn test_clear_on_default_store_is_noop() {
    let store = create_store();
    store.clear();

    assert!(!store.is_loading_completed());
    assert!(!store.is_loading_failed());
    assert!(store.get_error_message().is_none());
}
