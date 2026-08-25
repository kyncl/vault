use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::{
    page::{Page, PageMetadata},
    vault::Vault,
};

impl Vault {
    pub fn set_pages(
        &mut self,
        md_files: Vec<PathBuf>,
        md_root: &Path,
        html_root: &Path,
    ) -> Result<&mut Self> {
        for path in md_files {
            if let Ok(data_str) = fs::read_to_string(&path) {
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                let name = file_name
                    .strip_suffix(".md")
                    .unwrap_or(&file_name)
                    .to_string();
                let name = name.replace("-", " ").replace("_", " ");

                let rel_path = path.strip_prefix(md_root)?;
                let html_path = html_root.join(rel_path).with_extension("html");
                let rel_html_path = html_path
                    .strip_prefix(html_root)?
                    .to_string_lossy()
                    .replace('\\', "/");

                // Root pages have no parent vault category; nested pages get a capitalized category name
                let parent_dir = rel_path.parent();
                let category = if parent_dir.is_none_or(|p| p.as_os_str().is_empty()) {
                    None
                } else {
                    let folder_str = parent_dir.unwrap().to_string_lossy();
                    let formatted = folder_str.replace("-", " ").replace("_", " ");
                    let mut chars = formatted.chars();
                    Some(match chars.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().to_string() + chars.as_str(),
                    })
                };

                let metadata = PageMetadata {
                    md_path: path,
                    html_path,
                    name,
                    rel_html_path,
                    category,
                    file_name,
                };
                let data = Page::new(&data_str, metadata);
                self.add_file(data);
            }
        }
        Ok(self)
    }
}
