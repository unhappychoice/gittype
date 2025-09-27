use gittype::domain::services::history::SessionHistory;

#[test]
fn session_history_default_is_empty() {
    let history = SessionHistory::new();
    let entries = history.get_history().expect("history should load");
    assert!(entries.is_empty());
}
