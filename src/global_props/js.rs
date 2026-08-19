use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{global_props::GlobalProperty, parsing::minify::minify_html};

#[derive(Default)]
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
    pub fn compile_with_flags(&self, use_searching: bool, asset_prefix: &str) -> Result<String> {
        let mut cached_js = String::new();
        let mut found_search_script = false;
        for path in &self.cached {
            if !use_searching {
                let name = path.file_name();
                if let Some(name) = name
                    && name.to_string_lossy().ends_with("search.js")
                {
                    found_search_script = true;
                    continue;
                }
            }
            let data = fs::read_to_string(path)?;
            cached_js.push_str(&data);
            cached_js.push('\n');
        }

        let mut html_scripts = String::new();
        for path in &self.lazy {
            if !use_searching {
                let name = path.file_name();
                if let Some(name) = name
                    && name.to_string_lossy().ends_with("search.js")
                {
                    found_search_script = true;
                    continue;
                }
            }
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

        if !found_search_script && !use_searching {
            println!(
                "Couldn't found script that handles searching. This may cause unwanted results."
            );
        }

        let js = format!("<script>{cached_js}</script>{html_scripts}");
        let minified = minify_html(&js);
        if !minified.is_empty() {
            Ok(minified)
        } else {
            Ok(String::new())
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
        println!("For compiling Javascript it is better to use `compile_with_flags`");
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
        println!("For compiling Javascript it is better to use `compile_with_flags`");
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
