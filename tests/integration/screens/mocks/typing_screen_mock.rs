use gittype::domain::events::EventBusInterface;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::models::Challenge;
use gittype::domain::services::scoring::tracker::StageTracker;
use gittype::domain::services::scoring::{
    SessionTracker, SessionTrackerInterface, TotalTracker, TotalTrackerInterface,
};
use gittype::domain::services::session_manager_service::SessionManagerInterface;
use gittype::domain::services::stage_builder_service::StageRepository;
use gittype::domain::services::stage_builder_service::StageRepositoryInterface;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::domain::services::SessionManager;
use gittype::domain::stores::{ChallengeStore, RepositoryStore, SessionStore};
use gittype::presentation::tui::screens::typing_screen::TypingScreen;
use gittype::presentation::tui::{Screen, ScreenDataProvider};
use gittype::Result;
use std::sync::Arc;

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
    let (_challenge_store, repository_store, _session_store, stage_repository) =
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

            let challenge_store = Arc::new(ChallengeStore::new_for_test())
                as Arc<dyn gittype::domain::stores::ChallengeStoreInterface>;
            challenge_store.set_challenges(vec![challenge.clone()]);

            let repository_store = Arc::new(RepositoryStore::new_for_test())
                as Arc<dyn gittype::domain::stores::RepositoryStoreInterface>;
            let session_store = Arc::new(SessionStore::new_for_test())
                as Arc<dyn gittype::domain::stores::SessionStoreInterface>;

            let stage_repo = StageRepository::new(
                None,
                challenge_store.clone(),
                repository_store.clone(),
                session_store.clone(),
            );

            // Build difficulty indices for challenge lookup
            stage_repo.build_difficulty_indices();

            let stage_repo_arc = Arc::new(stage_repo) as Arc<dyn StageRepositoryInterface>;

            (
                challenge_store,
                repository_store,
                session_store,
                stage_repo_arc,
            )
        } else {
            let challenge_store = Arc::new(ChallengeStore::new_for_test())
                as Arc<dyn gittype::domain::stores::ChallengeStoreInterface>;
            let repository_store = Arc::new(RepositoryStore::new_for_test())
                as Arc<dyn gittype::domain::stores::RepositoryStoreInterface>;
            let session_store = Arc::new(SessionStore::new_for_test())
                as Arc<dyn gittype::domain::stores::SessionStoreInterface>;

            let stage_repo = Arc::new(StageRepository::new(
                None,
                challenge_store.clone(),
                repository_store.clone(),
                session_store.clone(),
            )) as Arc<dyn StageRepositoryInterface>;

            (challenge_store, repository_store, session_store, stage_repo)
        };

    let session_tracker: Arc<dyn SessionTrackerInterface> = Arc::new(SessionTracker::default());
    let total_tracker: Arc<dyn TotalTrackerInterface> = Arc::new(TotalTracker::default());
    let session_manager = SessionManager::new_with_dependencies(
        event_bus.clone(),
        stage_repository.clone(),
        session_tracker,
        total_tracker,
    );
    let session_manager_arc = Arc::new(session_manager);

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

        use gittype::domain::models::{DifficultyLevel, SessionConfig, SessionState};
        use std::time::Instant;

        // Set difficulty to Easy to match the test challenge
        let config = SessionConfig {
            difficulty: DifficultyLevel::Easy,
            ..Default::default()
        };
        session_manager_arc.set_config(config);

        // Manually set state to InProgress
        session_manager_arc.set_state(SessionState::InProgress {
            current_stage: 0,
            started_at: Instant::now(),
        });

        // Set current stage tracker for metrics display
        session_manager_arc.set_current_stage_tracker(stage_tracker.clone());

        // Add stage data for tracking
        session_manager_arc.add_stage_data("test_stage".to_string(), stage_tracker, challenge);
    }

    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    let screen = TypingScreen::new(
        event_bus,
        theme_service,
        repository_store,
        session_manager_arc as Arc<dyn SessionManagerInterface>,
    );

    // Load challenge if provided
    if code.is_some() {
        // init_with_data will call load_current_challenge internally
        let _ = screen.init_with_data(Box::new(()));
    }

    screen
}
