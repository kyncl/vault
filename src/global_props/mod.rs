use anyhow::Result;
use std::path::Path;

use crate::utils::crawler::collect_files;

pub mod css;
pub mod font;
pub mod js;

pub trait GlobalProperty {
    fn add_cached<P: AsRef<Path>>(&mut self, path: P);
    fn add_lazy<P: AsRef<Path>>(&mut self, path: P);
    fn compile_lazy(&self, asset_prefix: &str) -> Result<String>;
    fn compile_cached(&self) -> Result<String>;

    /// Automatically routes paths based on filename conventions
    /// By default all files are cached means injected into the html
    /// To load it later the file must have `lazy__` prefix
    fn add<P: AsRef<Path>>(&mut self, path: P) {
        let path_ref = path.as_ref();
        let file_name = path_ref.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if file_name.to_lowercase().strip_prefix("lazy__").is_some() {
            self.add_lazy(path_ref);
        } else {
            self.add_cached(path_ref);
        }
    }

    fn set_global_property<P: AsRef<Path>>(&mut self, folder: P, extension: &str) -> Result<()> {
        let mut files = vec![];
        collect_files(folder.as_ref(), &mut files, extension)?;
        files.sort();
        files.iter().for_each(|file| self.add(file));
        Ok(())
    }
}
