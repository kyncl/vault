use std::path::Path;

use anyhow::Result;
use colored::*;
use serde::Serialize;

use crate::{
    features::Features,
    global_props::{css::GlobalCSS, font::GlobalFonts, js::GlobalJS},
    html::{sidebar::SidebarSection, styling::Style},
    page::Page,
};

#[derive(Serialize)]
pub struct SearchEntry {
    pub title: String,
    pub url: String,
    pub category: Option<String>,
    pub headers: Vec<String>,
}

#[derive(Default)]
pub struct Global {
    pub css: GlobalCSS,
    pub js: GlobalJS,
    pub fonts: GlobalFonts,
}

pub struct Vault {
    pub pages: Vec<Page>,
    pub sidebar_sections: Vec<SidebarSection>,
    pub global_elem: String, // navbar and sidebar
    pub inside_main_elem: String,
    pub global: Global,
}
impl Vault {
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
            sidebar_sections: Vec::new(),
            global: Global::default(),
            global_elem: String::new(),
            inside_main_elem: String::new(),
        }
    }

    pub fn get_headers(&self) -> Vec<(String, usize)> {
        vec![]
    }

    pub fn add_file(&mut self, page: Page) -> &mut Self {
        self.pages.push(page);
        self
    }

    pub fn pages(mut self, pages: Vec<Page>) -> Self {
        self.pages = pages;
        self
    }

    pub fn global_css(&mut self, css: GlobalCSS) -> &mut Self {
        self.global.css = css;
        self
    }

    pub fn global_js(&mut self, js: GlobalJS) -> &mut Self {
        self.global.js = js;
        self
    }
    pub fn global_fonts(&mut self, fonts: GlobalFonts) -> &mut Self {
        self.global.fonts = fonts;
        self
    }

    pub fn global_elem(&mut self, elem: impl Into<String>) -> &mut Self {
        self.global_elem = elem.into();
        self
    }

    pub fn inside_main_elem(&mut self, elem: impl Into<String>) -> &mut Self {
        self.inside_main_elem = elem.into();
        self
    }

    pub fn render_all<P: AsRef<Path>>(
        &mut self,
        html_root: P,
        style: &Style,
        features: &Features,
    ) -> Result<()> {
        println!("Parsing MD files...");
        let mut rendered_htmls = Vec::with_capacity(self.pages.len());
        for page in &self.pages {
            println!(
                "Parsing: {}",
                page.metadata.md_path.display().to_string().underline()
            );
            rendered_htmls.push(page.render(self, &html_root, style, features)?);
        }
        for (page, html) in self.pages.iter_mut().zip(rendered_htmls) {
            page.html = Some(html);
        }
        Ok(())
    }

    pub fn sort_pages(&mut self) -> &mut Self {
        if self.pages.is_empty() {
            eprintln!("Pages are empty. Did you chain correctly?");
        }
        self.pages.sort_by(|a, b| {
            let is_priority = |name: &str| {
                let lower = name.to_lowercase();
                lower == "index" || lower == "overview"
            };

            let a_prio = is_priority(&a.metadata.name);
            let b_prio = is_priority(&b.metadata.name);

            if a.metadata.category == b.metadata.category {
                if a_prio && !b_prio {
                    return std::cmp::Ordering::Less;
                } else if !a_prio && b_prio {
                    return std::cmp::Ordering::Greater;
                } else {
                    return a.metadata.name.cmp(&b.metadata.name);
                }
            }

            // Sort by category (None/root comes first)
            match (&a.metadata.category, &b.metadata.category) {
                (None, Some(_)) => std::cmp::Ordering::Less,
                (Some(_), None) => std::cmp::Ordering::Greater,
                (Some(c1), Some(c2)) => c1.cmp(c2),
                (None, None) => unreachable!(),
            }
        });
        self
    }

    pub fn generate_search_index(&self) -> String {
        let entries: Vec<SearchEntry> = self
            .pages
            .iter()
            .map(|p| SearchEntry {
                title: p.metadata.name.clone(),
                url: p.metadata.rel_html_path.clone(),
                category: p.metadata.category.clone(),
                headers: p.headers.iter().map(|(_, h)| h.clone()).collect(),
            })
            .collect();

        let json = serde_json::to_string(&entries).unwrap_or_else(|_| "[]".into());
        format!("window.VAULT_SEARCH_INDEX = {};", json)
    }
}

impl Default for Vault {
    fn default() -> Self {
        Self::new()
    }
}
