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
    pub file_type: Type
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Type {
    Document,
    Image,
    Video,
    Audio,
    Archive,
    Code,
    Exe,
    Other,
}

impl Type {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_ascii_lowercase().as_str() {
            "pdf" | "doc" | "docx" | "txt" | "md" | "csv" | "xlsx" | "odp" => Self::Document,
            "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" => Self::Image,
            "mp4" | "mov" | "mkv" | "avi" | "webm" => Self::Video,
            "mp3" | "wav" | "flac" | "aac" | "ogg" => Self::Audio,
            "zip" | "tar" | "gz" | "7z" | "rar" => Self::Archive,
            "rs" | "py" | "js" | "ts" | "json" | "toml" | "html" | "css" => Self::Code,
            "exe" | "msi" | "dmg" | "pkg" | "app" | "deb" | "apk" => Self::Exe,
            _ => Self::Other,
        }
    }
    pub fn dir_name(&self) -> &'static str {
        match self {
            Self::Document => "Documents",
            Self::Image => "Images",
            Self::Video => "Videos",
            Self::Audio => "Audio",
            Self::Archive => "Archives",
            Self::Code => "Code",
            Self::Exe => "Executables",
            Self::Other => "Other",
        }
    }
}

impl File {
    pub fn new_from_dir_entry(file: &DirEntry) -> Result<Self> {
        let path: PathBuf = file.path();
        let filename: String = file.file_name().to_string_lossy().to_string();
        let hidden: bool = is_hidden(&filename);
        let ext: String = get_ext_string(&path);
        let f_type: Type = Type::from_extension(&ext);
        let abs_path: PathBuf = path::absolute(&path)
            .with_context(|| format!("Failed to resolve file path : {}", path.display()))?;
        let metadata: Metadata = fs::symlink_metadata(&path)
            .with_context(|| format!("Unable to find metadata for {}", path.display()))?;

        Ok(File {
            name: filename,
            extension: ext,
            path: abs_path,
            hidden,
            is_symlink: metadata.file_type().is_symlink(),
            is_dir: metadata.file_type().is_dir(),
            is_file: metadata.file_type().is_file(),
            size: metadata.len(),
            file_type: f_type
        })
    }
}