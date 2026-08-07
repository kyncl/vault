use std::path::Path;

use crate::{
    global_props::GlobalProperty,
    html::{sidebar_items::generate_sidebar_items, styling::Style, toc::generate_toc},
    page::Page,
    parsing::{highlighting::highlight_html_blocks, minify::minify_html},
    utils::slugify::add_heading_ids,
    vault::Vault,
};
use anyhow::{Result, anyhow};
use markdown::{CompileOptions, Constructs, Options, ParseOptions};

impl Page {
    pub fn render<P: AsRef<Path>>(
        &self,
        vault: &Vault,
        html_root: P,
        style: &Style,
    ) -> Result<String> {
        let options = Options {
            parse: ParseOptions {
                constructs: Constructs {
                    gfm_table: true,
                    gfm_task_list_item: true,
                    gfm_strikethrough: true,
                    gfm_autolink_literal: true,
                    html_flow: true,
                    html_text: true,
                    ..Constructs::default()
                },
                ..ParseOptions::default()
            },
            compile: CompileOptions {
                allow_dangerous_html: true,
                ..CompileOptions::default()
            },
        };

        let description = self
            .description
            .as_deref()
            .unwrap_or("Part of Knot documentation");
        let dynamic_desc = if self.description.is_some() {
            description.to_string()
        } else {
            format!("{} about {}", description, self.metadata.name)
        };

        let asset_depth = self.metadata.rel_html_path.matches('/').count();
        let asset_prefix = "../".repeat(asset_depth);

        let html_dir = self
            .metadata
            .html_path
            .parent()
            .unwrap_or(std::path::Path::new(""));
        let depth_to_root = html_dir
            .components()
            .filter(|c| matches!(c, std::path::Component::Normal(_)))
            .count();

        let back_to_root = "../".repeat(depth_to_root);
        let clean_md_path: std::path::PathBuf = self
            .metadata
            .md_path
            .components()
            .filter(|c| matches!(c, std::path::Component::Normal(_)))
            .collect();
        let md_path_str = clean_md_path.to_string_lossy().replace('\\', "/");
        let to_md = format!("{}{}", back_to_root, md_path_str);

        let raw_docs = markdown::to_html_with_options(&self.text, &options)
            .map_err(|e| anyhow!("Error found during markdown parsing. Cause: {e}"))?;
        let docs = add_heading_ids(&raw_docs);
        let toc = generate_toc(&docs);

        let js = format!(
            "{}\n{}",
            vault.global.js.compile_cached()?,
            vault.global.js.compile_lazy(&asset_prefix)?,
        );
        let styling = format!(
            "{}\n{}",
            vault.global.css.compile_cached()?,
            vault.global.css.compile_lazy(&asset_prefix)?
        );
        let fonts = format!(
            "{}\n{}",
            vault.global.fonts.compile_cached()?,
            vault.global.fonts.compile_lazy(&asset_prefix)?
        );

        let html = format!(
            r#"<!DOCTYPE html>
            <html lang="en" {headers}>
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>{title}</title>
                <meta name="description" content="{dynamic_desc}">
                <link rel="icon" type="image/x-icon" href="./icon.svg">
                {styling}
                {js}
                {fonts}
            </head>
            <body class="jetbrains-mono background foreground">
                {global_elem}
                <main class="doc-content">
                    {inside_main_elem}
                    <div>
                        {docs}
                    </div>
                    %__PAGINATION__%
                </main>
                <aside class="toc-sidebar">
                    {toc}
                </aside>
                <div class="raw-md">
                    <a href="{to_md}" aria-label="to md file">Raw Markdown</a>
                </div>
            </body>
            </html>"#,
            title = self.metadata.name,
            global_elem = vault.global_elem,
            inside_main_elem = vault.inside_main_elem,
            headers = style.make_header()
        );

        let rel_from_root = self.metadata.html_path.strip_prefix(html_root)?;
        let depth = rel_from_root.parent().map_or(0, |p| p.components().count());
        let prefix = "../".repeat(depth);

        let sections_html = generate_sidebar_items(&vault.sidebar_sections, &prefix);
        let pagination_html = self.get_previous_next_btn(&prefix);
        let html = html
            .replace("%__HOME_HREF__%", &prefix)
            .replace("%__SIDEBAR_SECTIONS__%", &sections_html)
            .replace("%__PAGINATION__%", &pagination_html);
        Ok(minify_html(&highlight_html_blocks(&html)))
    }
}
