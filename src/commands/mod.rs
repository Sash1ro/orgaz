pub mod stats;

use crate::cli::Commands;

pub fn execute(command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::Run { target, dry, hidden } => {},
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
