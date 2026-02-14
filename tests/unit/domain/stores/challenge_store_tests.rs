use gittype::domain::stores::{ChallengeStore, ChallengeStoreInterface};

use crate::fixtures::models::challenge;

fn create_store() -> ChallengeStore {
    ChallengeStore::new_for_test()
}

#[test]
fn test_get_challenges_returns_none_by_default() {
    let store = create_store();
    assert!(store.get_challenges().is_none());
}

#[test]
fn test_set_and_get_challenges() {
    let store = create_store();
    let challenges = vec![challenge::build(), challenge::build_with_id("second")];

    store.set_challenges(challenges.clone());

    let result = store.get_challenges().unwrap();
    assert_eq!(result.len(), 2);
}

#[test]
fn test_set_empty_challenges() {
    let store = create_store();
    store.set_challenges(vec![]);

    let result = store.get_challenges().unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_set_challenges_overwrites_previous() {
    let store = create_store();
    store.set_challenges(vec![challenge::build()]);
    store.set_challenges(vec![
        challenge::build_with_id("a"),
        challenge::build_with_id("b"),
        challenge::build_with_id("c"),
    ]);

    let result = store.get_challenges().unwrap();
    assert_eq!(result.len(), 3);
}

#[test]
fn test_clear_removes_challenges() {
    let store = create_store();
    store.set_challenges(vec![challenge::build()]);

    store.clear();

    assert!(store.get_challenges().is_none());
}

#[test]
fn test_clear_on_empty_is_noop() {
    let store = create_store();
    store.clear();
    assert!(store.get_challenges().is_none());
}

#[test]
fn test_take_challenges_returns_and_removes() {
    let store = create_store();
    store.set_challenges(vec![challenge::build()]);

    let taken = store.take_challenges();
    assert!(taken.is_some());
    assert_eq!(taken.unwrap().len(), 1);

    // After take, store should be empty
    assert!(store.get_challenges().is_none());
}

#[test]
fn test_take_challenges_returns_none_when_empty() {
    let store = create_store();
    assert!(store.take_challenges().is_none());
}

#[test]
fn test_take_challenges_twice_returns_none_second_time() {
    let store = create_store();
    store.set_challenges(vec![challenge::build()]);

    let first = store.take_challenges();
    let second = store.take_challenges();

    assert!(first.is_some());
    assert!(second.is_none());
}
