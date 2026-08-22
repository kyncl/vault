use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};

use crate::{CONFIG_FILE, CONFIGURATION_FOLDER};

pub fn resolve_config_paths<P: AsRef<Path>>(input: Option<P>) -> Result<(PathBuf, PathBuf)> {
    let raw = input
        .map(|p| p.as_ref().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    if raw.is_file() {
        let parent = raw.parent().unwrap_or(Path::new(".")).to_path_buf();
        let parent_name = parent.file_name().unwrap_or_default().to_string_lossy();

        if parent_name.starts_with(CONFIGURATION_FOLDER) && parent.exists() && raw.exists() {
            Ok((raw, parent))
        } else {
            let p_with_app = parent.join(CONFIGURATION_FOLDER);
            if p_with_app.exists() && raw.exists() {
                Ok((raw, p_with_app))
            } else {
                Err(anyhow!(
                    "Couldn't find {CONFIGURATION_FOLDER}. Please create {CONFIGURATION_FOLDER}/ in your workspace directory"
                ))
            }
        }
    } else {
        let dir_name = raw.file_name().unwrap_or_default().to_string_lossy();
        if dir_name.starts_with(CONFIGURATION_FOLDER)
            && raw.exists()
            && raw.join(CONFIG_FILE).exists()
        {
            Ok((raw.join(CONFIG_FILE), raw))
        } else {
            let config_dir = raw.join(CONFIGURATION_FOLDER);
            if config_dir.exists() && config_dir.join(CONFIG_FILE).exists() {
                Ok((config_dir.join(CONFIG_FILE), config_dir))
            } else {
                Err(anyhow!(
                    "Couldn't find {CONFIGURATION_FOLDER}. Please create {CONFIGURATION_FOLDER}/ in your workspace directory"
                ))
            }
        }
    }
}
