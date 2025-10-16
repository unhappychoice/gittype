use clap::Parser;
use gittype::infrastructure::logging::log_error_to_file;
use gittype::presentation::cli::{run_cli, Cli};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Err(e) = run_cli(cli) {
        log_error_to_file(&e);
        return Err(e.into());
    }

    Ok(())
}
