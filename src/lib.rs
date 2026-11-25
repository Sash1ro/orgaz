mod cli;
mod stats;
mod file;
mod scanner;
mod commands;

use anyhow::Result;
use cli::Cli;
use clap::Parser;

pub fn run() -> Result<()> {
    let args: Cli = Cli::parse();
    commands::execute(args.command)?;

    Ok(())
}
