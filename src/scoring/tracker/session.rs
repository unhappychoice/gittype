use crate::models::StageResult;
use std::time::Instant;

/// Session level raw data tracking
#[derive(Clone)]
pub struct SessionTracker {
    session_start_time: Instant,
    stage_results: Vec<StageResult>,
}

impl SessionTracker {
    pub fn new() -> Self {
        Self {
            session_start_time: Instant::now(),
            stage_results: Vec::new(),
        }
    }

    pub fn record(&mut self, stage_result: StageResult) {
        self.stage_results.push(stage_result);
    }

    pub fn get_data(&self) -> SessionTrackerData {
        SessionTrackerData {
            session_start_time: self.session_start_time,
            stage_results: self.stage_results.clone(),
        }
    }
}

impl Default for SessionTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct SessionTrackerData {
    pub session_start_time: Instant,
    pub stage_results: Vec<StageResult>,
}
