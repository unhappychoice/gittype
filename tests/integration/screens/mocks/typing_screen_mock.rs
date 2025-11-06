use gittype::domain::events::EventBusInterface;
use gittype::domain::models::Challenge;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::scoring::tracker::StageTracker;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
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
    event_bus: Arc<dyn EventBusInterface>,
    code: Option<&str>,
) -> TypingScreen {
    let (game_data, stage_repository) = if let Some(code_content) = code {
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

        let mock_game_data = GameData {
            challenges: Some(vec![challenge.clone()]),
            ..Default::default()
        };
        let mock_game_data_arc = Arc::new(Mutex::new(mock_game_data));

        let stage_repo_arc = Arc::new(Mutex::new(StageRepository::new(
            None,
            mock_game_data_arc.clone(),
        )));

        // Build difficulty indices for challenge lookup
        if let Ok(mut repo) = stage_repo_arc.lock() {
            repo.build_difficulty_indices();
        }

        (mock_game_data_arc, stage_repo_arc)
    } else {
        let game_data_arc = Arc::new(Mutex::new(GameData::default()));
        let stage_repo = Arc::new(Mutex::new(StageRepository::new(
            None,
            game_data_arc.clone(),
        )));
        (game_data_arc, stage_repo)
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
            use gittype::domain::models::{DifficultyLevel, SessionConfig, SessionState};
            use std::time::Instant;

            // Set difficulty to Easy to match the test challenge
            let config = SessionConfig {
                difficulty: DifficultyLevel::Easy,
                ..Default::default()
            };
            manager.set_config(config);

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

    let theme_service = Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>;
    let screen = TypingScreen::new(event_bus, theme_service, game_data, session_manager);

    // Load challenge if provided
    if code.is_some() {
        let _ = screen.load_current_challenge();
    }

    screen
}
