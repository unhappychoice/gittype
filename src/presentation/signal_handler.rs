use crate::infrastructure::logging::{log_error_to_file, log_panic_to_file};
use crate::presentation::game::events::ExitRequested;
use crate::presentation::game::models::Screen;
use crate::presentation::game::screen_manager::ScreenManager;
use crate::presentation::game::screens::PanicScreen;
use crate::GitTypeError;
use crossterm::cursor::{Hide, Show};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};

pub fn setup_signal_handlers(screen_manager: Arc<Mutex<ScreenManager>>) {
    let manager_for_panic = screen_manager.clone();

    std::panic::set_hook(Box::new(move |panic_info| {
        // Restore terminal to normal state
        let _ = disable_raw_mode();
        let _ = execute!(std::io::stderr(), LeaveAlternateScreen, Show);

        // Get panic message
        let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic occurred".to_string()
        };

        // Add location info if available
        let full_message = if let Some(location) = panic_info.location() {
            format!(
                "{}\n\nLocation: {}:{}:{}",
                message,
                location.file(),
                location.line(),
                location.column()
            )
        } else {
            message
        };

        // Log panic information to file
        log_panic_to_file(panic_info);

        // Also log with our error type for consistency
        let error = GitTypeError::PanicError(full_message.clone());
        log_error_to_file(&error);

        // Try to show panic screen - if this fails, fall back to standard panic behavior
        if show_panic_screen(&full_message, &manager_for_panic).is_err() {
            // Clean up terminal using the existing static cleanup
            ScreenManager::cleanup_terminal_static();

            // Fallback to standard panic message
            eprintln!("\\n💥 GitType encountered an unexpected error:");
            eprintln!("{}", full_message);
            eprintln!("\\nThe error has been logged. Please report this issue at:");
            eprintln!("https://github.com/unhappychoice/gittype/issues");
            std::process::exit(1);
        }
    }));

    ctrlc::set_handler(move || {
        // Get EventBus from ScreenManager and publish ExitRequested event
        screen_manager
            .lock()
            .ok()
            .map(|manager| manager.get_event_bus().publish(ExitRequested))
            .unwrap_or_else(|| {
                // Fallback: just cleanup and exit
                ScreenManager::cleanup_terminal_static();
                std::process::exit(0);
            });
    })
    .expect("Error setting Ctrl-C handler");
}

/// Show panic screen using the PanicScreen component with ratatui
fn show_panic_screen(
    error_message: &str,
    screen_manager: &Arc<Mutex<ScreenManager>>,
) -> anyhow::Result<()> {
    // Initialize terminal for panic screen
    let mut raw_mode_enabled = false;
    let mut terminal_initialized = false;

    // Try to enable raw mode and enter alternate screen
    if enable_raw_mode().is_ok() {
        raw_mode_enabled = true;
        if execute!(stdout(), EnterAlternateScreen, Hide, Clear(ClearType::All)).is_ok() {
            terminal_initialized = true;
        }
    }

    // Create ratatui terminal
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // Get EventBus from ScreenManager
    let event_bus = screen_manager
        .lock()
        .ok()
        .map(|mgr| mgr.get_event_bus())
        .unwrap_or_default();
    let mut panic_screen =
        PanicScreen::with_error_message(error_message.to_string(), event_bus, None);

    let result = panic_screen_loop_ratatui(&mut terminal, &mut panic_screen, error_message);

    // Always clean up terminal state
    cleanup_panic_terminal(terminal_initialized, raw_mode_enabled);

    result
}

/// Main loop for panic screen interaction using ratatui
fn panic_screen_loop_ratatui(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    panic_screen: &mut PanicScreen,
    error_message: &str,
) -> anyhow::Result<()> {
    use crossterm::event::{poll, read, Event};
    use std::time::Duration;

    let mut needs_render = true;

    loop {
        // Only render when necessary
        if needs_render {
            terminal.draw(|frame| {
                if panic_screen.render_ratatui(frame).is_err() {
                    // If render fails, we'll exit the loop
                }
            })?;
            needs_render = false; // Reset the render flag
        }

        // Wait for input with timeout - handle errors gracefully
        match poll(Duration::from_millis(100)) {
            Ok(true) => {
                if let Ok(Event::Key(key_event)) = read() {
                    match panic_screen.handle_key_event(key_event) {
                        Ok(()) => {
                            // Only set render flag if screen state might have changed
                            needs_render = true;
                            continue;
                        }
                        Err(_) => break,
                    }
                }
            }
            Ok(false) => continue, // No input, no need to render
            Err(_) => {
                // In case of poll error, show a simple fallback message
                // Clean up terminal first
                cleanup_panic_terminal(true, true);
                eprintln!("\\n💥 PANIC SCREEN (simplified due to terminal limitations)");
                eprintln!("Error: {}", error_message);
                eprintln!("\\nPress Enter to exit...");
                let _ = std::io::stdin().read_line(&mut String::new());
                break;
            }
        }
    }

    Ok(())
}

/// Clean up terminal state after panic screen
fn cleanup_panic_terminal(terminal_initialized: bool, raw_mode_enabled: bool) {
    if terminal_initialized {
        // Exit alternate screen and show cursor
        let _ = execute!(std::io::stdout(), LeaveAlternateScreen, Show);
    }

    if raw_mode_enabled {
        // Disable raw mode
        let _ = disable_raw_mode();
    }

    // Flush stdout to ensure all output is displayed
    let _ = std::io::stdout().flush();
}
