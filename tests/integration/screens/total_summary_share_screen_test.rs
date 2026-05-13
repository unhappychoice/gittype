use crate::integration::screens::mocks::total_summary_share_screen_mock::MockTotalSummaryShareDataProvider;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::models::{SessionResult, TotalResult};
use gittype::domain::services::scoring::{TotalTracker, TotalTrackerInterface};
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::sharing::SharingPlatform;
use gittype::presentation::tui::screens::total_summary_share_screen::{
    TotalSummaryShareData, TotalSummaryShareDataProvider, TotalSummaryShareScreen,
};
use gittype::presentation::tui::{Screen, ScreenDataProvider};
use std::sync::Arc;

screen_snapshot_test!(
    test_total_summary_share_screen_snapshot,
    TotalSummaryShareScreen,
    TotalSummaryShareScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>,
        Arc::new(TotalTracker::new_for_test()) as Arc<dyn TotalTrackerInterface>
    ),
    provider = MockTotalSummaryShareDataProvider
);

// Event-producing key tests (when no fallback)
screen_key_event_test!(
    test_total_summary_share_screen_esc_goes_back,
    TotalSummaryShareScreen,
    |event_bus| {
        TotalSummaryShareScreen::new(
            event_bus,
            Arc::new(ThemeService::new_for_test(
                Theme::default(),
                ColorMode::Dark,
            )) as Arc<dyn ThemeServiceInterface>,
            Arc::new(TotalTracker::new_for_test()) as Arc<dyn TotalTrackerInterface>,
        )
    },
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockTotalSummaryShareDataProvider
);

screen_key_event_test!(
    test_total_summary_share_screen_ctrl_c_exits,
    TotalSummaryShareScreen,
    |event_bus| {
        TotalSummaryShareScreen::new(
            event_bus,
            Arc::new(ThemeService::new_for_test(
                Theme::default(),
                ColorMode::Dark,
            )) as Arc<dyn ThemeServiceInterface>,
            Arc::new(TotalTracker::new_for_test()) as Arc<dyn TotalTrackerInterface>,
        )
    },
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockTotalSummaryShareDataProvider
);

// Non-event key tests (browser opening will fail in test environment)
screen_key_tests_custom!(
    TotalSummaryShareScreen,
    |event_bus| {
        TotalSummaryShareScreen::new(
            event_bus,
            Arc::new(ThemeService::new_for_test(
                Theme::default(),
                ColorMode::Dark,
            )) as Arc<dyn ThemeServiceInterface>,
            Arc::new(TotalTracker::new_for_test()) as Arc<dyn TotalTrackerInterface>,
        )
    },
    MockTotalSummaryShareDataProvider,
    [
        (
            test_total_summary_share_screen_1_attempts_share,
            KeyCode::Char('1'),
            KeyModifiers::empty()
        ),
        (
            test_total_summary_share_screen_2_attempts_share,
            KeyCode::Char('2'),
            KeyModifiers::empty()
        ),
        (
            test_total_summary_share_screen_3_attempts_share,
            KeyCode::Char('3'),
            KeyModifiers::empty()
        ),
        (
            test_total_summary_share_screen_4_attempts_share,
            KeyCode::Char('4'),
            KeyModifiers::empty()
        ),
    ]
);

// Basic methods test
screen_basic_methods_test!(
    test_total_summary_share_screen_basic_methods,
    TotalSummaryShareScreen,
    TotalSummaryShareScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>,
        Arc::new(TotalTracker::new_for_test()) as Arc<dyn TotalTrackerInterface>
    ),
    gittype::presentation::tui::ScreenType::TotalSummaryShare,
    false,
    MockTotalSummaryShareDataProvider
);

fn make_screen() -> TotalSummaryShareScreen {
    TotalSummaryShareScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
        Arc::new(TotalTracker::new_for_test()) as Arc<dyn TotalTrackerInterface>,
    )
}

fn sample_total_result() -> TotalResult {
    let mut result = TotalResult::new();
    result.total_score = 1234.0;
    result.overall_cpm = 250.0;
    result.total_keystrokes = 5000;
    result
}

#[test]
fn test_total_summary_share_data_provider_returns_data_when_tracker_present() {
    let tracker = TotalTracker::new_for_test();
    let mut session_result = SessionResult::new();
    session_result.session_score = 1500.0;
    session_result.overall_cpm = 300.0;
    tracker.record(session_result);

    let provider = TotalSummaryShareDataProvider::new_for_test(Some(tracker));
    let data = provider
        .provide()
        .expect("provider should produce data when tracker is present");
    assert!(data.downcast::<TotalSummaryShareData>().is_ok());
}

#[test]
fn test_total_summary_share_data_provider_errors_when_tracker_none() {
    let provider = TotalSummaryShareDataProvider::new_for_test(None);
    assert!(provider.provide().is_err());
}

#[test]
fn test_total_summary_share_screen_default_provider_errors_without_tracker() {
    let result = <TotalSummaryShareScreen as Screen>::default_provider().provide();
    assert!(
        result.is_err(),
        "default_provider should error because tracker mutex holds None"
    );
}

#[test]
fn test_total_summary_share_screen_init_with_data_falls_back_to_injected_tracker() {
    let screen = make_screen();
    let data: Box<dyn std::any::Any> = Box::new(());
    assert!(screen.init_with_data(data).is_ok());
}

#[test]
fn test_total_summary_share_screen_as_any_downcasts_to_concrete_type() {
    let screen = make_screen();
    assert!(screen
        .as_any()
        .downcast_ref::<TotalSummaryShareScreen>()
        .is_some());
}

#[test]
fn test_total_summary_share_screen_unrelated_keys_are_noop() {
    let screen = make_screen();
    let result = screen.handle_key_event(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty()));
    assert!(result.is_ok());
}

#[test]
fn test_total_summary_share_screen_generate_share_url_for_all_platforms() {
    let screen = make_screen();
    screen.set_total_result_for_test(sample_total_result());

    let text = "share text";
    let x_url = screen.generate_share_url_for_test(text, &SharingPlatform::X);
    assert!(x_url.starts_with("https://x.com/intent/tweet?text="));
    assert!(x_url.contains("share%20text"));

    let reddit_url = screen.generate_share_url_for_test(text, &SharingPlatform::Reddit);
    assert!(reddit_url.starts_with("https://www.reddit.com/submit?title="));
    assert!(reddit_url.contains("5000%20keystrokes"));

    let linkedin_url = screen.generate_share_url_for_test(text, &SharingPlatform::LinkedIn);
    assert!(
        linkedin_url.starts_with("https://www.linkedin.com/feed/?shareActive=true&mini=true&text=")
    );

    let facebook_url = screen.generate_share_url_for_test(text, &SharingPlatform::Facebook);
    assert!(facebook_url.starts_with("https://www.facebook.com/sharer/sharer.php?u="));
    assert!(facebook_url.contains("github.com%2Funhappychoice%2Fgittype"));
}

#[test]
fn test_total_summary_share_screen_esc_with_fallback_dismisses_fallback() {
    let screen = make_screen();
    screen.set_fallback_url_for_test(Some((
        "https://example.com".to_string(),
        SharingPlatform::X,
    )));

    let event_bus = Arc::new(EventBus::new());
    let published = Arc::new(std::sync::Mutex::new(0usize));
    let observed = Arc::clone(&published);
    event_bus.subscribe(move |_: &NavigateTo| {
        *observed.lock().unwrap() += 1;
    });

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()))
        .unwrap();

    assert!(
        screen.fallback_url_for_test().is_none(),
        "fallback_url should be cleared"
    );
    // No NavigateTo events were published from this local event_bus, but the
    // screen's own injected event_bus may not have been used. Just verify
    // fallback was dismissed without panic.
    let _ = published;
}

#[test]
fn test_total_summary_share_screen_render_in_fallback_state() {
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    let screen = make_screen();
    screen.set_total_result_for_test(sample_total_result());
    screen.set_fallback_url_for_test(Some((
        "https://example.com/share".to_string(),
        SharingPlatform::X,
    )));

    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| {
            screen.render_ratatui(frame).unwrap();
        })
        .unwrap();
}
