mod cli;
mod stats;
mod scanner;
mod commands;

use cli::Cli;
use clap::Parser;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::parse();
    commands::execute(args.command)?;

    Ok(())
}
