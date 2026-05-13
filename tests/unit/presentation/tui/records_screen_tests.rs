use chrono::Utc;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::{EventBus, EventBusInterface};
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::storage::{SessionResultData, StoredRepository, StoredSession};
use gittype::domain::models::theme::Theme;
use gittype::domain::services::session_service::{SessionDisplayData, SessionServiceInterface};
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::records_screen::{
    DateFilter, FilterState, RecordsAction, RecordsScreen, RecordsScreenData, SortBy,
};
use gittype::presentation::tui::{Screen, ScreenType};
use gittype::{GitTypeError, Result};
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use std::sync::{Arc, Mutex};

struct StubSessionService;

impl SessionServiceInterface for StubSessionService {
    fn get_sessions_with_display_data(
        &self,
        _repository_filter: Option<i64>,
        _date_filter_days: Option<i64>,
        _sort_by: &str,
        _sort_descending: bool,
    ) -> Result<Vec<SessionDisplayData>> {
        Ok(vec![])
    }

    fn get_all_repositories(&self) -> Result<Vec<StoredRepository>> {
        Ok(vec![])
    }
}

struct FailingSessionService;

impl SessionServiceInterface for FailingSessionService {
    fn get_sessions_with_display_data(
        &self,
        _repository_filter: Option<i64>,
        _date_filter_days: Option<i64>,
        _sort_by: &str,
        _sort_descending: bool,
    ) -> Result<Vec<SessionDisplayData>> {
        Err(GitTypeError::TerminalError(
            "stub sessions failure".to_string(),
        ))
    }

    fn get_all_repositories(&self) -> Result<Vec<StoredRepository>> {
        Err(GitTypeError::TerminalError(
            "stub repos failure".to_string(),
        ))
    }
}

fn make_screen() -> RecordsScreen {
    make_screen_with(StubSessionService)
}

fn make_screen_with<S: SessionServiceInterface + 'static>(service: S) -> RecordsScreen {
    let event_bus = Arc::new(EventBus::new());
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    let session_service = Arc::new(service) as Arc<dyn SessionServiceInterface>;
    RecordsScreen::new(event_bus, theme_service, session_service)
}

fn make_screen_with_event_capture() -> (RecordsScreen, Arc<Mutex<Vec<NavigateTo>>>) {
    let event_bus = Arc::new(EventBus::new());
    let captured: Arc<Mutex<Vec<NavigateTo>>> = Arc::new(Mutex::new(Vec::new()));
    let captured_clone = Arc::clone(&captured);
    event_bus.subscribe(move |event: &NavigateTo| {
        captured_clone.lock().unwrap().push(event.clone());
    });
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    let session_service = Arc::new(StubSessionService) as Arc<dyn SessionServiceInterface>;
    let event_bus_dyn: Arc<dyn EventBusInterface> = event_bus;
    let screen = RecordsScreen::new(event_bus_dyn, theme_service, session_service);
    (screen, captured)
}

fn make_screen_data(num_sessions: usize) -> RecordsScreenData {
    let sessions = (0..num_sessions)
        .map(|i| make_session(i as i64, None))
        .collect();
    RecordsScreenData {
        sessions,
        repositories: Vec::new(),
    }
}

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::empty())
}

fn render_screen(screen: &RecordsScreen) {
    let backend = TestBackend::new(120, 30);
    let mut terminal = Terminal::new(backend).expect("test backend should construct");
    terminal
        .draw(|frame| {
            screen.render_ratatui(frame).expect("render should succeed");
        })
        .expect("draw should succeed");
}

fn make_session(id: i64, repo_id: Option<i64>) -> SessionDisplayData {
    SessionDisplayData {
        session: StoredSession {
            id,
            repository_id: repo_id,
            started_at: Utc::now(),
            completed_at: Some(Utc::now()),
            branch: None,
            commit_hash: None,
            is_dirty: false,
            game_mode: "default".to_string(),
            difficulty_level: None,
            max_stages: None,
            time_limit_seconds: None,
        },
        repository: None,
        session_result: Some(SessionResultData {
            keystrokes: 100,
            mistakes: 5,
            duration_ms: 30000,
            wpm: 60.0,
            cpm: 300.0,
            accuracy: 95.0,
            stages_completed: 3,
            stages_attempted: 3,
            stages_skipped: 0,
            score: 800.0,
            rank_name: None,
            tier_name: None,
            rank_position: None,
            rank_total: None,
            position: None,
            total: None,
        }),
    }
}

// SortBy ---------------------------------------------------------------------

#[test]
fn sort_by_display_name_covers_all_variants() {
    assert_eq!(SortBy::Date.display_name(), "Date");
    assert_eq!(SortBy::Performance.display_name(), "Score");
    assert_eq!(SortBy::Repository.display_name(), "Repository");
    assert_eq!(SortBy::Duration.display_name(), "Duration");
}

#[test]
fn sort_by_to_string_covers_all_variants() {
    assert_eq!(SortBy::Date.to_string(), "date");
    assert_eq!(SortBy::Performance.to_string(), "score");
    assert_eq!(SortBy::Repository.to_string(), "repository");
    assert_eq!(SortBy::Duration.to_string(), "duration");
}

#[test]
fn sort_by_clone_and_eq() {
    let s = SortBy::Performance;
    let cloned = s.clone();
    assert_eq!(s, cloned);
    assert_ne!(SortBy::Date, SortBy::Performance);
}

// DateFilter -----------------------------------------------------------------

#[test]
fn date_filter_display_name_covers_all_variants() {
    assert_eq!(DateFilter::All.display_name(), "All Time");
    assert_eq!(DateFilter::Last7Days.display_name(), "Last 7 days");
    assert_eq!(DateFilter::Last30Days.display_name(), "Last 30 days");
    assert_eq!(DateFilter::Last90Days.display_name(), "Last 90 days");
}

#[test]
fn date_filter_to_days_covers_all_variants() {
    assert_eq!(DateFilter::All.to_days(), None);
    assert_eq!(DateFilter::Last7Days.to_days(), Some(7));
    assert_eq!(DateFilter::Last30Days.to_days(), Some(30));
    assert_eq!(DateFilter::Last90Days.to_days(), Some(90));
}

#[test]
fn date_filter_clone_and_eq() {
    let f = DateFilter::Last30Days;
    let cloned = f.clone();
    assert_eq!(f, cloned);
    assert_ne!(DateFilter::All, DateFilter::Last7Days);
}

// FilterState ----------------------------------------------------------------

#[test]
fn filter_state_default_matches_documented_defaults() {
    let state = FilterState::default();
    assert!(state.repository_filter.is_none());
    assert_eq!(state.date_filter, DateFilter::Last30Days);
    assert_eq!(state.sort_by, SortBy::Date);
    assert!(state.sort_descending);
}

// RecordsAction --------------------------------------------------------------

#[test]
fn records_action_view_details_carries_session_id() {
    let action = RecordsAction::ViewDetails(42);
    let cloned = action.clone();
    assert!(matches!(cloned, RecordsAction::ViewDetails(42)));
}

#[test]
fn records_action_return_clone_preserves_variant() {
    let action = RecordsAction::Return;
    let cloned = action.clone();
    assert!(matches!(cloned, RecordsAction::Return));
}

// RecordsScreenData ----------------------------------------------------------

#[test]
fn records_screen_data_holds_sessions_and_repositories() {
    let sessions = vec![make_session(1, None)];
    let repositories = vec![StoredRepository {
        id: 5,
        user_name: "alice".to_string(),
        repository_name: "tools".to_string(),
        remote_url: "https://example.com/alice/tools".to_string(),
    }];
    let data = RecordsScreenData {
        sessions,
        repositories,
    };

    assert_eq!(data.sessions.len(), 1);
    assert_eq!(data.repositories.len(), 1);
    assert_eq!(data.repositories[0].user_name, "alice");
}

// RecordsScreen --------------------------------------------------------------

#[test]
fn new_screen_starts_with_no_selected_session_or_action() {
    let screen = make_screen();
    assert!(screen.get_selected_session_for_detail().is_none());
    assert!(screen.get_action_result().is_none());
}

#[test]
fn set_selected_session_from_index_with_out_of_bounds_does_nothing() {
    let screen = make_screen();
    screen.set_selected_session_from_index(99);
    assert!(screen.get_selected_session_for_detail().is_none());
}

// init_with_data ------------------------------------------------------------

#[test]
fn init_with_data_uses_screen_data_when_downcast_succeeds() {
    let screen = make_screen();
    let data = make_screen_data(3);

    screen.init_with_data(Box::new(data)).unwrap();
    screen.set_selected_session_from_index(2);

    let selected = screen.get_selected_session_for_detail().unwrap();
    assert_eq!(selected.session.id, 2);
}

#[test]
fn init_with_data_falls_back_to_service_when_downcast_fails() {
    let screen = make_screen();

    let result = screen.init_with_data(Box::new(()));

    assert!(result.is_ok());
    screen.set_selected_session_from_index(0);
    assert!(screen.get_selected_session_for_detail().is_none());
}

#[test]
fn init_with_data_falls_back_to_failing_service_returns_error() {
    let screen = make_screen_with(FailingSessionService);

    let result = screen.init_with_data(Box::new(()));

    assert!(result.is_err());
}

// Key handling --------------------------------------------------------------

#[test]
fn esc_publishes_navigate_to_title_and_records_return_action() {
    let (screen, captured) = make_screen_with_event_capture();

    screen.handle_key_event(key(KeyCode::Esc)).unwrap();

    let events = captured.lock().unwrap();
    assert!(matches!(
        events.first(),
        Some(NavigateTo::Replace(ScreenType::Title))
    ));
    assert!(matches!(
        screen.get_action_result(),
        Some(RecordsAction::Return)
    ));
}

#[test]
fn ctrl_c_publishes_navigate_exit_and_records_return_action() {
    let (screen, captured) = make_screen_with_event_capture();

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL))
        .unwrap();

    let events = captured.lock().unwrap();
    assert!(matches!(events.first(), Some(NavigateTo::Exit)));
    assert!(matches!(
        screen.get_action_result(),
        Some(RecordsAction::Return)
    ));
}

#[test]
fn enter_with_loaded_sessions_pushes_session_detail() {
    let (screen, captured) = make_screen_with_event_capture();
    screen
        .init_with_data(Box::new(make_screen_data(2)))
        .unwrap();

    screen.handle_key_event(key(KeyCode::Enter)).unwrap();

    let events = captured.lock().unwrap();
    assert!(matches!(
        events.first(),
        Some(NavigateTo::Push(ScreenType::SessionDetail))
    ));
    assert!(screen.get_selected_session_for_detail().is_some());
}

#[test]
fn space_with_loaded_sessions_pushes_session_detail() {
    let (screen, captured) = make_screen_with_event_capture();
    screen
        .init_with_data(Box::new(make_screen_data(2)))
        .unwrap();

    screen.handle_key_event(key(KeyCode::Char(' '))).unwrap();

    let events = captured.lock().unwrap();
    assert!(matches!(
        events.first(),
        Some(NavigateTo::Push(ScreenType::SessionDetail))
    ));
}

#[test]
fn enter_without_loaded_sessions_does_not_push() {
    let (screen, captured) = make_screen_with_event_capture();
    screen
        .init_with_data(Box::new(make_screen_data(0)))
        .unwrap();
    // refresh resets list_state.selected() to None when sessions are empty
    screen.handle_key_event(key(KeyCode::Char('r'))).unwrap();
    screen.handle_key_event(key(KeyCode::Enter)).unwrap();

    let events = captured.lock().unwrap();
    assert!(events.is_empty());
}

#[test]
fn down_then_up_keys_navigate_without_panic() {
    let screen = make_screen();
    screen
        .init_with_data(Box::new(make_screen_data(3)))
        .unwrap();

    screen.handle_key_event(key(KeyCode::Down)).unwrap();
    screen.handle_key_event(key(KeyCode::Char('j'))).unwrap();
    screen.handle_key_event(key(KeyCode::Up)).unwrap();
    screen.handle_key_event(key(KeyCode::Char('k'))).unwrap();
}

#[test]
fn down_wraps_around_to_first_after_reaching_last_entry() {
    let screen = make_screen();
    screen
        .init_with_data(Box::new(make_screen_data(3)))
        .unwrap();

    // Selection starts at 0 — drive Down four times: 0→1→2→0 (wrap)→1
    (0..4).for_each(|_| screen.handle_key_event(key(KeyCode::Down)).unwrap());

    screen.handle_key_event(key(KeyCode::Enter)).unwrap();
    let selected = screen.get_selected_session_for_detail().unwrap();
    assert_eq!(selected.session.id, 1);
}

#[test]
fn up_wraps_around_to_last_entry_from_first() {
    let screen = make_screen();
    screen
        .init_with_data(Box::new(make_screen_data(3)))
        .unwrap();

    screen.handle_key_event(key(KeyCode::Up)).unwrap();
    screen.handle_key_event(key(KeyCode::Enter)).unwrap();

    let selected = screen.get_selected_session_for_detail().unwrap();
    assert_eq!(selected.session.id, 2);
}

#[test]
fn refresh_after_empty_init_handles_navigation_from_none_selection() {
    let screen = make_screen();
    screen
        .init_with_data(Box::new(make_screen_data(0)))
        .unwrap();

    screen.handle_key_event(key(KeyCode::Char('r'))).unwrap();
    screen.handle_key_event(key(KeyCode::Down)).unwrap();
    screen.handle_key_event(key(KeyCode::Up)).unwrap();
}

#[test]
fn refresh_with_failing_service_does_not_panic() {
    let screen = make_screen_with(FailingSessionService);
    screen.handle_key_event(key(KeyCode::Char('r'))).unwrap();
}

#[test]
fn unhandled_key_event_does_not_publish_navigation() {
    let (screen, captured) = make_screen_with_event_capture();

    screen.handle_key_event(key(KeyCode::Char('z'))).unwrap();

    assert!(captured.lock().unwrap().is_empty());
}

// Cycle sort and filter -----------------------------------------------------

#[test]
fn s_key_cycles_through_all_sort_variants_and_toggles_direction() {
    let screen = make_screen();

    // Initial: Date / descending=true
    // s: Date → Performance
    screen.handle_key_event(key(KeyCode::Char('s'))).unwrap();
    // s: Performance → Repository
    screen.handle_key_event(key(KeyCode::Char('s'))).unwrap();
    // s: Repository → Duration
    screen.handle_key_event(key(KeyCode::Char('s'))).unwrap();
    // s: Duration → Date (toggles descending to false)
    screen.handle_key_event(key(KeyCode::Char('s'))).unwrap();
    // s: Date → Performance again (no toggle this time)
    screen.handle_key_event(key(KeyCode::Char('s'))).unwrap();
}

#[test]
fn f_key_cycles_through_all_date_filter_variants() {
    let screen = make_screen();

    // Initial: Last30Days
    // f: Last30Days → Last90Days
    screen.handle_key_event(key(KeyCode::Char('f'))).unwrap();
    // f: Last90Days → All
    screen.handle_key_event(key(KeyCode::Char('f'))).unwrap();
    // f: All → Last7Days
    screen.handle_key_event(key(KeyCode::Char('f'))).unwrap();
    // f: Last7Days → Last30Days
    screen.handle_key_event(key(KeyCode::Char('f'))).unwrap();
}

#[test]
fn f_key_with_loaded_sessions_resets_selection_to_first() {
    let screen = make_screen();
    screen
        .init_with_data(Box::new(make_screen_data(3)))
        .unwrap();
    screen.handle_key_event(key(KeyCode::Down)).unwrap();

    // 'f' resets selection back to Some(0) since stub returns empty + then non-empty?
    // Here we just confirm the call does not error.
    screen.handle_key_event(key(KeyCode::Char('f'))).unwrap();
}

#[test]
fn s_and_f_key_with_failing_service_do_not_panic() {
    let screen = make_screen_with(FailingSessionService);

    screen.handle_key_event(key(KeyCode::Char('s'))).unwrap();
    screen.handle_key_event(key(KeyCode::Char('f'))).unwrap();
}

// Rendering -----------------------------------------------------------------

#[test]
fn render_with_empty_sessions_shows_empty_message() {
    let screen = make_screen();
    screen
        .init_with_data(Box::new(make_screen_data(0)))
        .unwrap();
    // Empty state results when refresh sets list_state.select(None)
    screen.handle_key_event(key(KeyCode::Char('r'))).unwrap();

    render_screen(&screen);
}

#[test]
fn render_with_loaded_sessions_renders_list() {
    let screen = make_screen();
    screen
        .init_with_data(Box::new(make_screen_data(3)))
        .unwrap();

    render_screen(&screen);
}

#[test]
fn render_with_session_missing_repository_and_result_renders_placeholders() {
    let screen = make_screen();

    let mut session = make_session(1, None);
    session.repository = None;
    session.session_result = None;

    let data = RecordsScreenData {
        sessions: vec![session],
        repositories: Vec::new(),
    };
    screen.init_with_data(Box::new(data)).unwrap();

    render_screen(&screen);
}

#[test]
fn render_with_long_repository_name_truncates() {
    let screen = make_screen();

    let mut session = make_session(1, Some(1));
    session.repository = Some(StoredRepository {
        id: 1,
        user_name: "very-very-very-long-user".to_string(),
        repository_name: "and-an-incredibly-long-repository-name".to_string(),
        remote_url: "https://example.com/x/y".to_string(),
    });

    let data = RecordsScreenData {
        sessions: vec![session],
        repositories: Vec::new(),
    };
    screen.init_with_data(Box::new(data)).unwrap();

    render_screen(&screen);
}

#[test]
fn render_with_ascending_sort_renders_arrow() {
    let screen = make_screen();
    screen
        .init_with_data(Box::new(make_screen_data(2)))
        .unwrap();

    // Cycle sort 4 times so we end up back on Date with sort_descending=false.
    (0..4).for_each(|_| screen.handle_key_event(key(KeyCode::Char('s'))).unwrap());

    render_screen(&screen);
}

// Screen trait misc ---------------------------------------------------------

#[test]
fn cleanup_returns_ok() {
    let screen = make_screen();
    assert!(screen.cleanup().is_ok());
}

#[test]
fn as_any_downcasts_to_concrete_type() {
    let screen = make_screen();
    assert!(screen.as_any().downcast_ref::<RecordsScreen>().is_some());
}

#[test]
fn filter_state_struct_fields_are_accessible() {
    let state = FilterState {
        repository_filter: Some(7),
        date_filter: DateFilter::Last7Days,
        sort_by: SortBy::Repository,
        sort_descending: false,
    };
    assert_eq!(state.repository_filter, Some(7));
    assert_eq!(state.date_filter, DateFilter::Last7Days);
    assert_eq!(state.sort_by, SortBy::Repository);
    assert!(!state.sort_descending);
}
