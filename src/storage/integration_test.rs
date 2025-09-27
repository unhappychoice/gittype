use super::{Database, SessionRepository};
use crate::logging::setup_console_logging;
use crate::domain::models::{Challenge, GitRepository, SessionResult};
use crate::scoring::tracker::{StageInput, StageTracker};

pub fn test_session_recording_integration() -> crate::Result<()> {
    // Setup logging for testing
    setup_console_logging();
    log::info!("Starting session recording integration test");

    // Create test data
    let git_repo = GitRepository {
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "https://github.com/testuser/testrepo.git".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let mut session_result = SessionResult::new();
    session_result.stages_completed = 2;
    session_result.stages_attempted = 2;
    session_result.valid_keystrokes = 250;
    session_result.valid_mistakes = 5;
    // session_score is now calculated dynamically, no need to set manually

    // Create mock stage engines
    let stage_engines = vec![
        ("challenge1".to_string(), create_mock_engine(100, 45.0)),
        ("challenge2".to_string(), create_mock_engine(150, 52.0)),
    ];

    // Create mock challenges
    let challenges = vec![
        Challenge::new("challenge1".to_string(), "fn test() {}".to_string())
            .with_language("rust".to_string())
            .with_difficulty_level(crate::game::DifficultyLevel::Easy),
        Challenge::new("challenge2".to_string(), "fn main() {}".to_string())
            .with_language("rust".to_string())
            .with_difficulty_level(crate::game::DifficultyLevel::Normal),
    ];

    // Create test database and repository
    let database = Database::new_test()?;
    database.init()?;

    // Create repository with test database
    let repository = SessionRepository::new()?;

    // Test session recording
    log::info!("Recording session data...");
    let session_id = repository.record_session(
        &session_result,
        Some(&git_repo),
        "Normal",
        Some("Easy"),
        &stage_engines,
        &challenges,
    )?;
    log::info!("Session recorded with ID: {}", session_id);

    // Stage recording is now handled within record_session, no separate recording needed

    // Session and stage data verification would require additional methods
    // For now, just verify that recording succeeded
    assert!(session_id > 0, "Session should be recorded with valid ID");
    log::info!("Session and stage data recorded successfully");

    log::info!("Integration test completed successfully");
    Ok(())
}

/// Create a mock scoring engine for testing
fn create_mock_engine(total_chars: usize, _wpm: f64) -> StageTracker {
    let mut engine = StageTracker::new("test content".to_string());

    // Start scoring to initialize the engine
    engine.record(StageInput::Start);

    // Simulate typing the content
    for i in 0..total_chars {
        engine.record(StageInput::Keystroke {
            ch: 'a',
            position: i,
        });
    }

    engine
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_integration() {
        // Only run this test manually, as it requires file system access
        // and can conflict with other tests
        if env::var("RUN_INTEGRATION_TEST").is_ok() {
            test_session_recording_integration().expect("Integration test should pass");
        }
    }
}
