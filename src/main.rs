use clap::Parser;
use gittype::presentation::cli::{run_cli, Cli};
use gittype::logging::log_error_to_file;
use gittype::signal_handler::setup_signal_handlers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_signal_handlers();

    let cli = Cli::parse();

    if let Err(e) = run_cli(cli).await {
        log_error_to_file(&e);
        return Err(e.into());
    }

    Ok(())
}
