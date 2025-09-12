use crate::game::screen_manager::ScreenManager;
use crate::logging::log_panic_to_file;

pub fn setup_signal_handlers() {
    std::panic::set_hook(Box::new(|panic_info| {
        // Restore terminal to normal state
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(
            std::io::stderr(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::cursor::Show
        );

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
        let error = crate::error::GitTypeError::PanicError(full_message.clone());
        crate::logging::log_error_to_file(&error);

        // Try to show panic screen - if this fails, fall back to standard panic behavior
        if show_panic_screen(&full_message).is_err() {
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
        ScreenManager::show_session_summary_on_interrupt();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}

/// Show panic screen using the PanicScreen component with ratatui
fn show_panic_screen(error_message: &str) -> anyhow::Result<()> {
    use crate::game::{models::Screen, screens::PanicScreen};
    use crossterm::{
        cursor::Hide,
        execute,
        terminal::{enable_raw_mode, Clear, ClearType, EnterAlternateScreen},
    };
    use ratatui::{backend::CrosstermBackend, Terminal};
    use std::io::stdout;

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

    let mut panic_screen = PanicScreen::with_error_message(error_message.to_string());

    // Initialize the panic screen
    if panic_screen.init().is_err() {
        cleanup_panic_terminal(terminal_initialized, raw_mode_enabled);
        return Err(anyhow::anyhow!("Failed to initialize panic screen"));
    }

    let result = panic_screen_loop_ratatui(&mut terminal, &mut panic_screen, error_message);

    // Always clean up terminal state
    cleanup_panic_terminal(terminal_initialized, raw_mode_enabled);

    result
}

/// Main loop for panic screen interaction using ratatui
fn panic_screen_loop_ratatui(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    panic_screen: &mut crate::game::screens::PanicScreen,
    error_message: &str,
) -> anyhow::Result<()> {
    use crate::game::models::{Screen, ScreenTransition};
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
                        Ok(ScreenTransition::Exit | ScreenTransition::Replace(_)) => break,
                        Ok(_) => {
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
    use crossterm::{
        cursor::Show,
        execute,
        terminal::{disable_raw_mode, LeaveAlternateScreen},
    };

    if terminal_initialized {
        // Exit alternate screen and show cursor
        let _ = execute!(std::io::stdout(), LeaveAlternateScreen, Show);
    }

    if raw_mode_enabled {
        // Disable raw mode
        let _ = disable_raw_mode();
    }

    // Flush stdout to ensure all output is displayed
    use std::io::Write;
    let _ = std::io::stdout().flush();
}
