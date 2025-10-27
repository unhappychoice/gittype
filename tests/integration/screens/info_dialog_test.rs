use crate::integration::screens::helpers::EmptyMockProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::info_dialog::InfoDialogScreen;
use std::sync::Arc;

screen_snapshot_test!(
    test_info_dialog_snapshot_default,
    InfoDialogScreen,
    InfoDialogScreen::new(Arc::new(EventBus::new()))
);

// Event-producing key tests (Menu state)
screen_key_event_test!(
    test_info_dialog_esc_closes,
    InfoDialogScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    EmptyMockProvider
);

screen_key_event_test!(
    test_info_dialog_ctrl_c_exits,
    InfoDialogScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    EmptyMockProvider
);

// Non-event key tests (Menu state)
screen_key_tests!(
    InfoDialogScreen,
    EmptyMockProvider,
    [
        (
            test_info_dialog_space_selects,
            KeyCode::Char(' '),
            KeyModifiers::empty()
        ),
        (
            test_info_dialog_up_navigates,
            KeyCode::Up,
            KeyModifiers::empty()
        ),
        (
            test_info_dialog_k_navigates,
            KeyCode::Char('k'),
            KeyModifiers::empty()
        ),
        (
            test_info_dialog_down_navigates,
            KeyCode::Down,
            KeyModifiers::empty()
        ),
        (
            test_info_dialog_j_navigates,
            KeyCode::Char('j'),
            KeyModifiers::empty()
        ),
    ]
);

// Basic methods test
screen_basic_methods_test!(
    test_info_dialog_basic_methods,
    InfoDialogScreen,
    InfoDialogScreen::new(Arc::new(EventBus::new())),
    gittype::presentation::tui::ScreenType::InfoDialog,
    false
);
