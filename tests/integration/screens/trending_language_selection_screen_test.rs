use crate::integration::screens::helpers::EmptyMockProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::TrendingLanguageSelectionScreen;

screen_snapshot_test!(
    test_trending_language_selection_screen_snapshot,
    TrendingLanguageSelectionScreen,
    TrendingLanguageSelectionScreen::new(EventBus::new())
);

// Event-producing key tests
screen_key_event_test!(
    test_trending_language_selection_screen_esc_exits,
    TrendingLanguageSelectionScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    EmptyMockProvider
);

screen_key_event_test!(
    test_trending_language_selection_screen_ctrl_c_exits,
    TrendingLanguageSelectionScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    EmptyMockProvider
);

screen_key_event_test!(
    test_trending_language_selection_screen_space_selects,
    TrendingLanguageSelectionScreen,
    NavigateTo,
    KeyCode::Char(' '),
    KeyModifiers::empty(),
    EmptyMockProvider
);

// Non-event key tests
screen_key_tests!(
    TrendingLanguageSelectionScreen,
    EmptyMockProvider,
    [
        (
            test_trending_language_selection_screen_up_navigates,
            KeyCode::Up,
            KeyModifiers::empty()
        ),
        (
            test_trending_language_selection_screen_k_navigates,
            KeyCode::Char('k'),
            KeyModifiers::empty()
        ),
        (
            test_trending_language_selection_screen_down_navigates,
            KeyCode::Down,
            KeyModifiers::empty()
        ),
        (
            test_trending_language_selection_screen_j_navigates,
            KeyCode::Char('j'),
            KeyModifiers::empty()
        ),
    ]
);
