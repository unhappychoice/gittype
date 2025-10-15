//! Test fixtures for Session model

use super::stage;
use gittype::domain::models::Session;

/// Creates a default test Session with 3 stages
pub fn build() -> Session {
    Session::new(stage::build_multiple(3))
}

/// Creates a Session with single stage
pub fn build_single_stage() -> Session {
    Session::new(vec![stage::build()])
}

/// Creates a Session with custom number of stages
pub fn build_with_stages(count: usize) -> Session {
    Session::new(stage::build_multiple(count))
}
