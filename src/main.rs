use anyhow::Result;
use std::{
    fs::{self, create_dir_all},
    io::Write,
    path::Path,
};
use vault::{
    global_props::{GlobalProperty, css::GlobalCSS, js::GlobalJS},
    html::generate_global_elem,
    utils::crawler::collect_files,
    vault::Vault,
};

fn main() -> Result<()> {
    let md_root = Path::new("./docs/md");
    let html_root = Path::new("./docs/html");

    let mut g_css = GlobalCSS::new();
    let mut g_js = GlobalJS::new();
    g_css.set_global_property(html_root.join("css"), "css")?;
    g_js.set_global_property(html_root.join("js"), "js")?;

    let mut md_files = Vec::new();
    collect_files(md_root, &mut md_files, "md")?;

    let mut vault = Vault::new();
    vault
        .global_js(g_js)
        .global_css(g_css)
        .inside_main_elem("")
        .global_elem(generate_global_elem("Title"))
        .set_pages(md_files, md_root, html_root)?
        .sort_pages()
        .set_neighbors()
        .set_sidebar_sections()
        .render_all(html_root)?;

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

    Ok(())
}
