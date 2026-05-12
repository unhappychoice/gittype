use crate::integration::screens::mocks::stage_summary_screen_mock::MockStageSummaryDataProvider;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::{EventBus, EventBusInterface};
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::scoring::{
    SessionTracker, SessionTrackerInterface, TotalTracker, TotalTrackerInterface,
};
use gittype::domain::services::session_manager_service::SessionManagerInterface;
use gittype::domain::services::stage_builder_service::{StageRepository, StageRepositoryInterface};
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::domain::services::SessionManager;
use gittype::domain::stores::{ChallengeStore, RepositoryStore, SessionStore};
use gittype::domain::stores::{
    ChallengeStoreInterface, RepositoryStoreInterface, SessionStoreInterface,
};
use gittype::presentation::tui::screens::stage_summary_screen::{
    StageSummaryData, StageSummaryScreen,
};
use gittype::presentation::tui::screens::ResultAction;
use gittype::presentation::tui::Screen;
use gittype::GitTypeError;
use ratatui::{backend::TestBackend, buffer::Buffer, Terminal};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug)]
struct NonConcreteSessionManager;

impl SessionManagerInterface for NonConcreteSessionManager {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// Helper function to create StageSummaryScreen with all required dependencies
fn create_stage_summary_screen(event_bus: Arc<dyn EventBusInterface>) -> StageSummaryScreen {
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    let challenge_store =
        Arc::new(ChallengeStore::new_for_test()) as Arc<dyn ChallengeStoreInterface>;
    let repository_store =
        Arc::new(RepositoryStore::new_for_test()) as Arc<dyn RepositoryStoreInterface>;
    let session_store = Arc::new(SessionStore::new_for_test()) as Arc<dyn SessionStoreInterface>;
    let stage_repository = Arc::new(StageRepository::new(
        None,
        challenge_store,
        repository_store,
        session_store,
    )) as Arc<dyn StageRepositoryInterface>;
    let session_tracker: Arc<dyn SessionTrackerInterface> = Arc::new(SessionTracker::default());
    let total_tracker: Arc<dyn TotalTrackerInterface> = Arc::new(TotalTracker::default());
    let session_manager = Arc::new(SessionManager::new_with_dependencies(
        event_bus.clone(),
        stage_repository,
        session_tracker,
        total_tracker,
    )) as Arc<dyn SessionManagerInterface>;

    StageSummaryScreen::new(event_bus, theme_service, session_manager)
}

fn create_stage_summary_screen_with_session_manager(
    event_bus: Arc<dyn EventBusInterface>,
    session_manager: Arc<dyn SessionManagerInterface>,
) -> StageSummaryScreen {
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;

    StageSummaryScreen::new(event_bus, theme_service, session_manager)
}

fn stage_result() -> gittype::domain::models::StageResult {
    gittype::domain::models::StageResult {
        challenge_score: 123.0,
        cpm: 240.0,
        wpm: 48.0,
        accuracy: 97.5,
        completion_time: Duration::from_secs_f64(10.5),
        mistakes: 1,
        keystrokes: 42,
        consistency_streaks: vec![4, 5],
        rank_name: "Compiler".to_string(),
        tier_name: "Master".to_string(),
        tier_position: 2,
        tier_total: 10,
        overall_position: 12,
        overall_total: 100,
        was_failed: false,
        was_skipped: false,
        challenge_path: "src/lib.rs".to_string(),
    }
}

fn buffer_text(buffer: &Buffer) -> String {
    (0..buffer.area.height)
        .map(|row| {
            (0..buffer.area.width)
                .map(|column| buffer[(column, row)].symbol().to_string())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

screen_snapshot_test!(
    test_stage_summary_screen_snapshot,
    StageSummaryScreen,
    create_stage_summary_screen(Arc::new(EventBus::new())),
    provider = MockStageSummaryDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_stage_summary_screen_esc_navigates_to_session_failure,
    StageSummaryScreen,
    create_stage_summary_screen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockStageSummaryDataProvider
);

screen_key_event_test!(
    test_stage_summary_screen_ctrl_c_navigates_to_session_failure,
    StageSummaryScreen,
    create_stage_summary_screen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockStageSummaryDataProvider
);

screen_key_event_test!(
    test_stage_summary_screen_space_continues,
    StageSummaryScreen,
    create_stage_summary_screen,
    NavigateTo,
    KeyCode::Char(' '),
    KeyModifiers::empty(),
    MockStageSummaryDataProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_stage_summary_screen_basic_methods,
    StageSummaryScreen,
    create_stage_summary_screen(Arc::new(EventBus::new())),
    gittype::presentation::tui::ScreenType::StageSummary,
    false,
    MockStageSummaryDataProvider
);

#[test]
fn test_stage_summary_screen_records_escape_action_result() {
    let screen = create_stage_summary_screen(Arc::new(EventBus::new()));

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()))
        .unwrap();

    assert!(matches!(
        screen.get_action_result(),
        Some(ResultAction::BackToTitle)
    ));
}

#[test]
fn test_stage_summary_screen_renders_completed_stage_progress() {
    let screen = create_stage_summary_screen(Arc::new(EventBus::new()));
    screen
        .init_with_data(Box::new(StageSummaryData {
            stage_result: stage_result(),
            current_stage: 3,
            total_stages: 3,
            is_completed: true,
        }))
        .unwrap();

    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| {
            screen.render_ratatui(frame).unwrap();
        })
        .unwrap();

    let output = buffer_text(terminal.backend().buffer());
    assert!(output.contains("=== STAGE 3 COMPLETE ==="));
    assert!(output.contains("Stage 3 of 3"));
    assert!(!output.contains("Next stage starting..."));
}

#[test]
fn test_stage_summary_screen_init_without_data_rejects_non_concrete_session_manager() {
    let screen = create_stage_summary_screen_with_session_manager(
        Arc::new(EventBus::new()),
        Arc::new(NonConcreteSessionManager),
    );

    let error = screen.init_with_data(Box::new(())).unwrap_err();

    assert!(matches!(
        error,
        GitTypeError::TerminalError(message)
            if message == "Failed to get SessionManager"
    ));
}

#[test]
fn test_stage_summary_screen_space_defaults_to_animation_for_non_concrete_session_manager() {
    let event_bus = Arc::new(EventBus::new());
    let published_events = Arc::new(Mutex::new(Vec::<NavigateTo>::new()));
    let observed_events = Arc::clone(&published_events);
    let screen = create_stage_summary_screen_with_session_manager(
        event_bus.clone(),
        Arc::new(NonConcreteSessionManager),
    );

    event_bus.subscribe(move |event: &NavigateTo| {
        observed_events.lock().unwrap().push(event.clone());
    });

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()))
        .unwrap();

    let events = published_events.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert!(matches!(
        events[0],
        NavigateTo::Replace(gittype::presentation::tui::ScreenType::Animation)
    ));
}
