use crate::html::sidebar::SidebarSection;

pub fn generate_sidebar_items(sections: &[SidebarSection], prefix: &str) -> String {
    let mut sections_html = String::new();

    for section in sections {
        match &section.title {
            Some(title) => {
                let index_page = section
                    .items
                    .iter()
                    .find(|meta| meta.name.to_lowercase() == "index");
                let title_html = match index_page {
                    Some(meta) => {
                        let href = format!("{}{}", prefix, meta.rel_html_path);
                        format!(
                            r#"<a href="{}" class="sidebar-title-link">{}</a>"#,
                            href, title
                        )
                    }
                    None => title.clone(),
                };

                sections_html.push_str(&format!(
                    r#"<div class="sidebar-section">
                            <div class="sidebar-title">{}</div>
                            <ul class="sidebar-list">"#,
                    title_html
                ));

                for meta in &section.items {
                    if meta.name.to_lowercase() == "index" {
                        continue;
                    }

                    let href = format!("{}{}", prefix, meta.rel_html_path);
                    sections_html.push_str(&format!(
                        r#"<li class="sidebar-item"><a href="{}" class="sidebar-link">{}</a></li>"#,
                        href, meta.name
                    ));
                }
                sections_html.push_str("</ul></div>");
            }
            None => {
                sections_html.push_str(r#"<ul class="sidebar-list">"#);
                for meta in &section.items {
                    if meta.name.to_lowercase() == "index" {
                        continue;
                    }

                    let href = format!("{}{}", prefix, meta.rel_html_path);
                    sections_html.push_str(&format!(
                        r#"<li class="sidebar-item"><a href="{}" class="sidebar-link">{}</a></li>"#,
                        href, meta.name
                    ));
                }
                sections_html.push_str("</ul>");
            }
        }
    }

    sections_html
}
