use gittype::domain::models::StageResult;
use gittype::domain::services::scoring::tracker::{SessionTracker, SessionTrackerInterface};

#[test]
fn test_new_session_tracker() {
    let tracker = SessionTracker::new_for_test();
    let data = tracker.get_data();
    assert!(data.stage_results.is_empty());
    // session_start_time is Instant::now(), so we can't assert its exact value
}

#[test]
fn test_record_stage_result() {
    let tracker = SessionTracker::new_for_test();
    let stage_result = StageResult::default(); // Use a default stage result for testing
    tracker.record(stage_result.clone());
    let data = tracker.get_data();
    assert_eq!(data.stage_results.len(), 1);
    assert_eq!(data.stage_results[0], stage_result);
}

#[test]
fn test_record_multiple_stage_results() {
    let tracker = SessionTracker::new_for_test();
    let stage_result1 = StageResult::default();
    let stage_result2 = StageResult::default();
    tracker.record(stage_result1.clone());
    tracker.record(stage_result2.clone());
    let data = tracker.get_data();
    assert_eq!(data.stage_results.len(), 2);
    assert_eq!(data.stage_results[0], stage_result1);
    assert_eq!(data.stage_results[1], stage_result2);
}

#[test]
fn test_default_session_tracker() {
    let tracker = SessionTracker::default();
    let data = tracker.get_data();
    assert!(data.stage_results.is_empty());
}

#[test]
fn test_tracker_data_independence() {
    let tracker = SessionTracker::new_for_test();
    let stage_result = StageResult::default();
    tracker.record(stage_result.clone());

    let data = tracker.get_data();
    assert_eq!(data.stage_results.len(), 1);
    assert_eq!(data.stage_results[0], stage_result);
}

#[test]
fn test_tracker_data_clone() {
    let tracker = SessionTracker::new_for_test();
    let data = tracker.get_data();
    let cloned_data = data.clone();
    assert_eq!(data.stage_results.len(), cloned_data.stage_results.len());
}
