use super::Event;
use std::any::Any;

// Unified domain event enum for pattern matching
#[derive(Debug, Clone)]
pub enum DomainEvent {
    KeyPressed { key: char, position: usize },
    StageStarted { start_time: std::time::Instant },
    StagePaused,
    StageResumed,
    StageFinalized,
    StageSkipped,
    ChallengeLoaded { text: String, source_path: String },
}

impl Event for DomainEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
