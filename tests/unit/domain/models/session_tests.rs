use crate::fixtures::models::challenge;
use gittype::domain::models::session::{Session, SessionResult};
use gittype::domain::models::Stage;
use std::time::Duration;

fn sample_stage(id: &str) -> Stage {
    Stage::new(challenge::build_with_id(id), 1)
}

#[test]
fn session_new_preserves_stage_list() {
    let stages = vec![sample_stage("a"), sample_stage("b")];
    let session = Session::new(stages.clone());

    assert_eq!(session.stages.len(), 2);
    assert_eq!(session.stages[0].challenge.id, "a");
    assert_eq!(session.stages[1].challenge.id, "b");
}

#[test]
fn session_new_with_empty_stages() {
    let session = Session::new(vec![]);
    assert_eq!(session.stages.len(), 0);
}

#[test]
fn session_clone() {
    let stages = vec![sample_stage("a")];
    let session = Session::new(stages);
    let cloned = session.clone();

    assert_eq!(session.stages.len(), cloned.stages.len());
}

#[test]
fn session_result_completion_status_covers_cases() {
    let mut result = SessionResult::new();
    assert_eq!(
        result.get_session_completion_status(),
        "No challenges attempted"
    );

    result.stages_completed = 2;
    assert_eq!(
        result.get_session_completion_status(),
        "Perfect session! 2 challenges completed"
    );

    result.stages_skipped = 1;
    assert_eq!(
        result.get_session_completion_status(),
        "2 completed, 1 skipped"
    );
}

#[test]
fn session_result_new_initializes_metrics() {
    let result = SessionResult::new();

    assert_eq!(result.stages_completed, 0);
    assert_eq!(result.stages_attempted, 0);
    assert_eq!(result.stages_skipped, 0);
    assert_eq!(result.overall_accuracy, 0.0);
    assert_eq!(result.overall_wpm, 0.0);
    assert_eq!(result.overall_cpm, 0.0);
    assert_eq!(result.valid_keystrokes, 0);
    assert_eq!(result.valid_mistakes, 0);
    assert_eq!(result.invalid_keystrokes, 0);
    assert_eq!(result.invalid_mistakes, 0);
    assert_eq!(result.best_stage_wpm, 0.0);
    assert_eq!(result.worst_stage_wpm, f64::MAX);
    assert_eq!(result.best_stage_accuracy, 0.0);
    assert_eq!(result.worst_stage_accuracy, f64::MAX);
    assert_eq!(result.session_score, 0.0);
    assert!(!result.session_successful);
    assert!(result.stage_results.is_empty());
}

#[test]
fn session_result_default() {
    let result = SessionResult::default();
    assert_eq!(result.stages_completed, 0);
    assert_eq!(result.worst_stage_wpm, f64::MAX);
}

#[test]
fn session_result_clone() {
    let mut result = SessionResult::new();
    result.session_score = 1000.0;
    result.stages_completed = 5;

    let cloned = result.clone();
    assert_eq!(result.session_score, cloned.session_score);
    assert_eq!(result.stages_completed, cloned.stages_completed);
}

#[test]
fn session_result_partial_eq() {
    let result1 = SessionResult::new();
    let result2 = SessionResult::new();

    // Note: PartialEq compares all fields including Instant which may differ
    // This test verifies the trait exists
    let _ = result1 == result2;
}

#[test]
fn session_result_with_durations() {
    let mut result = SessionResult::new();
    result.session_duration = Duration::from_secs(60);
    result.valid_session_duration = Duration::from_secs(50);
    result.invalid_session_duration = Duration::from_secs(10);

    assert_eq!(result.session_duration.as_secs(), 60);
    assert_eq!(result.valid_session_duration.as_secs(), 50);
    assert_eq!(result.invalid_session_duration.as_secs(), 10);
}

#[test]
fn session_result_with_metrics() {
    let mut result = SessionResult::new();
    result.overall_accuracy = 95.5;
    result.overall_wpm = 60.0;
    result.overall_cpm = 300.0;
    result.session_score = 5000.0;

    assert_eq!(result.overall_accuracy, 95.5);
    assert_eq!(result.overall_wpm, 60.0);
    assert_eq!(result.overall_cpm, 300.0);
    assert_eq!(result.session_score, 5000.0);
}

#[test]
fn session_result_best_worst_performance() {
    let mut result = SessionResult::new();
    result.best_stage_wpm = 80.0;
    result.worst_stage_wpm = 40.0;
    result.best_stage_accuracy = 98.0;
    result.worst_stage_accuracy = 85.0;

    assert_eq!(result.best_stage_wpm, 80.0);
    assert_eq!(result.worst_stage_wpm, 40.0);
    assert_eq!(result.best_stage_accuracy, 98.0);
    assert_eq!(result.worst_stage_accuracy, 85.0);
}
