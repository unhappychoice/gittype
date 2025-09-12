use crate::cli::args::{Cli, Commands};
use crate::cli::commands::{run_export, run_game_session, run_history, run_stats};
use crate::logging::{setup_console_logging, setup_logging};
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
