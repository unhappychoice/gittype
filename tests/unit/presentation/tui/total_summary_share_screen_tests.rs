use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::EventBus;
use gittype::domain::events::EventBusInterface;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::models::TotalResult;
use gittype::domain::services::scoring::{TotalTracker, TotalTrackerInterface};
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::di::AppModule;
use gittype::presentation::sharing::SharingPlatform;
use gittype::presentation::tui::screens::total_summary_share_screen::{
    TotalSummaryShareScreen, TotalSummaryShareScreenProvider,
};
use gittype::presentation::tui::Screen;
use shaku::Provider;
use std::sync::{Arc, Mutex};

fn make_screen() -> TotalSummaryShareScreen {
    TotalSummaryShareScreen::new(
        Arc::new(EventBus::new()) as Arc<dyn EventBusInterface>,
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
        Arc::new(TotalTracker::new_for_test()) as Arc<dyn TotalTrackerInterface>,
    )
}

#[test]
fn generate_share_url_encodes_each_platform() {
    let screen = make_screen();
    let mut total_result = TotalResult::new();
    total_result.total_keystrokes = 1234;
    total_result.total_score = 9876.0;
    total_result.overall_cpm = 321.0;
    screen.set_total_result_for_test(total_result);

    let text = "Total score: 9876 & CPM: 321";

    let x_url = screen.generate_share_url_for_test(text, &SharingPlatform::X);
    assert!(x_url.starts_with("https://x.com/intent/tweet?text="));
    assert!(x_url.contains("Total%20score%3A%209876%20%26%20CPM%3A%20321"));

    let reddit_url = screen.generate_share_url_for_test(text, &SharingPlatform::Reddit);
    assert!(reddit_url.starts_with("https://www.reddit.com/submit?"));
    assert!(reddit_url.contains("title=Just%20demolished%201234%20keystrokes"));
    assert!(reddit_url.contains("selftext=true"));
    assert!(reddit_url.contains("text=Total%20score%3A%209876%20%26%20CPM%3A%20321"));

    let linked_in_url = screen.generate_share_url_for_test(text, &SharingPlatform::LinkedIn);
    assert!(linked_in_url.starts_with("https://www.linkedin.com/feed/"));
    assert!(linked_in_url.contains("shareActive=true"));
    assert!(linked_in_url.contains("mini=true"));

    let facebook_url = screen.generate_share_url_for_test(text, &SharingPlatform::Facebook);
    assert!(facebook_url.starts_with("https://www.facebook.com/sharer/sharer.php?"));
    assert!(facebook_url.contains("u=https%3A%2F%2Fgithub.com%2Funhappychoice%2Fgittype"));
    assert!(facebook_url.contains("quote=Total%20score%3A%209876%20%26%20CPM%3A%20321"));
}

#[test]
fn escape_clears_fallback_url_before_navigating_back() {
    let screen = make_screen();
    screen.set_fallback_url_for_test(Some((
        "https://example.test/share".to_string(),
        SharingPlatform::X,
    )));

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()))
        .unwrap();

    assert!(screen.fallback_url_for_test().is_none());
}

#[test]
fn provider_resolves_screen_from_app_module() {
    let module = AppModule::builder().build();
    let provided = <TotalSummaryShareScreenProvider as Provider<AppModule>>::provide(&module);

    assert!(provided.is_ok());
}

#[test]
fn escape_without_fallback_publishes_pop_navigation() {
    let event_bus = Arc::new(EventBus::new());
    let captured: Arc<Mutex<Vec<NavigateTo>>> = Arc::new(Mutex::new(Vec::new()));
    let observed = Arc::clone(&captured);
    event_bus.subscribe(move |event: &NavigateTo| {
        observed.lock().unwrap().push(event.clone());
    });

    let screen = TotalSummaryShareScreen::new(
        event_bus.clone() as Arc<dyn EventBusInterface>,
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
        Arc::new(TotalTracker::new_for_test()) as Arc<dyn TotalTrackerInterface>,
    );

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()))
        .unwrap();

    let events = captured.lock().unwrap();
    assert!(matches!(events.as_slice(), [NavigateTo::Pop]));
}

#[test]
fn ctrl_c_publishes_exit_navigation() {
    let event_bus = Arc::new(EventBus::new());
    let captured: Arc<Mutex<Vec<NavigateTo>>> = Arc::new(Mutex::new(Vec::new()));
    let observed = Arc::clone(&captured);
    event_bus.subscribe(move |event: &NavigateTo| {
        observed.lock().unwrap().push(event.clone());
    });

    let screen = TotalSummaryShareScreen::new(
        event_bus.clone() as Arc<dyn EventBusInterface>,
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
        Arc::new(TotalTracker::new_for_test()) as Arc<dyn TotalTrackerInterface>,
    );

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL))
        .unwrap();

    let events = captured.lock().unwrap();
    assert!(matches!(events.as_slice(), [NavigateTo::Exit]));
}

#[test]
fn share_platform_keys_publish_pop_when_browser_opens() {
    for key in ['1', '2', '3', '4'] {
        let event_bus = Arc::new(EventBus::new());
        let captured: Arc<Mutex<Vec<NavigateTo>>> = Arc::new(Mutex::new(Vec::new()));
        let observed = Arc::clone(&captured);
        event_bus.subscribe(move |event: &NavigateTo| {
            observed.lock().unwrap().push(event.clone());
        });

        let screen = TotalSummaryShareScreen::new(
            event_bus.clone() as Arc<dyn EventBusInterface>,
            Arc::new(ThemeService::new_for_test(
                Theme::default(),
                ColorMode::Dark,
            )) as Arc<dyn ThemeServiceInterface>,
            Arc::new(TotalTracker::new_for_test()) as Arc<dyn TotalTrackerInterface>,
        );

        screen
            .handle_key_event(KeyEvent::new(KeyCode::Char(key), KeyModifiers::empty()))
            .unwrap();

        let events = captured.lock().unwrap();
        assert!(
            matches!(events.as_slice(), [NavigateTo::Pop]),
            "key {key} should publish Pop, got {:?}",
            *events
        );
    }
}
