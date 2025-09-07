use gittype::models::SessionResult;
use gittype::scoring::tracker::TotalTracker;

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
