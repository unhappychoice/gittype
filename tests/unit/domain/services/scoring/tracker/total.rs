use gittype::domain::models::SessionResult;
use gittype::domain::services::scoring::tracker::TotalTracker;

#[test]
fn test_new_total_tracker() {
    let tracker = TotalTracker::new();
    let data = tracker.get_data();
    assert!(data.session_results.is_empty());
}

#[test]
fn test_record_session_result() {
    let mut tracker = TotalTracker::new();
    let session_result = SessionResult::default(); // Use a default session result for testing
    tracker.record(session_result.clone());
    let data = tracker.get_data();
    assert_eq!(data.session_results.len(), 1);
    assert_eq!(data.session_results[0], session_result);
}

#[test]
fn test_record_multiple_session_results() {
    let mut tracker = TotalTracker::new();
    let session_result1 = SessionResult::default();
    let session_result2 = SessionResult::default();
    tracker.record(session_result1.clone());
    tracker.record(session_result2.clone());
    let data = tracker.get_data();
    assert_eq!(data.session_results.len(), 2);
    assert_eq!(data.session_results[0], session_result1);
    assert_eq!(data.session_results[1], session_result2);
}

#[test]
fn test_default_total_tracker() {
    let tracker = TotalTracker::default();
    let data = tracker.get_data();
    assert!(data.session_results.is_empty());
}

#[test]
fn test_tracker_clone() {
    let mut tracker = TotalTracker::new();
    let session_result = SessionResult::default();
    tracker.record(session_result.clone());

    let cloned_tracker = tracker.clone();
    let data = cloned_tracker.get_data();
    assert_eq!(data.session_results.len(), 1);
    assert_eq!(data.session_results[0], session_result);
}

#[test]
fn test_initialize_global_instance() {
    let tracker = TotalTracker::new();
    TotalTracker::initialize_global_instance(tracker);
    // Just verify it doesn't panic
}

#[test]
fn test_tracker_data_clone() {
    let tracker = TotalTracker::new();
    let data = tracker.get_data();
    let cloned_data = data.clone();
    assert_eq!(
        data.session_results.len(),
        cloned_data.session_results.len()
    );
}
