use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{global_props::GlobalProperty, parsing::minify::minify_html};

#[derive(Default)]
pub struct GlobalCSS {
    pub cached: Vec<PathBuf>,
    pub lazy: Vec<PathBuf>,
}

impl GlobalCSS {
    pub fn new() -> Self {
        Self {
            cached: Vec::new(),
            lazy: Vec::new(),
        }
    }
}

impl GlobalProperty for GlobalCSS {
    fn add_cached<P: AsRef<Path>>(&mut self, path: P) {
        self.cached.push(path.as_ref().to_path_buf());
    }

    fn add_lazy<P: AsRef<Path>>(&mut self, path: P) {
        self.lazy.push(path.as_ref().to_path_buf());
    }

    fn compile_cached(&self) -> Result<String> {
        let mut css = String::new();
        for path in &self.cached {
            let data = fs::read_to_string(path)?;
            css.push_str(&data);
            css.push('\n');
        }
        let css = format!("<style>{css}</style>");
        if !css.is_empty() {
            Ok(minify_html(&css))
        } else {
            Ok(String::new())
        }
    }

    fn compile_lazy(&self, asset_prefix: &str) -> Result<String> {
        let mut html_links = String::new();
        for path in &self.lazy {
            let rel_path = path
                .components()
                .skip_while(|c| c.as_os_str().to_string_lossy() != "css")
                .collect::<PathBuf>();

            let web_path = format!(
                "{}{}",
                asset_prefix,
                rel_path.to_string_lossy().replace('\\', "/")
            );

            html_links.push_str(&format!(
                r#"<link rel="stylesheet" href="{}" media="print" onload="this.media='all'">"#,
                web_path
            ));
            html_links.push('\n');
        }
        if !html_links.is_empty() {
            Ok(minify_html(&html_links))
        } else {
            Ok(String::new())
        }
    }
}
