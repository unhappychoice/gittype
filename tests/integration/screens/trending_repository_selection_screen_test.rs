use crate::integration::screens::mocks::trending_repository_mock::MockTrendingRepository;
use crate::integration::screens::mocks::trending_repository_selection_screen_mock::MockTrendingRepositorySelectionDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::TrendingRepositorySelectionScreen;
use std::sync::Arc;

screen_snapshot_test!(
    test_trending_repository_selection_screen_snapshot,
    TrendingRepositorySelectionScreen,
    TrendingRepositorySelectionScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(MockTrendingRepository::new())
    ),
    provider = MockTrendingRepositorySelectionDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_trending_repository_selection_screen_esc_exits,
    TrendingRepositorySelectionScreen,
    |event_bus| TrendingRepositorySelectionScreen::new(
        event_bus,
        Arc::new(MockTrendingRepository::new())
    ),
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockTrendingRepositorySelectionDataProvider
);

screen_key_event_test!(
    test_trending_repository_selection_screen_ctrl_c_exits,
    TrendingRepositorySelectionScreen,
    |event_bus| TrendingRepositorySelectionScreen::new(
        event_bus,
        Arc::new(MockTrendingRepository::new())
    ),
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockTrendingRepositorySelectionDataProvider
);

screen_key_event_test!(
    test_trending_repository_selection_screen_space_selects,
    TrendingRepositorySelectionScreen,
    |event_bus| TrendingRepositorySelectionScreen::new(
        event_bus,
        Arc::new(MockTrendingRepository::new())
    ),
    NavigateTo,
    KeyCode::Char(' '),
    KeyModifiers::empty(),
    MockTrendingRepositorySelectionDataProvider
);

// Non-event key tests
screen_key_tests_custom!(
    TrendingRepositorySelectionScreen,
    |event_bus| TrendingRepositorySelectionScreen::new(
        event_bus,
        Arc::new(MockTrendingRepository::new())
    ),
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

// Basic methods test
screen_basic_methods_test!(
    test_trending_repository_selection_screen_basic_methods,
    TrendingRepositorySelectionScreen,
    TrendingRepositorySelectionScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(MockTrendingRepository::new())
    ),
    gittype::presentation::tui::ScreenType::TrendingRepositorySelection,
    true,
    MockTrendingRepositorySelectionDataProvider
);
