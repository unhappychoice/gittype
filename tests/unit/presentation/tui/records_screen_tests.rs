use chrono::Utc;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::storage::{SessionResultData, StoredRepository, StoredSession};
use gittype::domain::models::theme::Theme;
use gittype::domain::services::session_service::{SessionDisplayData, SessionServiceInterface};
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::records_screen::{
    DateFilter, FilterState, RecordsAction, RecordsScreen, RecordsScreenData, SortBy,
};
use gittype::Result;
use std::sync::Arc;

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

fn make_screen() -> RecordsScreen {
    let event_bus = Arc::new(EventBus::new());
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    let session_service = Arc::new(StubSessionService) as Arc<dyn SessionServiceInterface>;
    RecordsScreen::new(event_bus, theme_service, session_service)
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
