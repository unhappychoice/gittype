use gittype::domain::models::StageResult;
use gittype::scoring::tracker::SessionTracker;

#[test]
fn test_new_session_tracker() {
    let tracker = SessionTracker::new();
    let data = tracker.get_data();
    assert!(data.stage_results.is_empty());
    // session_start_time is Instant::now(), so we can't assert its exact value
}

#[test]
fn test_record_stage_result() {
    let mut tracker = SessionTracker::new();
    let stage_result = StageResult::default(); // Use a default stage result for testing
    tracker.record(stage_result.clone());
    let data = tracker.get_data();
    assert_eq!(data.stage_results.len(), 1);
    assert_eq!(data.stage_results[0], stage_result);
}

#[test]
fn test_record_multiple_stage_results() {
    let mut tracker = SessionTracker::new();
    let stage_result1 = StageResult::default();
    let stage_result2 = StageResult::default();
    tracker.record(stage_result1.clone());
    tracker.record(stage_result2.clone());
    let data = tracker.get_data();
    assert_eq!(data.stage_results.len(), 2);
    assert_eq!(data.stage_results[0], stage_result1);
    assert_eq!(data.stage_results[1], stage_result2);
}
