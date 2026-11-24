use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "orgaz")]
#[command(version)]
#[command(about = "A simple file organizer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Organize files in a directory
    Run {
        target: Option<PathBuf>,

        /// Preview changes without moving files
        #[arg(long)]
        dry: bool,

        /// Include hidden files
        #[arg(long)]
        hidden: bool,
    },

    /// Display directory statistics
    Stats {
        target: Option<PathBuf>,

        /// Output statistics as JSON
        #[arg(long)]
        json: bool,

        /// Include hidden files
        #[arg(long)]
        hidden: bool,

    },

    
}