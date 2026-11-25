use std::{fs::{self, DirEntry, Metadata}, path::{self, Path, PathBuf}};
use serde::Serialize;
use anyhow::{Context, Result};

fn get_ext_string(path: &Path) -> String {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from)
        .unwrap_or_else(|| "other".to_owned())
}

fn is_hidden(filename: &str) -> bool {
    filename.starts_with(".")
}

#[derive(Serialize)]
pub struct File {
    pub name : String,
    pub extension : String,
    pub path: PathBuf,
    pub hidden: bool,
    pub is_symlink: bool,
    pub is_dir: bool,
    pub is_file: bool,
    pub size: u64,
}

impl File {
    pub fn new_from_dir_entry(file: &DirEntry) -> Result<Self> {
        let path: PathBuf = file.path();
        let metadata: Metadata = fs::symlink_metadata(&path)
            .with_context(|| format!("Unable to find metadata for {}", path.display()))?;
        let filename: String = file.file_name().to_string_lossy().to_string();
        let abs_path: PathBuf = path::absolute(&path)
            .with_context(|| format!("Failed to resolve file path : {}", path.display()))?;
        let hidden = is_hidden(&filename);
        Ok(File {
            name: filename,
            extension: get_ext_string(&path),
            path: abs_path,
            hidden,
            is_symlink: metadata.file_type().is_symlink(),
            is_dir: metadata.file_type().is_dir(),
            is_file: metadata.file_type().is_file(),
            size: metadata.len()
        })
    }
}