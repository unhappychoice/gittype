use clap::Parser;
use gittype::cli::{run_cli, setup_signal_handlers, Cli};
use gittype::logging::log_error_to_file;

fn main() -> anyhow::Result<()> {
    setup_signal_handlers();

    let cli = Cli::parse();

    if let Err(e) = run_cli(cli) {
        // Log error details before exiting
        log_error_to_file(&e);
        return Err(e.into());
    }

    Ok(())
}
