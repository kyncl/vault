use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use ignore::gitignore::Gitignore;

use crate::ignore::should_ignore;

pub fn collect_files(
    dir: &Path,
    project_root: &Path,
    pages: &mut Vec<PathBuf>,
    extension: &str,
    ignorer: &Gitignore,
) -> Result<()> {
    if should_ignore(dir, project_root, ignorer, dir.is_dir()) {
        return Ok(());
    }

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let meta = entry.metadata()?;
            if should_ignore(&path, project_root, ignorer, meta.is_dir()) {
                continue;
            }

            if path.is_dir() {
                collect_files(&path, project_root, pages, extension, ignorer)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some(extension) {
                pages.push(path);
            }
        }
    }
    Ok(())
}
