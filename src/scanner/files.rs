use std::fs;
use std::path::Path;
use crate::stats::Stat;

pub fn get_ext_string(path: &Path) -> String {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from)
        .unwrap_or_else(|| "other".to_string())
}

pub fn read(path: &Path, stat: &mut Stat, hidden: bool) -> Result<(), Box<dyn std::error::Error>> {
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
        } else if meta.is_file() {
            stat.nb_files += 1;
            stat.total_size += meta.len();

            let ext = get_ext_string(&file_path);
            *stat.extensions.entry(ext).or_insert(0) += 1;
        }
    }

    Ok(())
}