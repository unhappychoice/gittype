use crate::integration::screens::mocks::trending_repository_selection_screen_mock::MockTrendingRepositorySelectionDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::TrendingRepositorySelectionScreen;

screen_snapshot_test!(
    test_trending_repository_selection_screen_snapshot,
    TrendingRepositorySelectionScreen,
    TrendingRepositorySelectionScreen::new(EventBus::new()),
    provider = MockTrendingRepositorySelectionDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_trending_repository_selection_screen_esc_exits,
    TrendingRepositorySelectionScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockTrendingRepositorySelectionDataProvider
);

screen_key_event_test!(
    test_trending_repository_selection_screen_ctrl_c_exits,
    TrendingRepositorySelectionScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockTrendingRepositorySelectionDataProvider
);

screen_key_event_test!(
    test_trending_repository_selection_screen_space_selects,
    TrendingRepositorySelectionScreen,
    NavigateTo,
    KeyCode::Char(' '),
    KeyModifiers::empty(),
    MockTrendingRepositorySelectionDataProvider
);

// Non-event key tests
screen_key_tests!(
    TrendingRepositorySelectionScreen,
    MockTrendingRepositorySelectionDataProvider,
    [
        (
            test_trending_repository_selection_screen_up_navigates,
            KeyCode::Up,
            KeyModifiers::empty()
        ),
        (
            test_trending_repository_selection_screen_k_navigates,
            KeyCode::Char('k'),
            KeyModifiers::empty()
        ),
        (
            test_trending_repository_selection_screen_down_navigates,
            KeyCode::Down,
            KeyModifiers::empty()
        ),
        (
            test_trending_repository_selection_screen_j_navigates,
            KeyCode::Char('j'),
            KeyModifiers::empty()
        ),
    ]
);
