use gittype::presentation::tui::ScreenDataProvider;
use gittype::Result;

/// Empty mock provider for screens that don't need data
pub struct EmptyMockProvider;

impl ScreenDataProvider for EmptyMockProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        Ok(Box::new(()))
    }
}

/// Macro to test key event handling with event verification
#[macro_export]
macro_rules! screen_key_event_test {
    // Version without provider (5 params)
    ($test_name:ident, $screen_type:ty, $event_type:ty, $key_code:expr, $modifiers:expr) => {
        screen_key_event_test!(
            $test_name,
            $screen_type,
            $event_type,
            $key_code,
            $modifiers,
            $crate::integration::screens::helpers::EmptyMockProvider
        );
    };

    // Version with default new(event_bus) initialization (6 params)
    ($test_name:ident, $screen_type:ty, $event_type:ty, $key_code:expr, $modifiers:expr, $provider:expr) => {
        #[test]
        fn $test_name() {
            use gittype::domain::events::EventBus;
            use gittype::presentation::tui::Screen;
            use gittype::presentation::tui::ScreenDataProvider;
            use std::sync::{Arc, Mutex};

            let event_bus = Arc::new(EventBus::new());
            let events = Arc::new(Mutex::new(Vec::new()));
            let events_clone = Arc::clone(&events);

            event_bus.subscribe(move |event: &$event_type| {
                events_clone.lock().unwrap().push(event.clone());
            });

            let screen: $screen_type = <$screen_type>::new(event_bus);
            let data = $provider.provide().unwrap();
            let _ = screen.init_with_data(data);

            screen
                .handle_key_event(crossterm::event::KeyEvent::new($key_code, $modifiers))
                .unwrap();

            let captured_events = events.lock().unwrap();
            assert_eq!(captured_events.len(), 1);
        }
    };

    // Version with custom screen initialization using closure (7 params)
    ($test_name:ident, $screen_type:ty, $screen_init_fn:expr, $event_type:ty, $key_code:expr, $modifiers:expr, $provider:expr) => {
        #[test]
        fn $test_name() {
            use gittype::domain::events::EventBus;
            use gittype::presentation::tui::Screen;
            use gittype::presentation::tui::ScreenDataProvider;
            use std::sync::{Arc, Mutex};

            let event_bus = Arc::new(EventBus::new());
            let events = Arc::new(Mutex::new(Vec::new()));
            let events_clone = Arc::clone(&events);

            event_bus.subscribe(move |event: &$event_type| {
                events_clone.lock().unwrap().push(event.clone());
            });

            let screen: $screen_type = ($screen_init_fn)(event_bus);
            let data = $provider.provide().unwrap();
            let _ = screen.init_with_data(data);

            screen
                .handle_key_event(crossterm::event::KeyEvent::new($key_code, $modifiers))
                .unwrap();

            let captured_events = events.lock().unwrap();
            assert_eq!(captured_events.len(), 1);
        }
    };
}

/// Macro to test key event handling without event verification
#[macro_export]
macro_rules! screen_key_test {
    // Version with custom screen init (6 params)
    ($test_name:ident, $screen_type:ty, $screen_init_fn:expr, $key_code:expr, $modifiers:expr, $provider:expr) => {
        #[test]
        fn $test_name() {
            use gittype::domain::events::EventBus;
            use gittype::presentation::tui::Screen;
            use gittype::presentation::tui::ScreenDataProvider;
            use std::sync::Arc;

            let event_bus = Arc::new(EventBus::new());
            let screen: $screen_type = ($screen_init_fn)(event_bus);
            let data = $provider.provide().unwrap();
            let _ = screen.init_with_data(data);

            screen
                .handle_key_event(crossterm::event::KeyEvent::new($key_code, $modifiers))
                .unwrap();
        }
    };

    // Default version (5 params)
    ($test_name:ident, $screen_type:ty, $key_code:expr, $modifiers:expr, $provider:expr) => {
        #[test]
        fn $test_name() {
            use gittype::domain::events::EventBus;
            use gittype::presentation::tui::Screen;
            use gittype::presentation::tui::ScreenDataProvider;
            use std::sync::Arc;

            let event_bus = Arc::new(EventBus::new());
            let screen: $screen_type = <$screen_type>::new(event_bus);
            let data = $provider.provide().unwrap();
            let _ = screen.init_with_data(data);

            screen
                .handle_key_event(crossterm::event::KeyEvent::new($key_code, $modifiers))
                .unwrap();
        }
    };

    // Version without provider
    ($test_name:ident, $screen_type:ty, $key_code:expr, $modifiers:expr) => {
        screen_key_test!(
            $test_name,
            $screen_type,
            $key_code,
            $modifiers,
            $crate::integration::screens::helpers::EmptyMockProvider
        );
    };
}

/// Macro to test multiple key events for the same screen
#[macro_export]
macro_rules! screen_key_tests {
    (
        $screen_type:ty,
        $provider:expr,
        [$(($test_name:ident, $key_code:expr, $modifiers:expr)),* $(,)?]
    ) => {
        $(
            screen_key_test!($test_name, $screen_type, $key_code, $modifiers, $provider);
        )*
    };
}

/// Macro to test multiple key events with custom screen initialization
#[macro_export]
macro_rules! screen_key_tests_custom {
    (
        $screen_type:ty,
        $screen_init_fn:expr,
        $provider:expr,
        [$(($test_name:ident, $key_code:expr, $modifiers:expr)),* $(,)?]
    ) => {
        $(
            screen_key_test!($test_name, $screen_type, $screen_init_fn, $key_code, $modifiers, $provider);
        )*
    };
}

/// Macro to test multiple key events with event verification
#[macro_export]
macro_rules! screen_key_event_tests {
    (
        $screen_type:ty,
        $event_type:ty,
        $provider:expr,
        [$(($test_name:ident, $key_code:expr, $modifiers:expr)),* $(,)?]
    ) => {
        $(
            screen_key_event_test!($test_name, $screen_type, $event_type, $key_code, $modifiers, $provider);
        )*
    };
}

/// Macro to test multiple key events with event verification and custom initialization
#[macro_export]
macro_rules! screen_key_event_tests_custom {
    (
        $screen_type:ty,
        $screen_init_fn:expr,
        $event_type:ty,
        $provider:expr,
        [$(($test_name:ident, $key_code:expr, $modifiers:expr)),* $(,)?]
    ) => {
        $(
            screen_key_event_test!($test_name, $screen_type, $screen_init_fn, $event_type, $key_code, $modifiers, $provider);
        )*
    };
}

/// Helper macro to create snapshot tests for screens using ratatui TestBackend
#[macro_export]
macro_rules! screen_snapshot_test {
    // Version with custom provider
    ($test_name:ident, $screen_type:ty, $screen_init:expr, provider = $provider:expr) => {
        #[test]
        fn $test_name() {
            use gittype::presentation::tui::Screen;
            use gittype::presentation::tui::ScreenDataProvider;
            use ratatui::backend::TestBackend;
            use ratatui::Terminal;

            // Set timezone to UTC for consistent snapshots across environments
            std::env::set_var("TZ", "UTC");

            let screen: $screen_type = $screen_init;

            // Initialize screen with data from the provided mock provider
            let data = $provider.provide().unwrap();
            let _ = screen.init_with_data(data);

            // Create a test backend with a reasonable terminal size
            let backend = TestBackend::new(120, 40);
            let mut terminal = Terminal::new(backend).unwrap();

            terminal
                .draw(|frame| {
                    screen.render_ratatui(frame).unwrap();
                })
                .unwrap();

            // Get the rendered buffer as a string representation
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
    };

    // Version without custom provider (uses EmptyMockProvider)
    ($test_name:ident, $screen_type:ty, $screen_init:expr) => {
        screen_snapshot_test!($test_name, $screen_type, $screen_init, provider = $crate::integration::screens::helpers::EmptyMockProvider);
    };

    // Version with key events
    ($test_name:ident, $screen_type:ty, $screen_init:expr, provider = $provider:expr, keys = [$($key:expr),*]) => {
        #[test]
        fn $test_name() {
            use gittype::presentation::tui::Screen;
            use gittype::presentation::tui::ScreenDataProvider;
            use ratatui::backend::TestBackend;
            use ratatui::Terminal;

            // Set timezone to UTC for consistent snapshots across environments
            std::env::set_var("TZ", "UTC");

            let screen: $screen_type = $screen_init;

            // Initialize screen with data from the provided mock provider
            let data = $provider.provide().unwrap();
            let _ = screen.init_with_data(data);

            // Handle key events
            $(
                let _ = screen.handle_key_event($key);
            )*

            // Create a test backend with a reasonable terminal size
            let backend = TestBackend::new(120, 40);
            let mut terminal = Terminal::new(backend).unwrap();

            terminal
                .draw(|frame| {
                    screen.render_ratatui(frame).unwrap();
                })
                .unwrap();

            // Get the rendered buffer as a string representation
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
    };

    // Version with key events but without provider
    ($test_name:ident, $screen_type:ty, $screen_init:expr, keys = [$($key:expr),*]) => {
        screen_snapshot_test!($test_name, $screen_type, $screen_init, provider = $crate::integration::screens::helpers::EmptyMockProvider, keys = [$($key),*]);
    };

    // Version with on_pushed_from
    ($test_name:ident, $screen_type:ty, $screen_init:expr, pushed_from = $source_screen:expr) => {
        #[test]
        fn $test_name() {
            use gittype::presentation::tui::Screen;
            use ratatui::backend::TestBackend;
            use ratatui::Terminal;

            std::env::set_var("TZ", "UTC");

            let source = $source_screen;
            let screen: $screen_type = $screen_init;

            screen.on_pushed_from(&source).unwrap();

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
    };
}

/// Macro to test basic screen methods (get_update_strategy, update, is_exitable, get_type)
#[macro_export]
macro_rules! screen_basic_methods_test {
    ($test_name:ident, $screen_type:ty, $screen_init:expr, $expected_screen_type:expr, $expected_is_exitable:expr, $provider:expr) => {
        #[test]
        fn $test_name() {
            use gittype::presentation::tui::Screen;
            use gittype::presentation::tui::ScreenDataProvider;

            let screen: $screen_type = $screen_init;

            // Initialize with provider data if provided
            let data = $provider.provide().unwrap();
            let _ = screen.init_with_data(data);

            // Test get_type
            assert_eq!(screen.get_type(), $expected_screen_type);

            // Test is_exitable
            assert_eq!(screen.is_exitable(), $expected_is_exitable);

            // Test get_update_strategy (just ensure it doesn't panic)
            let _ = screen.get_update_strategy();

            // Test update (ensure it returns Ok)
            assert!(screen.update().is_ok());
        }
    };

    // Version without provider (uses EmptyMockProvider)
    ($test_name:ident, $screen_type:ty, $screen_init:expr, $expected_screen_type:expr, $expected_is_exitable:expr) => {
        screen_basic_methods_test!(
            $test_name,
            $screen_type,
            $screen_init,
            $expected_screen_type,
            $expected_is_exitable,
            $crate::integration::screens::helpers::EmptyMockProvider
        );
    };
}
