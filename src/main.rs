use clap::Parser;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    target: Option<PathBuf>,

    #[arg(long)]
    hidden: bool,

    #[arg(long)]
    json: bool,
}

#[derive(Serialize)]
struct Stat {
    nb_files: u64,
    nb_dirs: u64,
    nb_symlinks: u64,
    total_size: u64,
    extensions: HashMap<String, u64>,
}

impl Stat {
    pub fn new() -> Self {
        Stat {
            nb_files: 0,
            nb_dirs: 0,
            nb_symlinks: 0,
            total_size: 0,
            extensions: HashMap::new(),
        }
    }

    pub fn show(&self) {
        println!("Files stats");
        println!("Files : {}", self.nb_files);
        println!("Dirs : {}", self.nb_dirs);
        println!("Symlinks : {}", self.nb_symlinks);
        println!("Size : {}b", self.total_size);
        println!("---");
        println!("Extensions : ");

        let mut sorted: Vec<(&String, &u64)> = self.extensions.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));

        for (key, val) in sorted {
            println!(" - {} : {}", key, val);
        }
    }

    pub fn to_json(&self) -> Result<(), serde_json::Error> {
        println!("{}", serde_json::to_string_pretty(self)?);
        Ok(())
    }
}

fn get_ext_string(path: &Path) -> String {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from)
        .unwrap_or_else(|| "other".to_string())
}

fn read(path: &Path, stat: &mut Stat, hidden: bool) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_dir(path)?;

    for f in content {
        let file = f?;
        let filename = file.file_name();

        if !hidden && filename.to_string_lossy().starts_with('.') {
            continue;
        }

        let file_path = file.path();
        let meta = fs::symlink_metadata(&file_path)?;

        if meta.file_type().is_symlink() {
            stat.nb_symlinks += 1;
        } else if meta.is_dir() {
            stat.nb_dirs += 1;
            read(&file_path, stat, hidden)?;
        } else if meta.is_file() {
            stat.nb_files += 1;
            stat.total_size += meta.len();

            let ext = get_ext_string(&file_path);
            *stat.extensions.entry(ext).or_insert(0) += 1;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let target = args.target.unwrap_or(env::current_dir()?);
    let mut stat = Stat::new();

    read(&target, &mut stat, args.hidden)?;

    if args.json {
        stat.to_json()?;
    } else {
        stat.show();
    }

    Ok(())
}
