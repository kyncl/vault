use crate::{
    CONFIGURATION_FOLDER, IGNORE_FILE,
    config::Configuration,
    global_props::{GlobalProperty, css::GlobalCSS, font::GlobalFonts, js::GlobalJS},
    html::generate_global_elem,
    ignore::make_git_ignore,
    utils::crawler::collect_files,
    vault::Vault,
};
use anyhow::Result;
use colored::*;
use std::{
    fs::{self, create_dir_all},
    io::Write,
    path::{Path, PathBuf},
};

/// Reads the docs folder and create based on configuration and files the documentation
pub fn parse_docs<P: AsRef<Path>>(config: Configuration, config_dir: P) -> Result<()> {
    let config_dir = config_dir.as_ref();

    let mut patterns = if let Ok(data) = fs::read_to_string(config_dir.join(IGNORE_FILE)) {
        data.lines()
            .filter_map(|l| {
                if !l.starts_with("#") && !l.is_empty() {
                    Some(l.to_string())
                } else {
                    None
                }
            })
            .collect()
    } else {
        vec![]
    };
    patterns.push(CONFIGURATION_FOLDER.to_string());
    let ignorer = make_git_ignore(
        config_dir.parent().unwrap_or(&PathBuf::from(".")),
        &patterns,
    )?;

    let mut css = GlobalCSS::new();
    let mut js = GlobalJS::new();
    let mut fonts = GlobalFonts::new();
    css.set_global_property(
        config.html_path.join("css"),
        &config.html_path,
        "css",
        &ignorer,
    )?;
    js.set_global_property(
        config.html_path.join("js"),
        &config.html_path,
        "js",
        &ignorer,
    )?;

    let fonts_dir = config.html_path.join("fonts");
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
    collect_files(
        &config.md_path,
        &config.md_path,
        &mut md_files,
        "md",
        &ignorer,
    )?;

    let mut vault = Vault::new();
    vault
        .global_js(js)
        .global_css(css)
        .global_fonts(fonts)
        // .inside_main_elem("")
        .global_elem(generate_global_elem(&config.title, config.features.search))
        .set_pages(md_files, &config.md_path, &config.html_path)?
        .sort_pages()
        .set_neighbors()
        .set_sidebar_sections()
        .render_all(&config.html_path, &config.styling, &config.features)?;

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

    if config.html_path.join("index.html").exists() {
        println!(
            "Parsing complete.\nYou can check the homepage on: {}",
            config
                .html_path
                .join("index.html")
                .display()
                .to_string()
                .underline()
        );
    } else {
        println!("Parsing complete.");
    }

    Ok(())
}
