use gittype::domain::models::StageResult;
use gittype::scoring::calculator::SessionCalculator;
use gittype::scoring::tracker::SessionTracker;
use gittype::scoring::ScoreCalculator;
use std::time::Duration;

const EPSILON: f64 = 0.001;

#[test]
fn test_calculate_empty_session() {
    let tracker = SessionTracker::new();
    let result = SessionCalculator::calculate(&tracker);

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
    assert_eq!(result.worst_stage_wpm, 0.0);
    assert_eq!(result.best_stage_accuracy, 0.0);
    assert_eq!(result.worst_stage_accuracy, 0.0);
    assert_eq!(result.session_score, 0.0);
    assert!(result.session_successful);
}

#[test]
fn test_calculate_single_completed_stage() {
    let mut tracker = SessionTracker::new();
    let stage_result = StageResult {
        cpm: 100.0,
        wpm: 20.0,
        accuracy: 90.0,
        keystrokes: 100,
        mistakes: 10,
        completion_time: Duration::from_secs(60),
        challenge_score: 5000.0,
        ..Default::default()
    };
    tracker.record(stage_result);
    let result = SessionCalculator::calculate(&tracker);

    assert_eq!(result.stages_completed, 1);
    assert_eq!(result.stages_attempted, 1);
    assert_eq!(result.stages_skipped, 0);
    assert!((result.overall_accuracy - 90.0).abs() < EPSILON);
    assert!((result.overall_wpm - 20.0).abs() < EPSILON);
    assert!((result.overall_cpm - 100.0).abs() < EPSILON);
    assert_eq!(result.valid_keystrokes, 100);
    assert_eq!(result.valid_mistakes, 10);
    assert_eq!(result.invalid_keystrokes, 0);
    assert_eq!(result.invalid_mistakes, 0);
    assert!((result.best_stage_wpm - 20.0).abs() < EPSILON);
    assert!((result.worst_stage_wpm - 20.0).abs() < EPSILON);
    assert!((result.best_stage_accuracy - 90.0).abs() < EPSILON);
    assert!((result.worst_stage_accuracy - 90.0).abs() < EPSILON);
    assert!((result.session_score - 2700.0).abs() < EPSILON);
    assert!(result.session_successful);
}

#[test]
fn test_calculate_multiple_stages_mixed() {
    let mut tracker = SessionTracker::new();
    // Completed stage
    let stage_result1 = StageResult {
        cpm: 100.0,
        wpm: 20.0,
        accuracy: 90.0,
        keystrokes: 100,
        mistakes: 10,
        completion_time: Duration::from_secs(60),
        challenge_score: 5000.0,
        ..Default::default()
    };
    // Skipped stage
    let stage_result2 = StageResult {
        was_skipped: true,
        completion_time: Duration::from_secs(10),
        ..Default::default()
    };
    // Failed stage
    let stage_result3 = StageResult {
        was_failed: true,
        completion_time: Duration::from_secs(5),
        ..Default::default()
    };
    // Another completed stage
    let stage_result4 = StageResult {
        cpm: 150.0,
        wpm: 30.0,
        accuracy: 95.0,
        keystrokes: 150,
        mistakes: 7,
        completion_time: Duration::from_secs(60),
        challenge_score: 7000.0,
        ..Default::default()
    };

    tracker.record(stage_result1);
    tracker.record(stage_result2);
    tracker.record(stage_result3);
    tracker.record(stage_result4);

    let result = SessionCalculator::calculate(&tracker);

    assert_eq!(result.stages_completed, 2);
    assert_eq!(result.stages_attempted, 4);
    assert_eq!(result.stages_skipped, 1);

    // Overall metrics should only consider completed stages
    // Total valid keystrokes = 100 + 150 = 250
    // Total valid mistakes = 10 + 7 = 17
    // Total valid duration = 60 + 60 = 120 secs
    // Overall CPM = (250 / 120) * 60 = 125
    // Overall WPM = 125 / 5 = 25
    // Overall Accuracy = ((250 - 17) / 250) * 100 = (233 / 250) * 100 = 93.2
    assert!((result.overall_accuracy - 93.2).abs() < EPSILON);
    assert!((result.overall_wpm - 25.0).abs() < EPSILON);
    assert!((result.overall_cpm - 125.0).abs() < EPSILON);
    assert_eq!(result.valid_keystrokes, 250);
    assert_eq!(result.valid_mistakes, 17);
    assert_eq!(result.invalid_keystrokes, 0); // Default stage results have 0 keystrokes
    assert_eq!(result.invalid_mistakes, 0); // Default stage results have 0 mistakes

    // Best/Worst stage based on challenge_score
    assert!((result.best_stage_wpm - 30.0).abs() < EPSILON);
    assert!((result.worst_stage_wpm - 0.0).abs() < EPSILON); // Corrected expected value
    assert!((result.best_stage_accuracy - 95.0).abs() < EPSILON);
    assert!((result.worst_stage_accuracy - 0.0).abs() < EPSILON); // Corrected expected value

    let expected_session_score = {
        let valid_keystrokes: usize = result.valid_keystrokes;
        let valid_mistakes: usize = result.valid_mistakes;
        let valid_session_duration: Duration = result.valid_session_duration;

        if valid_keystrokes > 0 {
            let elapsed_secs = valid_session_duration.as_secs_f64().max(0.1);
            let cpm = (valid_keystrokes as f64 / elapsed_secs) * 60.0;
            let accuracy = ((valid_keystrokes.saturating_sub(valid_mistakes)) as f64
                / valid_keystrokes as f64)
                * 100.0;

            ScoreCalculator::calculate_score_from_metrics(
                cpm,
                accuracy,
                valid_mistakes,
                elapsed_secs,
                valid_keystrokes,
            )
        } else {
            0.0
        }
    };
    assert!((result.session_score - expected_session_score).abs() < EPSILON);
    assert!(!result.session_successful); // Because there was a failed stage
}

#[test]
fn test_calculate_session_successful() {
    let mut tracker = SessionTracker::new();
    let stage_result1 = StageResult {
        cpm: 100.0,
        wpm: 20.0,
        accuracy: 90.0,
        keystrokes: 100,
        mistakes: 10,
        completion_time: Duration::from_secs(60),
        challenge_score: 5000.0,
        ..Default::default()
    };
    let stage_result2 = StageResult {
        cpm: 150.0,
        wpm: 30.0,
        accuracy: 95.0,
        keystrokes: 150,
        mistakes: 7,
        completion_time: Duration::from_secs(60),
        challenge_score: 7000.0,
        ..Default::default()
    };
    tracker.record(stage_result1);
    tracker.record(stage_result2);
    let result = SessionCalculator::calculate(&tracker);
    assert!(result.session_successful);
}

#[test]
fn test_calculate_session_with_no_valid_keystrokes() {
    let mut tracker = SessionTracker::new();
    let stage_result = StageResult {
        cpm: 0.0,
        wpm: 0.0,
        accuracy: 0.0,
        keystrokes: 0,
        mistakes: 0,
        completion_time: Duration::from_secs(10),
        challenge_score: 0.0,
        ..Default::default()
    };
    tracker.record(stage_result);
    let result = SessionCalculator::calculate(&tracker);

    assert_eq!(result.overall_accuracy, 0.0);
    assert_eq!(result.overall_wpm, 0.0);
    assert_eq!(result.overall_cpm, 0.0);
    assert_eq!(result.session_score, 0.0);
}
