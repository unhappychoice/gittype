use super::{Database, SessionRepository};
use crate::logging::setup_console_logging;
use crate::models::{Challenge, GitRepository, SessionResult};
use crate::scoring::ScoringEngine;
use std::env;
use tempfile::TempDir;

/// Integration test to verify the full session recording pipeline
pub fn test_session_recording_integration() -> crate::Result<()> {
    // Setup logging for testing
    setup_console_logging();
    log::info!("Starting session recording integration test");

    // Create temporary home directory for test
    let temp_dir = TempDir::new()?;
    let old_home = env::var("HOME").ok();
    env::set_var("HOME", temp_dir.path());

    // Create test data
    let git_repo = GitRepository {
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "https://github.com/testuser/testrepo.git".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123".to_string()),
        is_dirty: false,
    };

    let mut session_result = SessionResult::new();
    session_result.stages_completed = 2;
    session_result.stages_attempted = 2;
    session_result.total_keystrokes = 250;
    session_result.total_mistakes = 5;
    session_result.session_score = 85.5;
    session_result.finalize_session();

    // Create mock stage engines
    let stage_engines = vec![
        ("challenge1".to_string(), create_mock_engine(100, 45.0)),
        ("challenge2".to_string(), create_mock_engine(150, 52.0)),
    ];

    // Create mock challenges
    let challenges = vec![
        Challenge::new("challenge1".to_string(), "fn test() {}".to_string())
            .with_language("rust".to_string())
            .with_difficulty_level(crate::game::stage_builder::DifficultyLevel::Easy),
        Challenge::new("challenge2".to_string(), "fn main() {}".to_string())
            .with_language("rust".to_string())
            .with_difficulty_level(crate::game::stage_builder::DifficultyLevel::Normal),
    ];

    // Initialize database first
    let database = Database::new()?;
    database.init()?;

    // Then initialize repository
    let repository = SessionRepository::new()?;
    log::info!("Database and SessionRepository initialized successfully");

    // Record session
    let session_id = repository.record_session(
        &session_result,
        Some(&git_repo),
        "Custom",
        Some("Normal"),
        &stage_engines,
        &challenges,
    )?;
    log::info!("Session recorded with ID: {}", session_id);

    // Verify data was saved by querying it back
    let verification_repository = SessionRepository::new()?;

    // Get all repositories
    let repositories = verification_repository.get_all_repositories()?;
    assert!(
        !repositories.is_empty(),
        "Should have at least one repository"
    );

    let repo = repositories.first().unwrap();
    assert_eq!(repo.user_name, "testuser");
    assert_eq!(repo.repository_name, "testrepo");
    log::info!(
        "Repository verification successful: {}/{}",
        repo.user_name,
        repo.repository_name
    );

    // Get sessions for the repository
    let sessions = verification_repository.get_repository_history(repo.id)?;
    assert!(!sessions.is_empty(), "Should have at least one session");

    let stored_session = sessions.first().unwrap();
    assert_eq!(stored_session.game_mode, "Custom");
    assert_eq!(stored_session.difficulty_level, Some("Normal".to_string()));
    log::info!(
        "Session verification successful: mode={}, difficulty={:?}",
        stored_session.game_mode,
        stored_session.difficulty_level
    );

    // Restore original HOME
    match old_home {
        Some(home) => env::set_var("HOME", home),
        None => env::remove_var("HOME"),
    }

    log::info!("Session recording integration test completed successfully");
    Ok(())
}

/// Create a mock scoring engine for testing
fn create_mock_engine(total_chars: usize, _wpm: f64) -> ScoringEngine {
    let mut engine = ScoringEngine::new("test content".to_string());

    // Start scoring to initialize the engine
    engine.start();

    // Simulate typing the content
    for i in 0..total_chars {
        engine.record_keystroke('a', i);
    }

    engine
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration() {
        // Only run this test manually, as it requires file system access
        // and can conflict with other tests
        if env::var("RUN_INTEGRATION_TEST").is_ok() {
            test_session_recording_integration().expect("Integration test should pass");
        }
    }
}
