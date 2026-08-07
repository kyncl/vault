use crate::{parsing::minify::minify_html, utils::page_neighbor::PageNeighbor};
use std::path::PathBuf;

pub struct Page {
    pub text: String,
    pub description: Option<String>,
    pub html: Option<String>,
    pub previous: Option<PageNeighbor>,
    pub next: Option<PageNeighbor>,
    pub metadata: PageMetadata,
}

#[derive(Clone)]
pub struct PageMetadata {
    pub md_path: PathBuf,
    pub html_path: PathBuf,
    pub name: String,
    pub rel_html_path: String,
    pub category: Option<String>,
}
impl Page {
    pub fn new(text: impl Into<String>, metadata: PageMetadata) -> Self {
        Self {
            text: text.into(),
            description: None,
            html: None,
            next: None,
            previous: None,
            metadata,
        }
    }
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn get_previous_next_btn(&self, prefix: &str) -> String {
        let previous = if let Some(prev) = &self.previous {
            let name = if prev.metadata.name.to_lowercase() == "index" {
                if let Some(cat) = &prev.metadata.category {
                    cat.to_string()
                } else {
                    "Homepage".to_string()
                }
            } else {
                prev.metadata.name.clone()
            };
            let href = format!("{}{}", prefix, prev.rel_html_path);
            minify_html(&format!(
                r#"<a href="{href}" class="pagination-btn prev">
                    <span class="pagination-label">&larr; Previous</span>
                    <span class="pagination-title">{name}</span>
                </a>"#
            ))
        } else {
            "".to_string()
        };

        let next = if let Some(next) = &self.next {
            let name = if next.metadata.name.to_lowercase() == "index" {
                if let Some(cat) = &next.metadata.category {
                    cat.to_string()
                } else {
                    "Homepage".to_string()
                }
            } else {
                next.metadata.name.clone()
            };
            let href = format!("{}{}", prefix, next.rel_html_path);
            minify_html(&format!(
                r#"<a href="{href}" class="pagination-btn next">
                    <span class="pagination-label">Next &rarr;</span>
                    <span class="pagination-title">{name}</span>
                </a>"#
            ))
        } else {
            "".to_string()
        };

        if previous.is_empty() && next.is_empty() {
            String::new()
        } else {
            format!(r#"<div class="pagination-nav">{previous}{next}</div>"#)
        }
    }
}
