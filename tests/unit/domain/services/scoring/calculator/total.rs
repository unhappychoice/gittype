use gittype::domain::models::SessionResult;
use gittype::domain::services::scoring::calculator::TotalCalculator;
use gittype::domain::services::scoring::tracker::{TotalTracker, TotalTrackerInterface};
use std::time::Duration;

const EPSILON: f64 = 0.001;

#[test]
fn test_calculate_empty_total() {
    let tracker = TotalTracker::new_for_test();
    let result = TotalCalculator::calculate(&tracker);

    assert_eq!(result.total_sessions_attempted, 0);
    assert_eq!(result.total_sessions_completed, 0);
    assert_eq!(result.total_stages_attempted, 0);
    assert_eq!(result.total_stages_completed, 0);
    assert_eq!(result.total_stages_skipped, 0);
    assert_eq!(result.total_keystrokes, 0);
    assert_eq!(result.total_mistakes, 0);
    assert!((result.overall_accuracy - 0.0).abs() < EPSILON);
    assert!((result.overall_wpm - 0.0).abs() < EPSILON);
    assert!((result.overall_cpm - 0.0).abs() < EPSILON);
    assert!((result.best_session_wpm - 0.0).abs() < EPSILON);
    assert!((result.worst_session_wpm - 0.0).abs() < EPSILON);
    assert!((result.best_session_accuracy - 0.0).abs() < EPSILON);
    assert!((result.worst_session_accuracy - 0.0).abs() < EPSILON);
    assert!((result.total_score - 0.0).abs() < EPSILON);
}

#[test]
fn test_calculate_single_session() {
    let tracker = TotalTracker::new_for_test();
    let session_result = SessionResult {
        session_duration: Duration::from_secs(120),
        valid_session_duration: Duration::from_secs(120),
        invalid_session_duration: Duration::ZERO,
        stages_completed: 2,
        stages_attempted: 2,
        stages_skipped: 0,
        overall_accuracy: 93.2,
        overall_wpm: 25.0,
        overall_cpm: 125.0,
        valid_keystrokes: 250,
        valid_mistakes: 17,
        invalid_keystrokes: 0,
        invalid_mistakes: 0,
        best_stage_wpm: 30.0,
        worst_stage_wpm: 20.0,
        best_stage_accuracy: 95.0,
        worst_stage_accuracy: 90.0,
        session_score: 3753.302792,
        session_successful: true,
        ..Default::default()
    };
    tracker.record(session_result);
    let result = TotalCalculator::calculate(&tracker);

    assert_eq!(result.total_sessions_attempted, 1);
    assert_eq!(result.total_sessions_completed, 1);
    assert_eq!(result.total_stages_attempted, 2);
    assert_eq!(result.total_stages_completed, 2);
    assert_eq!(result.total_stages_skipped, 0);
    assert_eq!(result.total_keystrokes, 250);
    assert_eq!(result.total_mistakes, 17);
    assert!((result.overall_accuracy - 93.2).abs() < EPSILON);
    assert!((result.overall_wpm - 25.0).abs() < EPSILON);
    assert!((result.overall_cpm - 125.0).abs() < EPSILON);
    assert!((result.best_session_wpm - 25.0).abs() < EPSILON);
    assert!((result.worst_session_wpm - 25.0).abs() < EPSILON);
    assert!((result.best_session_accuracy - 93.2).abs() < EPSILON);
    assert!((result.worst_session_accuracy - 93.2).abs() < EPSILON);
    assert!((result.total_score - 3753.302792).abs() < EPSILON);
}

#[test]
fn test_calculate_multiple_sessions_mixed() {
    let tracker = TotalTracker::new_for_test();
    // Session 1 (successful)
    let session_result1 = SessionResult {
        session_duration: Duration::from_secs(60),
        valid_session_duration: Duration::from_secs(60),
        stages_completed: 1,
        stages_attempted: 1,
        overall_accuracy: 90.0,
        overall_wpm: 20.0,
        overall_cpm: 100.0,
        valid_keystrokes: 100,
        valid_mistakes: 10,
        session_score: 2700.0,
        session_successful: true,
        ..Default::default()
    };
    // Session 2 (failed)
    let session_result2 = SessionResult {
        session_duration: Duration::from_secs(30),
        valid_session_duration: Duration::ZERO,
        stages_completed: 0,
        stages_attempted: 1,
        session_successful: false,
        ..Default::default()
    };
    // Session 3 (successful, higher score)
    let session_result3 = SessionResult {
        session_duration: Duration::from_secs(90),
        valid_session_duration: Duration::from_secs(90),
        stages_completed: 1,
        stages_attempted: 1,
        overall_accuracy: 95.0,
        overall_wpm: 30.0,
        overall_cpm: 150.0,
        valid_keystrokes: 150,
        valid_mistakes: 7,
        session_score: 5000.0,
        session_successful: true,
        ..Default::default()
    };

    tracker.record(session_result1);
    tracker.record(session_result2);
    tracker.record(session_result3);

    let result = TotalCalculator::calculate(&tracker);

    assert_eq!(result.total_sessions_attempted, 3);
    assert_eq!(result.total_sessions_completed, 2);
    assert_eq!(result.total_stages_attempted, 3);
    assert_eq!(result.total_stages_completed, 2);
    assert_eq!(result.total_stages_skipped, 0);

    // Total keystrokes = (100 + 0 + 150) = 250
    // Total mistakes = (10 + 0 + 7) = 17
    // Total time = (60 + 30 + 90) = 180 secs
    // Overall CPM = (250 / 180) * 60 = 83.333...
    // Overall WPM = 83.333... / 5 = 16.666...
    // Overall Accuracy = ((250 - 17) / 250) * 100 = 93.2
    assert_eq!(result.total_keystrokes, 250);
    assert_eq!(result.total_mistakes, 17);
    assert!((result.overall_accuracy - 93.2).abs() < EPSILON);
    assert!((result.overall_wpm - 16.666666666666668).abs() < EPSILON);
    assert!((result.overall_cpm - 83.33333333333333).abs() < EPSILON);

    // Best session is session_result3
    assert!((result.best_session_wpm - 30.0).abs() < EPSILON);
    assert!((result.best_session_accuracy - 95.0).abs() < EPSILON);

    // Worst session is session_result1 (among successful ones)
    assert!((result.worst_session_wpm - 20.0).abs() < EPSILON);
    assert!((result.worst_session_accuracy - 90.0).abs() < EPSILON);

    // Total score = 2700 + 5000 = 7700
    assert!((result.total_score - 7700.0).abs() < EPSILON);
}

#[test]
fn test_calculate_multiple_sessions_no_completed() {
    let tracker = TotalTracker::new_for_test();
    let session_result1 = SessionResult {
        session_successful: false,
        ..Default::default()
    };
    let session_result2 = SessionResult {
        session_successful: false,
        ..Default::default()
    };
    tracker.record(session_result1);
    tracker.record(session_result2);
    let result = TotalCalculator::calculate(&tracker);

    assert_eq!(result.total_sessions_attempted, 2);
    assert_eq!(result.total_sessions_completed, 0);
    assert!((result.total_score - 0.0).abs() < EPSILON);
    assert!((result.overall_accuracy - 0.0).abs() < EPSILON);
}
