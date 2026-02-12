use std::time::Instant;

#[derive(Debug, Clone, Default)]
pub enum SessionState {
    #[default]
    NotStarted,
    InProgress {
        current_stage: usize,
        started_at: Instant,
    },
    Completed {
        started_at: Instant,
        completed_at: Instant,
    },
    Aborted {
        started_at: Instant,
        aborted_at: Instant,
    },
}
