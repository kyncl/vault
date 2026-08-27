use anyhow::Result;
use colored::*;
use serde::Serialize;
use std::path::Path;

use crate::{
    features::Features,
    global_props::{css::GlobalCSS, font::GlobalFonts, js::GlobalJS},
    html::{sidebar::SidebarSection, styling::Style},
    page::Page,
    page_ordering::PageOrderManifest,
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

    pub fn sort_pages(&mut self, manifest: Option<&PageOrderManifest>) -> &mut Self {
        if self.pages.is_empty() {
            return self;
        }

        let man = match manifest {
            Some(m) => m,
            None => return self,
        };

        let get_clean_name = |name: &str| -> String {
            if let Some(idx) = name.rfind('.') {
                name[..idx].to_lowercase()
            } else {
                name.to_lowercase()
            }
        };

        let get_clean_cat = |cat: &Option<String>| -> Option<String> {
            let cleaned = cat
                .as_ref()
                .map(|c| c.replace('\\', "/").trim_matches('/').to_lowercase())
                .filter(|c| !c.is_empty())?;

            if cleaned == "html" {
                None
            } else {
                Some(cleaned)
            }
        };

        self.pages.sort_by_cached_key(|page| {
            let clean_name = get_clean_name(&page.metadata.file_name);
            let clean_cat = get_clean_cat(&page.metadata.category);
            let exact_path = match &clean_cat {
                Some(cat) => format!("{}/{}", cat, clean_name),
                None => clean_name.clone(),
            };
            let is_not_root = exact_path != "index";
            let not_index = clean_name != "index";
            let group_name = clean_cat.clone().unwrap_or_else(|| clean_name.clone());
            let group_rank = man.get_rank(&group_name).unwrap_or(usize::MAX);
            let file_rank = match man.get_rank(&exact_path) {
                Some(rank) => rank,
                None => {
                    if !exact_path.ends_with("index"){
                        eprintln!(
                            "[{}]: The file '{}' is not defined in your ordering manifest. It will default to the bottom of its section.",
                            "WARNING".yellow(),
                            exact_path
                        );
                    }
                    usize::MAX
                }
            };

            (is_not_root, group_rank, group_name, not_index, file_rank, clean_name)
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
