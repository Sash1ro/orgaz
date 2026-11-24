use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Stat {
    pub nb_files: u64,
    pub nb_dirs: u64,
    pub nb_symlinks: u64,
    pub total_size: u64,
    pub extensions: HashMap<String, u64>,
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