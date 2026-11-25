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
    let stat: Stat = scanner::read(&path, hidden)?;

    if !dry {
        scanner::backup(&path)?;
    } 

    Ok(())
}
