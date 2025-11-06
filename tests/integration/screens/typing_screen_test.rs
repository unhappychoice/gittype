use crate::integration::screens::mocks::typing_screen_mock::{
    create_typing_screen_with_challenge, MockTypingScreenDataProvider,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::presentation::tui::screens::typing_screen::TypingScreen;
use std::sync::Arc;

// Note: TypingScreen has complex state management (waiting_to_start, countdown, dialog_shown)
// These tests cover different display states

// Snapshot test: waiting state with challenge loaded
screen_snapshot_test!(
    test_typing_screen_snapshot_waiting_with_challenge,
    TypingScreen,
    create_typing_screen_with_challenge(
        Arc::new(EventBus::new()),
        Some("fn main() {\n    println!(\"Hello, world!\");\n}")
    ),
    provider = MockTypingScreenDataProvider
);

// Snapshot test: dialog shown state
screen_snapshot_test!(
    test_typing_screen_snapshot_dialog_shown,
    TypingScreen,
    create_typing_screen_with_challenge(Arc::new(EventBus::new()), Some("fn test() { }")),
    provider = MockTypingScreenDataProvider,
    keys = [KeyEvent::new(KeyCode::Esc, KeyModifiers::empty())]
);

// Snapshot test: countdown state (press SPACE to start countdown)
screen_snapshot_test!(
    test_typing_screen_snapshot_countdown,
    TypingScreen,
    create_typing_screen_with_challenge(
        Arc::new(EventBus::new()),
        Some("fn main() {\n    println!(\"Hello\");\n}")
    ),
    provider = MockTypingScreenDataProvider,
    keys = [KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty())]
);

// Snapshot test: typing in progress (after countdown)
#[test]
fn test_typing_screen_snapshot_typing_progress() {
    use gittype::presentation::tui::Screen;
    use gittype::presentation::tui::ScreenDataProvider;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    std::env::set_var("TZ", "UTC");

    let screen = create_typing_screen_with_challenge(
        Arc::new(EventBus::new()),
        Some("fn main() {\n    println!(\"Hello\");\n}"),
    );

    let data = MockTypingScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    // Press SPACE to start countdown
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()));

    // Skip countdown and start typing
    screen.skip_countdown_for_test();
    screen.set_waiting_to_start(false);

    // Type 'f'
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::empty()));

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
            let cell = &buffer[(x, y)];
            output.push_str(cell.symbol());
        }
        output.push('\n');
    }
    insta::assert_snapshot!(output);
}

// Key event test: Ctrl+C exits
screen_key_event_test!(
    test_typing_screen_ctrl_c_exits,
    TypingScreen,
    |event_bus| create_typing_screen_with_challenge(event_bus, None),
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockTypingScreenDataProvider
);

// Snapshot test: dialog Skip action
screen_snapshot_test!(
    test_typing_screen_snapshot_dialog_skip,
    TypingScreen,
    create_typing_screen_with_challenge(Arc::new(EventBus::new()), Some("fn test() { }")),
    provider = MockTypingScreenDataProvider,
    keys = [
        KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Char('s'), KeyModifiers::empty())
    ]
);

// Snapshot test: wrong character input
#[test]
fn test_typing_screen_snapshot_wrong_input() {
    use gittype::presentation::tui::Screen;
    use gittype::presentation::tui::ScreenDataProvider;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    std::env::set_var("TZ", "UTC");

    let screen = create_typing_screen_with_challenge(
        Arc::new(EventBus::new()),
        Some("fn main() {\n    println!(\"Hello\");\n}"),
    );

    let data = MockTypingScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    // Press SPACE to start countdown
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()));

    // Skip countdown and start typing
    screen.skip_countdown_for_test();
    screen.set_waiting_to_start(false);

    // Type wrong character 'x' instead of 'f'
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty()));

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
            let cell = &buffer[(x, y)];
            output.push_str(cell.symbol());
        }
        output.push('\n');
    }
    insta::assert_snapshot!(output);
}

// Snapshot test: backspace
#[test]
fn test_typing_screen_snapshot_backspace() {
    use gittype::presentation::tui::Screen;
    use gittype::presentation::tui::ScreenDataProvider;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    std::env::set_var("TZ", "UTC");

    let screen = create_typing_screen_with_challenge(
        Arc::new(EventBus::new()),
        Some("fn main() {\n    println!(\"Hello\");\n}"),
    );

    let data = MockTypingScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    // Press SPACE to start countdown
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()));

    // Skip countdown and start typing
    screen.skip_countdown_for_test();
    screen.set_waiting_to_start(false);

    // Type 'f', then backspace
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Backspace, KeyModifiers::empty()));

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
            let cell = &buffer[(x, y)];
            output.push_str(cell.symbol());
        }
        output.push('\n');
    }
    insta::assert_snapshot!(output);
}

// Snapshot test: countdown state with dialog (ESC during countdown)
#[test]
fn test_typing_screen_snapshot_countdown_with_dialog() {
    use gittype::presentation::tui::Screen;
    use gittype::presentation::tui::ScreenDataProvider;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    std::env::set_var("TZ", "UTC");

    let screen = create_typing_screen_with_challenge(
        Arc::new(EventBus::new()),
        Some("fn main() {\n    println!(\"Hello\");\n}"),
    );

    let data = MockTypingScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    // Press SPACE to start countdown
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()));

    // Press ESC to show dialog during countdown (without skipping countdown)
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()));

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
            let cell = &buffer[(x, y)];
            output.push_str(cell.symbol());
        }
        output.push('\n');
    }
    insta::assert_snapshot!(output);
}

// Test: Tab key during typing
#[test]
fn test_typing_screen_tab_key() {
    use gittype::presentation::tui::Screen;
    use gittype::presentation::tui::ScreenDataProvider;

    let screen = create_typing_screen_with_challenge(
        Arc::new(EventBus::new()),
        Some("fn main() {\n\tprintln!(\"Hello\");\n}"),
    );

    let data = MockTypingScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    // Press SPACE to start countdown
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()));

    // Skip countdown and start typing
    screen.skip_countdown_for_test();
    screen.set_waiting_to_start(false);

    // Type 'f', 'n', ' ', 'm', 'a', 'i', 'n', '(', ')', ' ', '{'
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('('), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char(')'), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('{'), KeyModifiers::empty()));

    // Press Enter to move to next line
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()));

    // Press Tab to match the tab character
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Tab, KeyModifiers::empty()));
}

// Test: Enter key during typing
#[test]
fn test_typing_screen_enter_key() {
    use gittype::presentation::tui::Screen;
    use gittype::presentation::tui::ScreenDataProvider;

    let screen = create_typing_screen_with_challenge(
        Arc::new(EventBus::new()),
        Some("fn main() {\n    println!(\"Hello\");\n}"),
    );

    let data = MockTypingScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    // Press SPACE to start countdown
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()));

    // Skip countdown and start typing
    screen.skip_countdown_for_test();
    screen.set_waiting_to_start(false);

    // Type 'f', 'n', ' ', 'm', 'a', 'i', 'n', '(', ')', ' ', '{'
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('('), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char(')'), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('{'), KeyModifiers::empty()));

    // Press Enter to move to next line
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()));
}

// Test: typing completion
#[test]
fn test_typing_screen_completion() {
    use gittype::presentation::tui::Screen;
    use gittype::presentation::tui::ScreenDataProvider;

    let screen = create_typing_screen_with_challenge(
        Arc::new(EventBus::new()),
        Some("hi"), // Very short text to complete easily
    );

    let data = MockTypingScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    // Press SPACE to start countdown
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()));

    // Skip countdown and start typing
    screen.skip_countdown_for_test();
    screen.set_waiting_to_start(false);

    // Type 'h', 'i' to complete
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::empty()));
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::empty()));
}

// Test: ESC during typing to show dialog
#[test]
fn test_typing_screen_esc_during_typing() {
    use gittype::presentation::tui::Screen;
    use gittype::presentation::tui::ScreenDataProvider;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    std::env::set_var("TZ", "UTC");

    let screen = create_typing_screen_with_challenge(
        Arc::new(EventBus::new()),
        Some("fn main() {\n    println!(\"Hello\");\n}"),
    );

    let data = MockTypingScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    // Press SPACE to start countdown
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()));

    // Skip countdown and start typing
    screen.skip_countdown_for_test();
    screen.set_waiting_to_start(false);

    // Type 'f'
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::empty()));

    // Press ESC to show dialog during typing
    let _ = screen.handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()));

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
            let cell = &buffer[(x, y)];
            output.push_str(cell.symbol());
        }
        output.push('\n');
    }
    insta::assert_snapshot!(output);
}

// Basic methods test
screen_basic_methods_test!(
    test_typing_screen_basic_methods,
    TypingScreen,
    create_typing_screen_with_challenge(Arc::new(EventBus::new()), None),
    gittype::presentation::tui::ScreenType::Typing,
    false,
    MockTypingScreenDataProvider
);
