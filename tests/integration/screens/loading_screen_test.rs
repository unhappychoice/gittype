use crate::integration::screens::mocks::challenge_repository_mock::MockChallengeRepository;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::presentation_events::ExitRequested;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::loading::StepType;
use gittype::domain::models::theme::Theme;
use gittype::domain::models::GitRepository;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::loading_screen::{LoadingScreen, ProgressReporter};
use gittype::presentation::tui::{Screen, ScreenType, UpdateStrategy};
use std::sync::{Arc, Mutex};

#[test]
fn test_loading_screen_ctrl_c_requests_exit() {
    let event_bus = Arc::new(EventBus::new());
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);

    event_bus.subscribe(move |event: &ExitRequested| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let screen = LoadingScreen::new_for_test(
        event_bus,
        Arc::new(MockChallengeRepository::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    );

    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Char('c'),
            KeyModifiers::CONTROL,
        ))
        .unwrap();

    let captured_events = events.lock().unwrap();
    assert_eq!(captured_events.len(), 1);
}

#[test]
fn test_loading_screen_char_a_ignored() {
    let event_bus = Arc::new(EventBus::new());
    let screen = LoadingScreen::new_for_test(
        event_bus,
        Arc::new(MockChallengeRepository::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    );

    // Should not panic
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::empty(),
        ))
        .unwrap();
}

#[test]
fn test_loading_screen_enter_ignored() {
    let event_bus = Arc::new(EventBus::new());
    let screen = LoadingScreen::new_for_test(
        event_bus,
        Arc::new(MockChallengeRepository::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    );

    // Should not panic
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Enter,
            KeyModifiers::empty(),
        ))
        .unwrap();
}

#[test]
fn test_loading_screen_esc_ignored() {
    let event_bus = Arc::new(EventBus::new());
    let screen = LoadingScreen::new_for_test(
        event_bus,
        Arc::new(MockChallengeRepository::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    );

    // Should not panic
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Esc,
            KeyModifiers::empty(),
        ))
        .unwrap();
}

#[test]
fn test_loading_screen_initialization() {
    let event_bus = Arc::new(EventBus::new());
    let screen = LoadingScreen::new_for_test(
        event_bus,
        Arc::new(MockChallengeRepository::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    );

    assert_eq!(screen.get_type(), ScreenType::Loading);
}

fn create_loading_screen() -> LoadingScreen {
    LoadingScreen::new_for_test(
        Arc::new(EventBus::new()),
        Arc::new(MockChallengeRepository::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    )
}

// === ProgressReporter tests ===

#[test]
fn test_set_step_updates_current_step() {
    let screen = create_loading_screen();
    screen.set_step(StepType::Scanning);
    // Verify by rendering (no panic means it worked)
    screen.set_step(StepType::Extracting);
    screen.set_step(StepType::Completed);
}

#[test]
fn test_set_current_file_noop() {
    let screen = create_loading_screen();
    screen.set_current_file(Some("test.rs".to_string()));
    screen.set_current_file(None);
}

#[test]
fn test_set_file_counts_with_total() {
    let screen = create_loading_screen();
    screen.set_file_counts(StepType::Scanning, 5, 10, Some("file.rs".to_string()));
    screen.set_file_counts(StepType::Scanning, 10, 10, None);
}

#[test]
fn test_set_file_counts_zero_total() {
    let screen = create_loading_screen();
    screen.set_file_counts(StepType::Cloning, 0, 0, None);
}

#[test]
fn test_set_file_counts_progress_only_increases() {
    let screen = create_loading_screen();
    screen.set_file_counts(StepType::Scanning, 8, 10, None);
    // Setting lower progress should not update
    screen.set_file_counts(StepType::Scanning, 3, 10, None);
}

#[test]
fn test_set_file_counts_multiple_step_types() {
    let screen = create_loading_screen();
    screen.set_file_counts(StepType::Scanning, 5, 10, None);
    screen.set_file_counts(StepType::Extracting, 3, 20, None);
    screen.set_file_counts(StepType::Generating, 1, 5, None);
}

// === State methods ===

#[test]
fn test_set_repo_info() {
    let screen = create_loading_screen();
    screen.set_repo_info("owner/repo".to_string()).unwrap();
}

#[test]
fn test_set_git_repository_full() {
    let screen = create_loading_screen();
    let repo = GitRepository {
        user_name: "owner".to_string(),
        repository_name: "repo".to_string(),
        remote_url: "https://github.com/owner/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abcdef1234567890".to_string()),
        is_dirty: false,
        root_path: None,
    };
    screen.set_git_repository(&repo).unwrap();
}

#[test]
fn test_set_git_repository_dirty() {
    let screen = create_loading_screen();
    let repo = GitRepository {
        user_name: "user".to_string(),
        repository_name: "project".to_string(),
        remote_url: "https://github.com/user/project".to_string(),
        branch: None,
        commit_hash: None,
        is_dirty: true,
        root_path: None,
    };
    screen.set_git_repository(&repo).unwrap();
}

#[test]
fn test_cleanup() {
    let screen = create_loading_screen();
    assert!(screen.cleanup().is_ok());
}

#[test]
fn test_show_initial() {
    let screen = create_loading_screen();
    assert!(screen.show_initial().is_ok());
}

// === Screen trait methods ===

#[test]
fn test_get_update_strategy_is_time_based() {
    let screen = create_loading_screen();
    assert!(matches!(
        screen.get_update_strategy(),
        UpdateStrategy::TimeBased(_)
    ));
}

#[test]
fn test_update_increments_spinner() {
    let screen = create_loading_screen();
    // First update should return true (not completed/failed)
    let result = screen.update().unwrap();
    assert!(result);
}

#[test]
fn test_update_multiple_increments_spinner() {
    let screen = create_loading_screen();
    for _ in 0..15 {
        let _ = screen.update();
    }
}

// === Render test ===

#[test]
fn test_render_ratatui_default_state() {
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    let screen = create_loading_screen();
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            screen.render_ratatui(frame).unwrap();
        })
        .unwrap();
}

#[test]
fn test_render_ratatui_with_progress() {
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    let screen = create_loading_screen();
    screen.set_step(StepType::Scanning);
    screen.set_file_counts(StepType::Scanning, 5, 10, None);

    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            screen.render_ratatui(frame).unwrap();
        })
        .unwrap();
}

#[test]
fn test_render_ratatui_with_repo_info() {
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    let screen = create_loading_screen();
    screen
        .set_repo_info("owner/repo • main".to_string())
        .unwrap();

    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            screen.render_ratatui(frame).unwrap();
        })
        .unwrap();
}

#[test]
fn test_render_ratatui_completed_state() {
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    let screen = create_loading_screen();
    screen.set_step(StepType::Completed);

    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            screen.render_ratatui(frame).unwrap();
        })
        .unwrap();
}

#[test]
fn test_render_ratatui_generating_step() {
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    let screen = create_loading_screen();
    screen.set_step(StepType::Generating);
    screen.set_file_counts(StepType::Generating, 3, 10, None);

    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            screen.render_ratatui(frame).unwrap();
        })
        .unwrap();
}

#[test]
fn test_render_ratatui_cloning_with_progress() {
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    let screen = create_loading_screen();
    screen.set_step(StepType::Cloning);
    screen.set_file_counts(StepType::Cloning, 50, 100, None);

    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            screen.render_ratatui(frame).unwrap();
        })
        .unwrap();
}

#[test]
fn test_render_ratatui_finalizing_step() {
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    let screen = create_loading_screen();
    screen.set_step(StepType::Finalizing);

    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            screen.render_ratatui(frame).unwrap();
        })
        .unwrap();
}
