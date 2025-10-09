use gittype::presentation::game::models::ScreenDataProvider;
use gittype::Result;

/// Empty mock provider for screens that don't need data
pub struct EmptyMockProvider;

impl ScreenDataProvider for EmptyMockProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        Ok(Box::new(()))
    }
}

/// Helper macro to create snapshot tests for screens using ratatui TestBackend
#[macro_export]
macro_rules! screen_snapshot_test {
    // Version with custom provider
    ($test_name:ident, $screen_type:ty, $screen_init:expr, provider = $provider:expr) => {
        #[test]
        fn $test_name() {
            use gittype::presentation::game::Screen;
            use gittype::presentation::game::models::ScreenDataProvider;
            use ratatui::backend::TestBackend;
            use ratatui::Terminal;

            // Set timezone to UTC for consistent snapshots across environments
            std::env::set_var("TZ", "UTC");

            let mut screen: $screen_type = $screen_init;

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
            use gittype::presentation::game::Screen;
            use gittype::presentation::game::models::ScreenDataProvider;
            use ratatui::backend::TestBackend;
            use ratatui::Terminal;

            // Set timezone to UTC for consistent snapshots across environments
            std::env::set_var("TZ", "UTC");

            let mut screen: $screen_type = $screen_init;

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
            use gittype::presentation::game::Screen;
            use ratatui::backend::TestBackend;
            use ratatui::Terminal;

            std::env::set_var("TZ", "UTC");

            let source = $source_screen;
            let mut screen: $screen_type = $screen_init;

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
