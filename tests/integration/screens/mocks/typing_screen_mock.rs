use gittype::domain::events::EventBus;
use gittype::domain::models::Challenge;
use gittype::domain::services::scoring::tracker::StageTracker;
use gittype::presentation::game::stage_repository::StageRepository;
use gittype::presentation::game::{GameData, SessionManager};
use gittype::presentation::tui::screens::typing_screen::TypingScreen;
use gittype::presentation::tui::ScreenDataProvider;
use gittype::Result;
use std::sync::{Arc, Mutex};

pub struct MockTypingScreenDataProvider;

impl ScreenDataProvider for MockTypingScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        // TypingScreen's init_with_data doesn't use the data
        // Challenge loading is done via load_current_challenge()
        Ok(Box::new(()))
    }
}

/// Helper function to create TypingScreen with optional challenge
pub fn create_typing_screen_with_challenge(
    event_bus: EventBus,
    code: Option<&str>,
) -> TypingScreen {
    let game_data = Arc::new(Mutex::new(GameData::default()));

    // Create a mock StageRepository with challenge if provided
    let stage_repository = if let Some(code_content) = code {
        let challenge = Challenge {
            id: "test_1".to_string(),
            source_file_path: Some("test.rs".to_string()),
            code_content: code_content.to_string(),
            start_line: Some(1),
            end_line: Some(code_content.lines().count()),
            language: Some("rust".to_string()),
            comment_ranges: vec![],
            difficulty_level: Some(gittype::domain::models::DifficultyLevel::Easy),
        };

        let mut repo = StageRepository::empty();
        repo.set_cached_challenges(vec![challenge]);
        Arc::new(Mutex::new(repo))
    } else {
        Arc::new(Mutex::new(StageRepository::empty()))
    };

    let session_manager = Arc::new(Mutex::new(SessionManager::with_stage_repository(
        stage_repository,
    )));

    // If code is provided, initialize and start session, then add to SessionManager for tracking
    if let Some(code_content) = code {
        let challenge = Challenge {
            id: "test_1".to_string(),
            source_file_path: Some("test.rs".to_string()),
            code_content: code_content.to_string(),
            start_line: Some(1),
            end_line: Some(code_content.lines().count()),
            language: Some("rust".to_string()),
            comment_ranges: vec![],
            difficulty_level: Some(gittype::domain::models::DifficultyLevel::Easy),
        };

        let stage_tracker = StageTracker::new(code_content.to_string());

        if let Ok(mut manager) = session_manager.lock() {
            use gittype::presentation::game::session_manager::SessionState;
            use std::time::Instant;

            // Manually set state to InProgress
            manager.set_state(SessionState::InProgress {
                current_stage: 0,
                started_at: Instant::now(),
            });

            // Set current stage tracker for metrics display
            manager.set_current_stage_tracker(stage_tracker.clone());

            // Add stage data for tracking
            manager.add_stage_data_instance("test_stage".to_string(), stage_tracker, challenge);
        }
    }

    let mut screen = TypingScreen::new(event_bus, game_data, session_manager);

    // Load challenge if provided
    if code.is_some() {
        let result = screen.load_current_challenge();
        if let Ok(false) = result {
            eprintln!("WARNING: load_current_challenge() returned false - no challenge loaded");
        } else if let Err(e) = result {
            eprintln!("ERROR: load_current_challenge() failed: {:?}", e);
        }
    }

    screen
}
