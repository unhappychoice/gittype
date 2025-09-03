use crate::models::SessionResult;
use crate::scoring::{ScoringEngine, StageResult};

// Extension methods for SessionResult will be added here when needed

#[derive(Clone)]
pub struct SessionTracker {
    summary: SessionResult,
}

impl SessionTracker {
    pub fn new() -> Self {
        Self {
            summary: SessionResult::new(),
        }
    }

    pub fn record_stage_completion(
        &mut self,
        stage_name: String,
        stage_result: StageResult,
        engine: &ScoringEngine,
    ) {
        self.summary
            .add_stage_result(stage_name, stage_result, engine);
    }

    pub fn record_skip(&mut self) {
        self.summary.add_skip();
    }

    pub fn record_partial_effort(&mut self, engine: &ScoringEngine, stage_result: &StageResult) {
        self.summary
            .add_partial_effort(engine.total_chars(), stage_result.mistakes);
    }

    pub fn finalize_and_get_summary(mut self) -> SessionResult {
        self.summary.finalize_session();
        self.summary
    }

    pub fn get_current_summary(&self) -> &SessionResult {
        &self.summary
    }
}

impl Default for SessionTracker {
    fn default() -> Self {
        Self::new()
    }
}
