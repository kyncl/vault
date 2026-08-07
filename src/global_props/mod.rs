use anyhow::Result;
use std::path::Path;

pub mod css;
pub mod js;

pub trait GlobalProperty {
    fn add_cached<P: AsRef<Path>>(&mut self, path: P);
    fn add_lazy<P: AsRef<Path>>(&mut self, path: P);
    fn compile_lazy(&self, asset_prefix: &str) -> Result<String>;
    fn compile_cached(&self) -> Result<String>;
}
