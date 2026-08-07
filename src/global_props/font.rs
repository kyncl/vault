use std::path::{Path, PathBuf};

use anyhow::{Ok, Result};

use crate::global_props::GlobalProperty;

#[derive(Default)]
pub struct GlobalFonts {
    pub fonts: Vec<PathBuf>,
}

impl GlobalFonts {
    pub fn new() -> Self {
        Self { fonts: Vec::new() }
    }

    fn parse_fonts(&self, asset_prefix: &str) -> Result<String> {
        if self.fonts.is_empty() {
            return Ok(String::new());
        }

        let mut preloads = String::new();
        for font_path in &self.fonts {
            let file_name = font_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let ext = font_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            preloads.push_str(&format!(
                "    <link rel=\"preload\" href=\"{asset_prefix}fonts/{file_name}\" as=\"font\" type=\"font/{ext}\" crossorigin>\n"
            ));
        }
        Ok(preloads)
    }
}

impl GlobalProperty for GlobalFonts {
    fn add_cached<P: AsRef<Path>>(&mut self, path: P) {
        self.fonts.push(path.as_ref().to_path_buf());
    }

    fn add_lazy<P: AsRef<Path>>(&mut self, path: P) {
        self.fonts.push(path.as_ref().to_path_buf());
    }

    fn compile_cached(&self) -> Result<String> {
        // All fonts are lazy
        Ok(String::new())
    }

    fn compile_lazy(&self, asset_prefix: &str) -> Result<String> {
        self.parse_fonts(asset_prefix)
    }
}
