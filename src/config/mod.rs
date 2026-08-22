use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::{features::Features, html::styling::Style};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub title: String,
    pub md_path: PathBuf,
    pub html_path: PathBuf,

    pub features: Features,
    pub styling: Style,
}

impl Configuration {
    pub fn new<P: AsRef<Path>, T: AsRef<Path>>(
        md_path: P,
        html_path: T,
        features: Features,
        styling: Style,
        title: String,
    ) -> Self {
        Self {
            styling,
            features,
            md_path: md_path.as_ref().to_path_buf(),
            html_path: html_path.as_ref().to_path_buf(),
            title,
        }
    }

    pub fn to_toml(&self) -> Result<String> {
        Ok(toml::to_string(self)?)
    }

    pub fn from_string<S: AsRef<str>>(data: S) -> Result<Self> {
        Ok(toml::from_str(data.as_ref())?)
    }
}
