use std::path::Path;

use anyhow::{Result, anyhow};

pub mod autocomplete;
pub mod config_path;
pub mod crawler;
pub mod page_neighbor;
pub mod slugify;

pub fn convert_home_path<P>(path: P, username: Option<String>) -> Result<String>
where
    P: AsRef<Path>,
{
    let mut path = path.as_ref().to_string_lossy().to_string();
    let home_dir = {
        let err = anyhow!("Couldn't get home directory of this system");
        let home_path = dirs::home_dir().ok_or(err)?;

        if let Some(username) = username {
            if path.starts_with("%USERPROFILE%") {
                format!("C:\\Users\\{username}")
            } else {
                format!("/home/{username}")
            }
        } else {
            home_path.to_string_lossy().to_string()
        }
    };

    if path.starts_with("~") {
        path = path.replacen("~", &home_dir, 1);
    }
    if !cfg!(unix) {
        path = path.replace("%USERPROFILE%", &home_dir);
    }
    Ok(path)
}
