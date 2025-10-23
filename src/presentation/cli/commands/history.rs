use crate::infrastructure::console::{Console, ConsoleImpl};
use crate::Result;

pub fn run_history() -> Result<()> {
    let console = ConsoleImpl::new();
    console.eprintln("❌ History command is not yet implemented")?;
    console.eprintln("💡 This feature is planned for a future release")?;
    std::process::exit(1);
}
