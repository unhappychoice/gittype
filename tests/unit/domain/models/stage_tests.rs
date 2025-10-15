use crate::fixtures::models::challenge;
use gittype::domain::models::challenge::Challenge;
use gittype::domain::models::stage::{Stage, StageResult};
use std::time::Duration;

fn sample_challenge() -> Challenge {
    Challenge::new("id".into(), "fn main() {}".into()).with_source_info("src/main.rs".into(), 1, 10)
}

#[test]
fn stage_new_assigns_fields() {
    let challenge = sample_challenge();
    let stage = Stage::new(challenge.clone(), 2);

    assert_eq!(stage.stage_number, 2);
    assert_eq!(stage.challenge.id, "id");
    assert_eq!(stage.challenge.start_line, Some(1));
}

#[test]
fn stage_new_with_different_numbers() {
    let c = challenge::build();
    let stage1 = Stage::new(c.clone(), 1);
    let stage5 = Stage::new(c.clone(), 5);

    assert_eq!(stage1.stage_number, 1);
    assert_eq!(stage5.stage_number, 5);
}

#[test]
fn stage_clone() {
    let stage = Stage::new(sample_challenge(), 3);
    let cloned = stage.clone();

    assert_eq!(stage.stage_number, cloned.stage_number);
    assert_eq!(stage.challenge.id, cloned.challenge.id);
}

#[test]
fn stage_result_default_values_are_sensible() {
    let stage_result = StageResult::default();

    assert_eq!(stage_result.rank_name, "Unranked");
    assert_eq!(stage_result.tier_name, "Beginner");
    assert_eq!(stage_result.completion_time.as_secs(), 0);
    assert!(!stage_result.was_skipped);
    assert!(!stage_result.was_failed);
    assert_eq!(stage_result.consistency_streaks, Vec::<usize>::new());
}

#[test]
fn stage_result_default_numeric_values() {
    let result = StageResult::default();

    assert_eq!(result.cpm, 0.0);
    assert_eq!(result.wpm, 0.0);
    assert_eq!(result.accuracy, 0.0);
    assert_eq!(result.keystrokes, 0);
    assert_eq!(result.mistakes, 0);
    assert_eq!(result.challenge_score, 0.0);
    assert_eq!(result.tier_position, 0);
    assert_eq!(result.tier_total, 0);
    assert_eq!(result.overall_position, 0);
    assert_eq!(result.overall_total, 0);
}

#[test]
fn stage_result_with_performance_metrics() {
    let result = StageResult {
        cpm: 450.0,
        wpm: 90.0,
        accuracy: 98.5,
        keystrokes: 500,
        mistakes: 8,
        ..Default::default()
    };

    assert_eq!(result.cpm, 450.0);
    assert_eq!(result.wpm, 90.0);
    assert_eq!(result.accuracy, 98.5);
    assert_eq!(result.keystrokes, 500);
    assert_eq!(result.mistakes, 8);
}

#[test]
fn stage_result_with_completion_data() {
    let result = StageResult {
        completion_time: Duration::from_secs(30),
        challenge_score: 5000.0,
        challenge_path: "src/lib.rs:10-20".to_string(),
        ..Default::default()
    };

    assert_eq!(result.completion_time.as_secs(), 30);
    assert_eq!(result.challenge_score, 5000.0);
    assert_eq!(result.challenge_path, "src/lib.rs:10-20");
}

#[test]
fn stage_result_with_rank_data() {
    let result = StageResult {
        rank_name: "Code Ninja".to_string(),
        tier_name: "Expert".to_string(),
        tier_position: 5,
        tier_total: 10,
        overall_position: 25,
        overall_total: 100,
        ..Default::default()
    };

    assert_eq!(result.rank_name, "Code Ninja");
    assert_eq!(result.tier_name, "Expert");
    assert_eq!(result.tier_position, 5);
    assert_eq!(result.tier_total, 10);
    assert_eq!(result.overall_position, 25);
    assert_eq!(result.overall_total, 100);
}

#[test]
fn stage_result_skipped() {
    let result = StageResult {
        was_skipped: true,
        ..Default::default()
    };

    assert!(result.was_skipped);
    assert!(!result.was_failed);
}

#[test]
fn stage_result_failed() {
    let result = StageResult {
        was_failed: true,
        ..Default::default()
    };

    assert!(result.was_failed);
    assert!(!result.was_skipped);
}

#[test]
fn stage_result_with_consistency_streaks() {
    let result = StageResult {
        consistency_streaks: vec![5, 10, 3, 15],
        ..Default::default()
    };

    assert_eq!(result.consistency_streaks.len(), 4);
    assert_eq!(result.consistency_streaks[3], 15);
}

#[test]
fn stage_result_clone() {
    let result = StageResult {
        wpm: 100.0,
        was_skipped: true,
        ..Default::default()
    };

    let cloned = result.clone();
    assert_eq!(result.wpm, cloned.wpm);
    assert_eq!(result.was_skipped, cloned.was_skipped);
}

#[test]
fn stage_result_partial_eq() {
    let result1 = StageResult::default();
    let result2 = StageResult::default();

    assert_eq!(result1, result2);

    let result3 = StageResult {
        wpm: 50.0,
        ..Default::default()
    };
    assert_ne!(result1, result3);
}
