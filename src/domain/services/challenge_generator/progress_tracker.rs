use crate::presentation::game::models::StepType;
use crate::presentation::game::screens::loading_screen::ProgressReporter;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

/// Tracks and reports progress during challenge generation
pub struct ProgressTracker {
    processed_total: Arc<AtomicUsize>,
    total_work: usize,
    update_frequency: usize,
}

impl ProgressTracker {
    pub fn new(total_work: usize) -> Self {
        Self {
            processed_total: Arc::new(AtomicUsize::new(0)),
            total_work,
            update_frequency: 10, // Update every 10 items by default
        }
    }

    /// Creates a new tracker with custom update frequency
    pub fn with_update_frequency(total_work: usize, update_frequency: usize) -> Self {
        Self {
            processed_total: Arc::new(AtomicUsize::new(0)),
            total_work,
            update_frequency,
        }
    }

    /// Initializes progress reporting
    pub fn initialize(&self, progress: &dyn ProgressReporter) {
        progress.set_file_counts(StepType::Generating, 0, self.total_work, None);
    }

    /// Increments progress and reports if necessary
    pub fn increment_and_report(&self, progress: &dyn ProgressReporter) {
        let current = self.processed_total.fetch_add(1, Ordering::Relaxed) + 1;

        // Report progress at regular intervals or when complete
        if current.is_multiple_of(self.update_frequency) || current == self.total_work {
            progress.set_file_counts(StepType::Generating, current, self.total_work, None);
        }
    }

    /// Forces a progress report with current values
    pub fn force_report(&self, progress: &dyn ProgressReporter) {
        let current = self.processed_total.load(Ordering::Relaxed);
        progress.set_file_counts(StepType::Generating, current, self.total_work, None);
    }

    /// Ensures final progress is reported as 100%
    pub fn finalize(&self, progress: &dyn ProgressReporter) {
        progress.set_file_counts(StepType::Generating, self.total_work, self.total_work, None);
    }

    /// Gets current progress count
    pub fn current_progress(&self) -> usize {
        self.processed_total.load(Ordering::Relaxed)
    }

    /// Gets total work count
    pub fn total_work(&self) -> usize {
        self.total_work
    }

    /// Checks if work is complete
    pub fn is_complete(&self) -> bool {
        self.current_progress() >= self.total_work
    }
}
