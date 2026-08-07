use crate::{
    global_props::GlobalProperty,
    html::toc::generate_toc,
    page::Page,
    parsing::{highlighting::highlight_html_blocks, minify::minify_html},
    utils::slugify::add_heading_ids,
    vault::Vault,
};
use anyhow::{Result, anyhow};
use markdown::{CompileOptions, Constructs, Options, ParseOptions};

impl Page {
    pub fn render(&self, vault: &Vault) -> Result<String> {
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
            .unwrap_or_else(|| "Part of Knot documentation");
        let dynamic_desc = if self.description.is_some() {
            description.to_string()
        } else {
            format!("{} about {}", description, self.metadata.name)
        };

        let asset_depth = self.metadata.rel_html_path.matches('/').count();
        let depth = self.metadata.rel_html_path.matches('/').count() + 1;

        let asset_prefix = "../".repeat(asset_depth);
        let prefix = "../".repeat(depth);

        let cleaned_path = self.metadata.rel_html_path.replace(".html", ".md");
        let to_md = format!("{}md/{}", prefix, cleaned_path);

        let raw_docs = markdown::to_html_with_options(&self.text, &options)
            .map_err(|e| anyhow!("Error found during markdown parsing. Cause: {e}"))?;
        let docs = add_heading_ids(&raw_docs);
        let toc = generate_toc(&docs);

        let js = format!(
            "{}\n{}",
            vault.global_js.compile_cached()?,
            vault.global_js.compile_lazy(&asset_prefix)?,
        );
        let styling = format!(
            "{}\n{}",
            vault.global_css.compile_cached()?,
            vault.global_css.compile_lazy(&asset_prefix)?
        );

        let html = format!(
            r#"<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>{title}</title>
                <meta name="description" content="{dynamic_desc}">
                <link rel="icon" type="image/x-icon" href="./icon.svg">
                {styling}
                {js}
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
        );

        let html = minify_html(&highlight_html_blocks(&html));
        Ok(html)
    }
}
