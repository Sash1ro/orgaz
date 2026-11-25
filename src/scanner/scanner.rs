use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result, bail, ensure};
use crate::stats::Stat;
use crate::file::File;

pub fn read(path: &Path, show_hidden: bool) -> Result<Stat> {
    let content = fs::read_dir(path)
        .with_context(|| format!("Failed to read directory {}", path.display()))?;
    let mut stats = Stat::new();

    for entry in content {
        let entry = entry.with_context(|| format!("Failed to read entry in {}", path.display()))?;
        let entry_path = entry.path();
       
        let file: File = File::new_from_dir_entry(&entry)
            .with_context(|| format!("Failed to inspect {}", entry_path.display()))?;

        if !show_hidden && file.hidden {
            continue;
        }

        stats.add_file(file);
    }

    Ok(stats)
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)
        .with_context(|| format!("Failed to create directory {}", dst.display()))?;

    let entries = fs::read_dir(src)
        .with_context(|| format!("Failed to read directory {}", src.display()))?;

    for entry in entries {
        let entry = entry.with_context(|| format!("Failed to read entry in {}", src.display()))?;
        let file_type = entry
            .file_type()
            .with_context(|| format!("Failed to read file type for {}", entry.path().display()))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else if file_type.is_symlink() {
            let link_target = fs::read_link(&src_path)
                .with_context(|| format!("Failed to read symlink {}", src_path.display()))?;
            #[cfg(unix)]
            std::os::unix::fs::symlink(&link_target, &dst_path).with_context(|| {
                format!(
                    "Failed to create symlink {} -> {}",
                    dst_path.display(),
                    link_target.display()
                )
            })?;
        } else {
            fs::copy(&src_path, &dst_path).with_context(|| {
                format!("Failed to copy {} to {}", src_path.display(), dst_path.display())
            })?;
        }
    }

    Ok(())
}


pub fn backup(path: &Path) -> Result<PathBuf> {
    let abs_path = fs::canonicalize(path)
        .with_context(|| format!("Failed to resolve backup path {}", path.display()))?;
    ensure!(
        abs_path.is_dir(),
        "Backup target is not a directory: {}",
        abs_path.display()
    );

    let name = abs_path
        .file_name()
        .context("Could not determine directory name")?
        .to_string_lossy();
    let parent = abs_path
        .parent()
        .context("Could not determine parent directory")?;
    let backup_path = parent.join(format!("{name}.back"));

    if backup_path.exists() {
        bail!("Backup directory already exists: {}", backup_path.display());
    }

    copy_dir_recursive(&abs_path, &backup_path)?;
    Ok(backup_path)
}
