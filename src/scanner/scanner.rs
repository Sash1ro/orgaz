use crate::file::File;
use crate::stats::Stat;
use anyhow::{Context, Result, bail, ensure};
use std::{env, fs};
use std::path::{Path, PathBuf};

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

pub fn organize(path: &Path, stat: &Stat, dry: bool) -> Result<()> {
    for file in stat.files.values().flatten() {
        if !file.is_file {
            continue;
        }

        let dir = file.file_type.dir_name();
        let dir_path = path.join(dir);
        let final_path = dir_path.join(file.name.to_owned());

        if dry {
            println!("[dry] {} => {}", file.path.display(), final_path.display());
            continue;
        }

        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)
                .with_context(|| format!("Failed to create {}", dir_path.display()))?;
        }

        fs::rename(&file.path, &final_path)
            .with_context(|| format!("Failed to move {}", file.path.display()))?;
    }

    Ok(())
}

fn copy_entry(src_path: &Path, dst_path: &Path, is_dir: bool, is_symlink: bool) -> Result<()> {
    if is_dir {
        copy_dir_recursive(src_path, dst_path)?;
    } else if is_symlink {
        let link_target = fs::read_link(src_path)
            .with_context(|| format!("Failed to read symlink {}", src_path.display()))?;
        #[cfg(unix)]
        std::os::unix::fs::symlink(&link_target, dst_path).with_context(|| {
            format!(
                "Failed to create symlink {} -> {}",
                dst_path.display(),
                link_target.display()
            )
        })?;
    } else {
        fs::copy(src_path, dst_path).with_context(|| {
            format!(
                "Failed to copy {} to {}",
                src_path.display(),
                dst_path.display()
            )
        })?;
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)
        .with_context(|| format!("Failed to create directory {}", dst.display()))?;

    let entries =
        fs::read_dir(src).with_context(|| format!("Failed to read directory {}", src.display()))?;

    for entry in entries {
        let entry = entry.with_context(|| format!("Failed to read entry in {}", src.display()))?;
        let file_type = entry
            .file_type()
            .with_context(|| format!("Failed to read file type for {}", entry.path().display()))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        copy_entry(
            &src_path,
            &dst_path,
            file_type.is_dir(),
            file_type.is_symlink(),
        )?;
    }

    Ok(())
}

pub fn backup(path: &Path, stat: &Stat) -> Result<PathBuf> {
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
    let parent = env::temp_dir();
    let backup_path = parent.join(format!("{name}.back"));

    if backup_path.exists() {
        bail!("Backup directory already exists: {}", backup_path.display());
    }

    fs::create_dir_all(&backup_path)
        .with_context(|| format!("Failed to create directory {}", backup_path.display()))?;

    for file in stat.files.values().flatten() {
        let dst_path = backup_path.join(&file.name);
        copy_entry(&file.path, &dst_path, file.is_dir, file.is_symlink)?;
    }

    Ok(backup_path)
}
