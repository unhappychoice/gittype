use crate::cli::args::{Cli, Commands};
use crate::cli::commands::{run_export, run_game_session, run_history, run_stats};
use crate::game::stage_manager::{cleanup_terminal, show_session_summary_on_interrupt};
use crate::logging::{log_panic_to_file, setup_console_logging, setup_logging};
use crate::Result;

pub async fn run_cli(cli: Cli) -> Result<()> {
    // Initialize logging first for all commands
    if let Err(e) = setup_logging() {
        setup_console_logging();
        eprintln!("⚠️ Warning: Failed to setup file logging: {}", e);
        eprintln!("   Logs will only be shown in console.");
    }

    match &cli.command {
        Some(Commands::History) => run_history(),
        Some(Commands::Stats) => run_stats(),
        Some(Commands::Export { format, output }) => run_export(format.clone(), output.clone()),
        None => run_game_session(cli),
    }
}

pub fn setup_signal_handlers() {
    std::panic::set_hook(Box::new(|panic_info| {
        use std::io::{stderr, Write};

        // Log panic information to file
        log_panic_to_file(panic_info);

        // Ensure panic message is displayed before terminal cleanup
        let _ = writeln!(stderr(), "Error: {}", panic_info);
        let _ = stderr().flush();

        cleanup_terminal();

        // Display panic info again after cleanup to ensure visibility
        eprintln!("Application encountered a panic: {}", panic_info);
        eprintln!("💡 Panic details have been logged to the log file for debugging.");
    }));

    ctrlc::set_handler(move || {
        show_session_summary_on_interrupt();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}
