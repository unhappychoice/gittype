use gittype::domain::models::session::Session;
use gittype::domain::models::total::{Total, TotalResult};

#[test]
fn total_new_creates_empty() {
    let total = Total::new();
    assert!(total.sessions.is_empty());
}

#[test]
fn total_default_creates_empty() {
    let total = Total::default();
    assert!(total.sessions.is_empty());
}

#[test]
fn total_add_session() {
    let mut total = Total::new();
    let session = Session::new(vec![]);
    total.add_session(session);
    assert_eq!(total.sessions.len(), 1);
}

#[test]
fn total_clone() {
    let mut total = Total::new();
    total.add_session(Session::new(vec![]));
    let cloned = total.clone();
    assert_eq!(cloned.sessions.len(), 1);
}

#[test]
fn total_result_new_initializes_defaults() {
    let result = TotalResult::new();
    assert_eq!(result.total_sessions_completed, 0);
    assert_eq!(result.total_sessions_attempted, 0);
    assert_eq!(result.overall_accuracy, 0.0);
    assert_eq!(result.overall_wpm, 0.0);
    assert_eq!(result.best_session_wpm, 0.0);
    assert_eq!(result.worst_session_wpm, f64::MAX);
}

#[test]
fn total_result_default_initializes_defaults() {
    let result = TotalResult::default();
    assert_eq!(result.total_sessions_completed, 0);
}

#[test]
fn total_result_finalize_sets_worst_defaults() {
    let mut result = TotalResult::new();
    result.finalize();

    // After finalize, worst values should be set to 0.0 if they were MAX
    assert_eq!(result.worst_session_wpm, 0.0);
    assert_eq!(result.worst_session_accuracy, 0.0);
}

#[test]
fn total_result_finalize_preserves_real_worst_values() {
    let mut result = TotalResult::new();
    result.worst_session_wpm = 50.0;
    result.worst_session_accuracy = 85.0;
    result.finalize();

    // Real values should be preserved
    assert_eq!(result.worst_session_wpm, 50.0);
    assert_eq!(result.worst_session_accuracy, 85.0);
}

#[test]
fn get_completion_status_no_sessions() {
    let result = TotalResult::new();
    let status = result.get_completion_status();
    assert_eq!(status, "No sessions attempted");
}

#[test]
fn get_completion_status_perfect() {
    let mut result = TotalResult::new();
    result.total_sessions_completed = 5;
    result.total_sessions_attempted = 5;
    let status = result.get_completion_status();
    assert_eq!(status, "Perfect! 5 sessions completed");
}

#[test]
fn get_completion_status_partial() {
    let mut result = TotalResult::new();
    result.total_sessions_completed = 3;
    result.total_sessions_attempted = 5;
    let status = result.get_completion_status();
    assert_eq!(status, "3/5 sessions completed");
}

#[test]
fn create_share_text_contains_stats() {
    let mut result = TotalResult::new();
    result.total_keystrokes = 1000;
    result.total_score = 5000.0;
    result.overall_cpm = 300.0;
    result.total_sessions_completed = 3;
    result.total_sessions_attempted = 5;

    let text = result.create_share_text();

    assert!(text.contains("1000"));
    assert!(text.contains("5000"));
    assert!(text.contains("300"));
    assert!(text.contains("3/5"));
    assert!(text.contains("gittype"));
    assert!(text.contains("github.com/unhappychoice/gittype"));
}

#[test]
fn create_share_text_format() {
    let result = TotalResult::new();
    let text = result.create_share_text();

    assert!(text.contains("#gittype"));
    assert!(text.contains("#typing"));
    assert!(text.contains("#coding"));
}

#[test]
fn total_result_clone() {
    let mut result = TotalResult::new();
    result.total_keystrokes = 100;
    result.total_score = 500.0;

    let cloned = result.clone();
    assert_eq!(cloned.total_keystrokes, 100);
    assert_eq!(cloned.total_score, 500.0);
}
