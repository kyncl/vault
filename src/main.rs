use anyhow::Result;
use clap::Parser;
use colored::*;
use std::{
    fs::{self, create_dir_all},
    io::Write,
    path::Path,
};
use vault::{
    cli::VaultArgs,
    global_props::{GlobalProperty, css::GlobalCSS, font::GlobalFonts, js::GlobalJS},
    html::{generate_global_elem, styling},
    utils::crawler::collect_files,
    vault::Vault,
};

fn main() -> Result<()> {
    let args = VaultArgs::parse();
    let md_root = Path::new(&args.md_path);
    let html_root = Path::new(&args.html_path);

    let mut css = GlobalCSS::new();
    let mut js = GlobalJS::new();
    let mut fonts = GlobalFonts::new();
    css.set_global_property(html_root.join("css"), "css")?;
    js.set_global_property(html_root.join("js"), "js")?;
    let style = styling::Style::new()?;

    let fonts_dir = html_root.join("fonts");
    if fonts_dir.exists() {
        for entry in std::fs::read_dir(fonts_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file()
                && let Some(ext) = path.extension().and_then(|e| e.to_str())
            {
                let ext_lower = ext.to_lowercase();
                if matches!(ext_lower.as_str(), "ttf" | "woff" | "woff2" | "otf") {
                    fonts.add_lazy(&path);
                }
            }
        }
    }

    let mut md_files = Vec::new();
    collect_files(md_root, &mut md_files, "md")?;

    let mut vault = Vault::new();
    vault
        .global_js(js)
        .global_css(css)
        .global_fonts(fonts)
        .inside_main_elem("")
        .global_elem(generate_global_elem(&args.title))
        .set_pages(md_files, md_root, html_root)?
        .sort_pages()
        .set_neighbors()
        .set_sidebar_sections()
        .render_all(html_root, &style)?;

    println!("Creating HTML files...");
    for data in &vault.pages {
        if let Some(html) = &data.html {
            if let Some(parent) = data.metadata.html_path.parent() {
                create_dir_all(parent)?;
            }
            let mut file = fs::File::create(&data.metadata.html_path)?;
            file.write_all(html.as_bytes())?;
            file.flush()?;
        } else {
            eprintln!(
                "Couldn't create {} because the content is empty",
                data.metadata.html_path.display()
            );
        }
    }

    if html_root.join("index.html").exists() {
        println!(
            "Compilation complete.\nYou can check the homepage on: {}",
            html_root
                .join("index.html")
                .display()
                .to_string()
                .underline()
        );
    } else {
        println!("Compilation complete.");
    }
    Ok(())
}
