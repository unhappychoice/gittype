use crate::cli::args::{Cli, Commands};
use crate::cli::commands::{run_export, run_game_session, run_history, run_stats};
use crate::game::stage_manager::{cleanup_terminal, show_session_summary_on_interrupt};
use crate::Result;

pub fn run_cli(cli: Cli) -> Result<()> {
    match cli.command {
        Some(Commands::History) => run_history()?,
        Some(Commands::Stats) => run_stats()?,
        Some(Commands::Export { format, output }) => run_export(format, output)?,
        None => run_game_session(cli)?,
    }

    Ok(())
}

pub fn setup_signal_handlers() {
    std::panic::set_hook(Box::new(|_| {
        cleanup_terminal();
    }));

    ctrlc::set_handler(move || {
        show_session_summary_on_interrupt();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}
