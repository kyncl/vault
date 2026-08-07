use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;

pub fn collect_files(dir: &Path, pages: &mut Vec<PathBuf>, extension: &str) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                collect_files(&path, pages, extension)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some(extension) {
                pages.push(path);
            }
        }
    }
    Ok(())
}
