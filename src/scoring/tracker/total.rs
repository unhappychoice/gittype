use crate::models::SessionResult;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

// Global total tracker for game-wide statistics
pub static GLOBAL_TOTAL_TRACKER: Lazy<Arc<Mutex<Option<TotalTracker>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

/// Total level raw data tracking
#[derive(Clone)]
pub struct TotalTracker {
    session_results: Vec<SessionResult>,
}

impl TotalTracker {
    pub fn new() -> Self {
        Self {
            session_results: Vec::new(),
        }
    }

    pub fn record(&mut self, session_result: SessionResult) {
        self.session_results.push(session_result);
    }

    pub fn get_data(&self) -> TotalTrackerData {
        TotalTrackerData {
            session_results: self.session_results.clone(),
        }
    }

    pub fn initialize_global_instance(tracker: TotalTracker) {
        if let Ok(mut global) = GLOBAL_TOTAL_TRACKER.lock() {
            *global = Some(tracker);
        }
    }
}

impl Default for TotalTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct TotalTrackerData {
    pub session_results: Vec<SessionResult>,
}
