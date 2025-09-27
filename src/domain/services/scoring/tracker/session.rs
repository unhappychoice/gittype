use crate::domain::models::StageResult;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::time::Instant;

// Global session tracker for Ctrl+C handler
pub static GLOBAL_SESSION_TRACKER: Lazy<Arc<Mutex<Option<SessionTracker>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

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

    pub fn initialize_global_instance(tracker: SessionTracker) {
        if let Ok(mut global) = GLOBAL_SESSION_TRACKER.lock() {
            *global = Some(tracker);
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
