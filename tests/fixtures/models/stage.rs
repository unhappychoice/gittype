//! Test fixtures for Stage model

use super::challenge;
use gittype::domain::models::Stage;

/// Creates a default test Stage
pub fn build() -> Stage {
    Stage::new(challenge::build(), 1)
}

/// Creates a Stage with custom stage number
pub fn build_with_number(stage_number: usize) -> Stage {
    Stage::new(challenge::build(), stage_number)
}

/// Creates a Stage with custom Challenge ID
pub fn build_with_challenge_id(id: &str) -> Stage {
    Stage::new(challenge::build_with_id(id), 1)
}

/// Creates multiple Stages with sequential IDs
pub fn build_multiple(count: usize) -> Vec<Stage> {
    (1..=count)
        .map(|i| {
            let challenge = challenge::build_with_id(&format!("challenge-{}", i));
            Stage::new(challenge, i)
        })
        .collect()
}
