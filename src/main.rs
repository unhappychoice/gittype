use clap::Parser;
use gittype::cli::{run_cli, setup_signal_handlers, Cli};

fn main() -> anyhow::Result<()> {
    setup_signal_handlers();

    let cli = Cli::parse();
    run_cli(cli)?;

    Ok(())
}
