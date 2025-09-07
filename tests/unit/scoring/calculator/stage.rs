use gittype::scoring::tracker::{StageInput, StageTracker};
use gittype::scoring::calculator::StageCalculator;
use std::time::Duration;

const EPSILON: f64 = 0.001;

#[test]
fn test_calculate_empty_tracker() {
    let tracker = StageTracker::new("test".to_string());
    let result = StageCalculator::calculate(&tracker);
    assert_eq!(result.cpm, 0.0);
    assert_eq!(result.wpm, 0.0);
    assert_eq!(result.accuracy, 0.0);
    assert_eq!(result.keystrokes, 0);
    assert_eq!(result.mistakes, 0);
    assert_eq!(result.challenge_score, 0.0);
    assert_eq!(result.rank_name, "Unranked"); // Default rank for empty tracker
}

#[test]
fn test_calculate_basic_correct() {
    let mut tracker = StageTracker::new("hello".to_string());
    tracker.record(StageInput::Start);
    std::thread::sleep(Duration::from_secs(5));
    tracker.record(StageInput::Keystroke { ch: 'h', position: 0 });
    tracker.record(StageInput::Keystroke { ch: 'e', position: 1 });
    tracker.record(StageInput::Keystroke { ch: 'l', position: 2 });
    tracker.record(StageInput::Keystroke { ch: 'l', position: 3 });
    tracker.record(StageInput::Keystroke { ch: 'o', position: 4 });
    tracker.record(StageInput::Finish);
    let result = StageCalculator::calculate(&tracker);

    assert!(result.cpm > 0.0);
    assert!(result.wpm > 0.0);
    assert!((result.accuracy - 100.0).abs() < EPSILON);
    assert_eq!(result.keystrokes, 5);
    assert_eq!(result.mistakes, 0);
    assert!(result.challenge_score > 0.0);
    assert_eq!(result.rank_name, "Bash Newbie"); // Expected rank for calculated score
}

#[test]
fn test_calculate_with_mistakes() {
    let mut tracker = StageTracker::new("hello".to_string());
    tracker.record(StageInput::Start);
    std::thread::sleep(Duration::from_millis(100));
    tracker.record(StageInput::Keystroke { ch: 'h', position: 0 });
    tracker.record(StageInput::Keystroke { ch: 'x', position: 1 }); // Mistake
    tracker.record(StageInput::Keystroke { ch: 'l', position: 2 });
    tracker.record(StageInput::Finish);
    let result = StageCalculator::calculate(&tracker);

    assert!(result.cpm > 0.0);
    assert!(result.wpm > 0.0);
    assert!((result.accuracy - (2.0/3.0)*100.0).abs() < EPSILON); // 2 correct out of 3 keystrokes
    assert_eq!(result.keystrokes, 3);
    assert_eq!(result.mistakes, 1);
    assert!(result.challenge_score > 0.0);
}

#[test]
fn test_calculate_skipped_stage() {
    let mut tracker = StageTracker::new("test".to_string());
    tracker.record(StageInput::Start);
    std::thread::sleep(Duration::from_millis(50));
    tracker.record(StageInput::Skip);
    let result = StageCalculator::calculate(&tracker);

    assert!(result.was_skipped);
    assert_eq!(result.cpm, 0.1); // Skipped stages should have 0 metrics
    assert_eq!(result.wpm, 0.02);
    assert_eq!(result.accuracy, 0.0);
    assert_eq!(result.keystrokes, 0);
    assert_eq!(result.mistakes, 0);
    assert_eq!(result.challenge_score, 100.0);
}

#[test]
fn test_calculate_failed_stage() {
    let mut tracker = StageTracker::new("test".to_string());
    tracker.record(StageInput::Start);
    std::thread::sleep(Duration::from_millis(50));
    tracker.record(StageInput::Fail);
    let result = StageCalculator::calculate(&tracker);

    assert!(result.was_failed);
    assert_eq!(result.cpm, 0.1); // Failed stages should have 0 metrics
    assert_eq!(result.wpm, 0.02);
    assert_eq!(result.accuracy, 0.0);
    assert_eq!(result.keystrokes, 0);
    assert_eq!(result.mistakes, 0);
    assert_eq!(result.challenge_score, 100.0);
}

#[test]
fn test_calculate_with_pauses() {
    let mut tracker = StageTracker::new("test".to_string());
    tracker.record(StageInput::Start);
    std::thread::sleep(Duration::from_millis(50));
    tracker.record(StageInput::Pause);
    std::thread::sleep(Duration::from_millis(100)); // Paused time
    tracker.record(StageInput::Resume);
    std::thread::sleep(Duration::from_millis(50));
    tracker.record(StageInput::Finish);
    let result = StageCalculator::calculate(&tracker);

    // The elapsed time should only count active typing time
    assert!((result.completion_time.as_millis() as i64 - 100).abs() < 20);
    assert!(result.cpm > 0.0);
}
