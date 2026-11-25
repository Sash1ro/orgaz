use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::HashMap;
use crate::file::File;

#[derive(Serialize)]
pub struct Stat {
    pub nb_files: u64,
    pub nb_dirs: u64,
    pub nb_symlinks: u64,
    pub total_size: u64,
    pub files: HashMap<String, Vec<File>>
}

impl Stat {
    pub fn new() -> Self {
        Stat {
            nb_files: 0,
            nb_dirs: 0,
            nb_symlinks: 0,
            total_size: 0,
            files: HashMap::new(),
        }
    }

    pub fn show(&self) {
        println!("Files : {}", self.nb_files);
        println!("Dirs : {}", self.nb_dirs);
        println!("Symlinks : {}", self.nb_symlinks);
        println!("Size : {}b", self.total_size);
        println!("Extensions : ");

        let mut sorted: Vec<(&String, usize)> = self
            .files
            .iter()
            .filter_map(|(ext, files)| {
                let count = files.iter().filter(|f| f.is_file).count();
                (count > 0).then_some((ext, count))
            })
            .collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        for (key, count) in sorted {
            println!(" - {} : {}", key, count);
        }
    }

    pub fn to_json(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .context("Failed to serialize stats to JSON")?;
        println!("{json}");
        Ok(())
    }

    pub fn add_file(&mut self, file: File) {
        self.total_size += file.size;

        if file.is_symlink {
            self.nb_symlinks += 1;
        } else if file.is_dir {
            self.nb_dirs += 1;
        } else if file.is_file {
            self.nb_files += 1;
        }

        self.files
            .entry(file.extension.to_owned())
            .or_default()
            .push(file);
    }
}