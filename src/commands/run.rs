use crate::cli;
use crate::scanner;
use crate::stats::Stat;
use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

pub fn execute(target: Option<PathBuf>, dry: bool, hidden: bool) -> Result<()> {
    let path: PathBuf = match target {
        Some(p) => p,
        None => env::current_dir().context("Failed to get current directory")?,
    };

    cli::step(format!("Scanning {}", path.display()));
    let stat: Stat = scanner::read(&path, hidden)?;

    if dry {
        cli::info("Dry run enabled, no files will be moved");
    } else {
        let backup_path = scanner::backup(&path, &stat)?;
        cli::success(format!("Backup created at {}", backup_path.display()));
    }

    let moved = scanner::organize(&path, &stat, dry)?;

    if dry {
        cli::info(format!("Would organize {moved} file(s)"));
    } else {
        cli::success(format!("Organized {moved} file(s)"));
    }

    Ok(())
}
