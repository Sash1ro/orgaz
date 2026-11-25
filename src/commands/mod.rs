pub mod run;
pub mod stats;

use crate::cli::Commands;
use anyhow::Result;

pub fn execute(command: Commands) -> Result<()> {
    match command {
        Commands::Run {
            target,
            dry,
            hidden,
        } => {
            run::execute(target, dry, hidden)?;
        }
        Commands::Stats {
            target,
            json,
            hidden,
        } => {
            stats::execute(target, json, hidden)?;
        }
    }

    Ok(())
}
