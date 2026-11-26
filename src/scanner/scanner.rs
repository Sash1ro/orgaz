use crate::cli;
use crate::file::File;
use crate::stats::Stat;
use anyhow::{Context, Result, ensure};
use std::path::{Path, PathBuf};
use std::{env, fs};

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

pub fn organize(path: &Path, stat: &Stat, dry: bool) -> Result<usize> {
    let mut moved = 0;

    for file in stat.files.values().flatten() {
        if !file.is_file {
            continue;
        }

        let dir = file.file_type.dir_name();
        let dir_path = path.join(dir);
        let final_path = dir_path.join(&file.name);

        if final_path.exists() {
            cli::warning(format!(
                "Skipped {} (already exists at {})",
                file.name,
                final_path.display()
            ));
            continue;
        }

        if dry {
            cli::step(format!("[dry] {} -> {}", file.path.display(), final_path.display()));
            moved += 1;
            continue;
        }

        fs::create_dir_all(&dir_path)
            .with_context(|| format!("Failed to create {}", dir_path.display()))?;

        fs::rename(&file.path, &final_path).with_context(|| {
            format!(
                "Failed to move {} to {}",
                file.path.display(),
                final_path.display()
            )
        })?;

        cli::info(format!("Moved {} -> {}", file.name, final_path.display()));
        moved += 1;
    }

    Ok(moved)
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

fn unique_backup_path(parent: &Path, name: &str) -> PathBuf {
    let initial = parent.join(format!("{name}.back"));
    if !initial.exists() {
        return initial;
    }

    for i in 1.. {
        let candidate = parent.join(format!("{name}.back.{i}"));
        if !candidate.exists() {
            return candidate;
        }
    }

    unreachable!("unbounded counter always finds a non-existent path")
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
    let backup_path = unique_backup_path(&parent, &name);

    fs::create_dir_all(&backup_path)
        .with_context(|| format!("Failed to create directory {}", backup_path.display()))?;

    for file in stat.files.values().flatten() {
        let dst_path = backup_path.join(&file.name);
        copy_entry(&file.path, &dst_path, file.is_dir, file.is_symlink)?;
    }

    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TempDir(PathBuf);

    impl TempDir {
        fn new(label: &str) -> Self {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = env::temp_dir().join(format!("orgaz_unit_{label}_{nanos}"));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn organize_dry_does_not_move_files() {
        let dir = TempDir::new("dry");
        fs::write(dir.0.join("photo.png"), b"img").unwrap();

        let stat = read(&dir.0, false).unwrap();
        let count = organize(&dir.0, &stat, true).unwrap();

        assert_eq!(count, 1);
        assert!(dir.0.join("photo.png").exists());
        assert!(!dir.0.join("Images").exists());
    }

    #[test]
    fn organize_skips_collisions_without_overwriting() {
        let dir = TempDir::new("collision");
        fs::create_dir_all(dir.0.join("Images")).unwrap();
        fs::write(dir.0.join("Images").join("photo.png"), b"original").unwrap();
        fs::write(dir.0.join("photo.png"), b"new").unwrap();
        fs::write(dir.0.join("notes.txt"), b"hello").unwrap();

        let stat = read(&dir.0, false).unwrap();
        let count = organize(&dir.0, &stat, false).unwrap();

        assert_eq!(count, 1);
        assert_eq!(
            fs::read(dir.0.join("Images").join("photo.png")).unwrap(),
            b"original"
        );
        assert!(dir.0.join("photo.png").exists());
        assert!(dir.0.join("Documents").join("notes.txt").exists());
    }

    #[test]
    fn backup_creates_unique_paths_on_repeat() {
        let dir = TempDir::new("backup");
        fs::write(dir.0.join("a.txt"), b"abc").unwrap();
        let stat = read(&dir.0, false).unwrap();

        let b1 = backup(&dir.0, &stat).unwrap();
        let b2 = backup(&dir.0, &stat).unwrap();

        assert_ne!(b1, b2);
        assert!(b1.join("a.txt").exists());
        assert!(b2.join("a.txt").exists());

        let _ = fs::remove_dir_all(b1);
        let _ = fs::remove_dir_all(b2);
    }
}
