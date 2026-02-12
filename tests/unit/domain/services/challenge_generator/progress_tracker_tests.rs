use gittype::domain::models::loading::StepType;
use gittype::domain::services::challenge_generator::progress_tracker::ProgressTracker;
use gittype::presentation::tui::screens::loading_screen::ProgressReporter;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
struct MockProgressReporter {
    #[allow(clippy::type_complexity)]
    count_calls: Arc<Mutex<Vec<(StepType, usize, usize, Option<String>)>>>,
}

impl MockProgressReporter {
    fn new() -> Self {
        Self::default()
    }

    fn get_count_calls(&self) -> Vec<(StepType, usize, usize, Option<String>)> {
        self.count_calls.lock().unwrap().clone()
    }
}

impl ProgressReporter for MockProgressReporter {
    fn set_step(&self, _step_type: StepType) {}

    fn set_current_file(&self, _file: Option<String>) {}

    fn set_file_counts(
        &self,
        step_type: StepType,
        processed: usize,
        total: usize,
        current_file: Option<String>,
    ) {
        self.count_calls
            .lock()
            .unwrap()
            .push((step_type, processed, total, current_file));
    }
}

#[test]
fn new_creates_tracker() {
    let tracker = ProgressTracker::new(100);
    assert_eq!(tracker.total_work(), 100);
    assert_eq!(tracker.current_progress(), 0);
}

#[test]
fn with_update_frequency_creates_tracker() {
    let tracker = ProgressTracker::with_update_frequency(100, 5);
    assert_eq!(tracker.total_work(), 100);
    assert_eq!(tracker.current_progress(), 0);
}

#[test]
fn initialize_reports_zero_progress() {
    let tracker = ProgressTracker::new(50);
    let progress = MockProgressReporter::new();

    tracker.initialize(&progress);

    let calls = progress.get_count_calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0], (StepType::Generating, 0, 50, None));
}

#[test]
fn increment_and_report_updates_progress() {
    let tracker = ProgressTracker::with_update_frequency(10, 1);
    let progress = MockProgressReporter::new();

    tracker.increment_and_report(&progress);

    assert_eq!(tracker.current_progress(), 1);
    let calls = progress.get_count_calls();
    assert!(!calls.is_empty());
}

#[test]
fn increment_respects_update_frequency() {
    let tracker = ProgressTracker::with_update_frequency(100, 10);
    let progress = MockProgressReporter::new();

    // First 9 increments should not report
    for _ in 0..9 {
        tracker.increment_and_report(&progress);
    }

    let calls = progress.get_count_calls();
    assert_eq!(calls.len(), 0);

    // 10th increment should report
    tracker.increment_and_report(&progress);
    let calls = progress.get_count_calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].1, 10); // processed = 10
}

#[test]
fn increment_reports_on_completion() {
    let tracker = ProgressTracker::with_update_frequency(5, 10);
    let progress = MockProgressReporter::new();

    // Increment to completion (5 times)
    for _ in 0..5 {
        tracker.increment_and_report(&progress);
    }

    // Should report because we reached total_work
    let calls = progress.get_count_calls();
    assert!(!calls.is_empty());
    let last_call = calls.last().unwrap();
    assert_eq!(last_call.1, 5); // processed = 5
    assert_eq!(last_call.2, 5); // total = 5
}

#[test]
fn force_report_always_reports() {
    let tracker = ProgressTracker::with_update_frequency(100, 10);
    let progress = MockProgressReporter::new();

    // Increment once (won't report due to frequency)
    tracker.increment_and_report(&progress);
    assert_eq!(progress.get_count_calls().len(), 0);

    // Force report
    tracker.force_report(&progress);
    let calls = progress.get_count_calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].1, 1); // processed = 1
}

#[test]
fn finalize_reports_completion() {
    let tracker = ProgressTracker::new(50);
    let progress = MockProgressReporter::new();

    tracker.finalize(&progress);

    let calls = progress.get_count_calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0], (StepType::Generating, 50, 50, None));
}

#[test]
fn current_progress_returns_correct_value() {
    let tracker = ProgressTracker::new(10);
    let progress = MockProgressReporter::new();

    assert_eq!(tracker.current_progress(), 0);

    tracker.increment_and_report(&progress);
    assert_eq!(tracker.current_progress(), 1);

    tracker.increment_and_report(&progress);
    tracker.increment_and_report(&progress);
    assert_eq!(tracker.current_progress(), 3);
}

#[test]
fn total_work_returns_correct_value() {
    let tracker = ProgressTracker::new(42);
    assert_eq!(tracker.total_work(), 42);
}

#[test]
fn is_complete_returns_false_initially() {
    let tracker = ProgressTracker::new(10);
    assert!(!tracker.is_complete());
}

#[test]
fn is_complete_returns_true_when_done() {
    let tracker = ProgressTracker::new(3);
    let progress = MockProgressReporter::new();

    for _ in 0..3 {
        tracker.increment_and_report(&progress);
    }

    assert!(tracker.is_complete());
}

#[test]
fn is_complete_returns_true_when_exceeded() {
    let tracker = ProgressTracker::new(2);
    let progress = MockProgressReporter::new();

    for _ in 0..5 {
        tracker.increment_and_report(&progress);
    }

    assert!(tracker.is_complete());
    assert!(tracker.current_progress() > tracker.total_work());
}

#[test]
fn multiple_increments_accumulate() {
    let tracker = ProgressTracker::new(100);
    let progress = MockProgressReporter::new();

    for _ in 0..25 {
        tracker.increment_and_report(&progress);
    }

    assert_eq!(tracker.current_progress(), 25);
}

#[test]
fn progress_reports_use_generating_step_type() {
    let tracker = ProgressTracker::new(10);
    let progress = MockProgressReporter::new();

    tracker.initialize(&progress);
    tracker.finalize(&progress);

    let calls = progress.get_count_calls();
    for call in calls {
        assert_eq!(call.0, StepType::Generating);
    }
}
