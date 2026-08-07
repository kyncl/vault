use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{global_props::GlobalProperty, parsing::minify::minify_html};

pub struct GlobalJS {
    pub cached: Vec<PathBuf>,
    pub lazy: Vec<PathBuf>,
}

impl GlobalJS {
    pub fn new() -> Self {
        Self {
            cached: Vec::new(),
            lazy: Vec::new(),
        }
    }

    /// Automatically routes paths based on filename conventions
    /// By default all css files are cached means injected into the html
    /// To load it later the file must have `lazy__` prefix
    pub fn add_auto<P: AsRef<Path>>(&mut self, path: P) {
        let path_ref = path.as_ref();
        let file_name = path_ref.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if file_name.to_lowercase().strip_prefix("lazy__").is_some() {
            self.add_lazy(path_ref);
        } else {
            self.add_cached(path_ref);
        }
    }
}

impl GlobalProperty for GlobalJS {
    fn add_cached<P: AsRef<Path>>(&mut self, path: P) {
        self.cached.push(path.as_ref().to_path_buf());
    }

    fn add_lazy<P: AsRef<Path>>(&mut self, path: P) {
        self.lazy.push(path.as_ref().to_path_buf());
    }

    fn compile_cached(&self) -> Result<String> {
        let mut js = String::new();
        for path in &self.cached {
            let data = fs::read_to_string(path)?;
            js.push_str(&data);
            js.push('\n');
        }

        let js = format!("<script>{js}</script>");
        let minified = minify_html(&js);
        if !minified.is_empty() {
            Ok(minified)
        } else {
            Ok(String::new())
        }
    }

    fn compile_lazy(&self, asset_prefix: &str) -> Result<String> {
        let mut html_scripts = String::new();
        for path in &self.lazy {
            let rel_path = path
                .components()
                .skip_while(|c| c.as_os_str().to_string_lossy() != "js")
                .collect::<PathBuf>();

            let web_path = format!(
                "{}{}",
                asset_prefix,
                rel_path.to_string_lossy().replace('\\', "/")
            );

            html_scripts.push_str(&format!(r#"<script src="{}" defer></script>"#, web_path));
            html_scripts.push('\n');
        }
        if !html_scripts.is_empty() {
            Ok(minify_html(&html_scripts))
        } else {
            Ok(String::new())
        }
    }
}
