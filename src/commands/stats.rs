use crate::scanner;
use crate::stats::Stat;
use std::env;
use std::path::PathBuf;

pub fn execute(
    target: Option<PathBuf>,
    json: bool,
    hidden: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stat: Stat = Stat::new();
    let path: PathBuf = match target {
        Some(p) => p,
        None => env::current_dir()?,
    };

    scanner::read(&path, &mut stat, hidden)?;

    if json {
        stat.to_json()?;
    } else {
        stat.show();
    }

    Ok(())
}
