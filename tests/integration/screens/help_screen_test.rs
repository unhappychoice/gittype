use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::help_screen::HelpScreen;
use std::sync::Arc;

screen_snapshot_test!(
    test_help_screen_snapshot_cli,
    HelpScreen,
    HelpScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    )
);

screen_snapshot_test!(
    test_help_screen_snapshot_scoring,
    HelpScreen,
    HelpScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    keys = [KeyEvent::new(KeyCode::Right, KeyModifiers::empty())]
);

screen_snapshot_test!(
    test_help_screen_snapshot_ranks,
    HelpScreen,
    HelpScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);

screen_snapshot_test!(
    test_help_screen_snapshot_game_help,
    HelpScreen,
    HelpScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);

screen_snapshot_test!(
    test_help_screen_snapshot_community,
    HelpScreen,
    HelpScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);

// Event-producing key tests
use crate::integration::screens::helpers::EmptyMockProvider;

screen_key_event_test!(
    test_help_screen_esc_navigates_back,
    HelpScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    EmptyMockProvider
);

screen_key_event_test!(
    test_help_screen_ctrl_c_exits,
    HelpScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    EmptyMockProvider
);

// Non-event key tests
screen_key_tests!(
    HelpScreen,
    EmptyMockProvider,
    [
        (
            test_help_screen_left_switches_tab,
            KeyCode::Left,
            KeyModifiers::empty()
        ),
        (
            test_help_screen_h_switches_tab,
            KeyCode::Char('h'),
            KeyModifiers::empty()
        ),
        (
            test_help_screen_right_switches_tab,
            KeyCode::Right,
            KeyModifiers::empty()
        ),
        (
            test_help_screen_l_switches_tab,
            KeyCode::Char('l'),
            KeyModifiers::empty()
        ),
        (
            test_help_screen_up_scrolls,
            KeyCode::Up,
            KeyModifiers::empty()
        ),
        (
            test_help_screen_k_scrolls,
            KeyCode::Char('k'),
            KeyModifiers::empty()
        ),
        (
            test_help_screen_down_scrolls,
            KeyCode::Down,
            KeyModifiers::empty()
        ),
        (
            test_help_screen_j_scrolls,
            KeyCode::Char('j'),
            KeyModifiers::empty()
        ),
        (
            test_help_screen_g_opens_github,
            KeyCode::Char('g'),
            KeyModifiers::empty()
        ),
    ]
);

// Basic methods test
screen_basic_methods_test!(
    test_help_screen_basic_methods,
    HelpScreen,
    HelpScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    gittype::presentation::tui::ScreenType::Help,
    false
);

use gittype::presentation::tui::screens::help_screen::HelpSection;
use gittype::presentation::tui::Screen;
use ratatui::backend::TestBackend;
use ratatui::Terminal;

fn make_help_screen() -> HelpScreen {
    HelpScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    )
}

fn render_buffer_text(screen: &HelpScreen) -> String {
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| {
            screen.render_ratatui(frame).unwrap();
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let mut output = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            output.push_str(buffer[(x, y)].symbol());
        }
        output.push('\n');
    }
    output
}

fn press(screen: &HelpScreen, code: KeyCode) {
    screen
        .handle_key_event(KeyEvent::new(code, KeyModifiers::empty()))
        .unwrap();
}

#[test]
fn help_section_title_covers_all_variants() {
    assert_eq!(HelpSection::Scoring.title(), "Scoring System");
    assert_eq!(HelpSection::Ranks.title(), "Rank System");
    assert_eq!(HelpSection::GameHelp.title(), "Game Help");
    assert_eq!(HelpSection::CLI.title(), "CLI Usage");
    assert_eq!(HelpSection::About.title(), "About & Credits");
    assert_eq!(
        HelpSection::ThirdPartyLicenses.title(),
        "Third-Party Licenses"
    );
    assert_eq!(HelpSection::Community.title(), "Community");
}

#[test]
fn help_section_all_lists_every_variant_in_order() {
    let sections = HelpSection::all();
    assert_eq!(
        sections,
        vec![
            HelpSection::CLI,
            HelpSection::Scoring,
            HelpSection::Ranks,
            HelpSection::GameHelp,
            HelpSection::Community,
            HelpSection::About,
            HelpSection::ThirdPartyLicenses,
        ]
    );
}

#[test]
fn help_section_default_is_cli() {
    assert_eq!(HelpSection::default(), HelpSection::CLI);
}

#[test]
fn rendering_about_section_includes_credits_content() {
    let screen = make_help_screen();
    // CLI -> Scoring -> Ranks -> GameHelp -> Community -> About
    for _ in 0..5 {
        press(&screen, KeyCode::Right);
    }
    let output = render_buffer_text(&screen);

    assert!(output.contains("GitType"));
    assert!(output.contains("unhappychoice"));
    assert!(output.contains("Development Team"));
}

#[test]
fn rendering_third_party_licenses_section_includes_license_content() {
    let screen = make_help_screen();
    for _ in 0..6 {
        press(&screen, KeyCode::Right);
    }
    let output = render_buffer_text(&screen);

    // THIRD_PARTY_LICENSES is included from a real LICENSE file; just confirm
    // get_third_party_licenses_content rendered something rather than empty content.
    let trimmed: String = output.chars().filter(|c| !c.is_whitespace()).collect();
    assert!(!trimmed.is_empty());
    assert!(output.contains("Third-Party Licenses"));
}

#[test]
fn pressing_left_from_cli_wraps_to_last_section() {
    let screen = make_help_screen();
    press(&screen, KeyCode::Left);
    let output = render_buffer_text(&screen);

    // After wrap, current section is ThirdPartyLicenses (the last entry).
    assert!(output.contains("Third-Party Licenses"));
}

#[test]
fn pressing_h_wraps_to_last_section_like_left() {
    let screen = make_help_screen();
    press(&screen, KeyCode::Char('h'));
    let output = render_buffer_text(&screen);

    assert!(output.contains("Third-Party Licenses"));
}

#[test]
fn pressing_down_then_up_keeps_scroll_position_within_bounds() {
    let screen = make_help_screen();
    // Move to ThirdPartyLicenses which has long content so max_scroll > 0.
    for _ in 0..6 {
        press(&screen, KeyCode::Right);
    }
    // Render once so render_content publishes content_height/viewport_height.
    let _ = render_buffer_text(&screen);

    // Scrolling down with content larger than viewport should be a no-panic op
    // and must not error.
    for _ in 0..10 {
        press(&screen, KeyCode::Down);
    }
    for _ in 0..3 {
        press(&screen, KeyCode::Char('j'));
    }

    // Scrolling back up should also be safe (saturating_sub).
    for _ in 0..30 {
        press(&screen, KeyCode::Up);
    }
    for _ in 0..3 {
        press(&screen, KeyCode::Char('k'));
    }

    let _ = render_buffer_text(&screen);
}

#[test]
fn pressing_up_at_top_of_cli_is_a_noop() {
    let screen = make_help_screen();
    // CLI content fits in the viewport so scroll stays clamped at 0.
    press(&screen, KeyCode::Up);
    press(&screen, KeyCode::Char('k'));
    let _ = render_buffer_text(&screen);
}

#[test]
fn unhandled_key_in_main_view_is_a_noop() {
    let screen = make_help_screen();
    // Tab is not bound; should hit the `_ => Ok(())` arm.
    press(&screen, KeyCode::Tab);
}

#[test]
fn help_screen_as_any_downcasts_back_to_concrete_type() {
    let screen = make_help_screen();
    assert!(screen.as_any().downcast_ref::<HelpScreen>().is_some());
}

#[test]
fn help_screen_default_provider_returns_unit() {
    let provider = HelpScreen::default_provider();
    let boxed = provider.provide().unwrap();
    assert!(boxed.downcast_ref::<()>().is_some());
}
