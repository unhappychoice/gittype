use crate::domain::models::SessionResult;
use shaku::Interface;
use std::sync::RwLock;

pub trait TotalTrackerInterface: Interface {
    fn record(&self, session_result: SessionResult);
    fn get_data(&self) -> TotalTrackerData;
    fn reset(&self);
}

#[derive(Default)]
pub struct TotalTrackerState {
    pub session_results: Vec<SessionResult>,
}

/// Total level raw data tracking
#[derive(shaku::Component)]
#[shaku(interface = TotalTrackerInterface)]
pub struct TotalTracker {
    #[shaku(default)]
    state: RwLock<TotalTrackerState>,
}

impl Default for TotalTracker {
    fn default() -> Self {
        Self {
            state: RwLock::new(TotalTrackerState::default()),
        }
    }
}

impl TotalTrackerInterface for TotalTracker {
    fn record(&self, session_result: SessionResult) {
        self.state
            .write()
            .unwrap()
            .session_results
            .push(session_result);
    }

    fn get_data(&self) -> TotalTrackerData {
        TotalTrackerData {
            session_results: self.state.read().unwrap().session_results.clone(),
        }
    }

    fn reset(&self) {
        self.state.write().unwrap().session_results.clear();
    }
}

#[cfg(feature = "test-mocks")]
impl TotalTracker {
    pub fn new_for_test() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct TotalTrackerData {
    pub session_results: Vec<SessionResult>,
}
