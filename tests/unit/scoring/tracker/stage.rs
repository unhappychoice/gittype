use gittype::scoring::tracker::{StageInput, StageTracker};
use std::time::Duration;

#[test]
fn test_new_stage_tracker() {
    let tracker = StageTracker::new("test".to_string());
    let data = tracker.get_data();
    assert_eq!(data.target_text, "test");
    assert!(data.keystrokes.is_empty());
    assert_eq!(data.current_streak, 0);
    assert!(data.streaks.is_empty());
    assert_eq!(data.elapsed_time, Duration::ZERO);
    assert!(!data.is_finished);
    assert!(!data.was_skipped);
    assert!(!data.was_failed);
}

#[test]
fn test_record_keystroke_correct() {
    let mut tracker = StageTracker::new("hello".to_string());
    tracker.record(StageInput::Start);
    std::thread::sleep(Duration::from_millis(10)); // Simulate time passing
    tracker.record(StageInput::Keystroke { ch: 'h', position: 0 });
    let data = tracker.get_data();
    assert_eq!(data.keystrokes.len(), 1);
    assert!(data.keystrokes[0].is_correct);
    assert_eq!(data.current_streak, 1);
    assert!(data.streaks.is_empty());
}

#[test]
fn test_record_keystroke_incorrect() {
    let mut tracker = StageTracker::new("hello".to_string());
    tracker.record(StageInput::Start);
    std::thread::sleep(Duration::from_millis(10));
    tracker.record(StageInput::Keystroke { ch: 'x', position: 0 });
    let data = tracker.get_data();
    assert_eq!(data.keystrokes.len(), 1);
    assert!(!data.keystrokes[0].is_correct);
    assert_eq!(data.current_streak, 0);
    assert!(data.streaks.is_empty());
}

#[test]
fn test_streaks() {
    let mut tracker = StageTracker::new("abc".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Keystroke { ch: 'a', position: 0 }); // Correct
    tracker.record(StageInput::Keystroke { ch: 'b', position: 1 }); // Correct
    tracker.record(StageInput::Keystroke { ch: 'x', position: 2 }); // Incorrect
    tracker.record(StageInput::Keystroke { ch: 'c', position: 2 }); // Correct (after mistake)
    let data = tracker.get_data();
    assert_eq!(data.streaks, vec![2]);
    assert_eq!(data.current_streak, 1);
}

#[test]
fn test_finish_updates_duration() {
    let mut tracker = StageTracker::new("test".to_string());
    tracker.record(StageInput::Start);
    std::thread::sleep(Duration::from_millis(100));
    tracker.record(StageInput::Finish);
    let data = tracker.get_data();
    assert!(data.elapsed_time > Duration::ZERO);
    assert!(data.is_finished);
}

#[test]
fn test_pause_resume() {
    let mut tracker = StageTracker::new("test".to_string());
    tracker.record(StageInput::Start);
    std::thread::sleep(Duration::from_millis(50));
    tracker.record(StageInput::Pause);
    std::thread::sleep(Duration::from_millis(100)); // Paused time
    tracker.record(StageInput::Resume);
    std::thread::sleep(Duration::from_millis(50));
    tracker.record(StageInput::Finish);
    let data = tracker.get_data();
    // Total active time should be around 100ms (50ms before pause + 50ms after resume)
    assert!((data.elapsed_time.as_millis() as i64 - 100).abs() < 20);
}

#[test]
fn test_skip_stage() {
    let mut tracker = StageTracker::new("test".to_string());
    tracker.record(StageInput::Start);
    std::thread::sleep(Duration::from_millis(50));
    tracker.record(StageInput::Skip);
    let data = tracker.get_data();
    assert!(data.was_skipped);
    assert!(data.is_finished);
    assert!(data.elapsed_time > Duration::ZERO);
}

#[test]
fn test_fail_stage() {
    let mut tracker = StageTracker::new("test".to_string());
    tracker.record(StageInput::Start);
    std::thread::sleep(Duration::from_millis(50));
    tracker.record(StageInput::Fail);
    let data = tracker.get_data();
    assert!(data.was_failed);
    assert!(data.is_finished);
    assert!(data.elapsed_time > Duration::ZERO);
}

#[test]
fn test_keystrokes_after_finish_ignored() {
    let mut tracker = StageTracker::new("test".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Finish);
    tracker.record(StageInput::Keystroke { ch: 'a', position: 0 });
    let data = tracker.get_data();
    assert_eq!(data.keystrokes.len(), 0);
}

#[test]
fn test_new_with_path() {
    let tracker = StageTracker::new_with_path("test".to_string(), "/path/to/challenge".to_string());
    let data = tracker.get_data();
    assert_eq!(data.challenge_path, "/path/to/challenge");
}

#[test]
fn test_empty_target_text() {
    let mut tracker = StageTracker::new("".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Keystroke { ch: 'a', position: 0 });
    let data = tracker.get_data();
    assert_eq!(data.keystrokes.len(), 1);
    assert!(!data.keystrokes[0].is_correct);
}

#[test]
fn test_position_out_of_bounds() {
    let mut tracker = StageTracker::new("a".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Keystroke { ch: 'a', position: 1 }); // Position out of bounds
    let data = tracker.get_data();
    assert_eq!(data.keystrokes.len(), 1);
    assert!(!data.keystrokes[0].is_correct);
}
