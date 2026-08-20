use crate::html::sidebar::SidebarSection;

pub fn generate_sidebar_items(sections: &[SidebarSection], prefix: &str) -> String {
    let mut sections_html = String::new();

    for section in sections {
        let child_items: Vec<_> = section
            .items
            .iter()
            .filter(|meta| meta.name.to_lowercase() != "index")
            .collect();

        if let Some(title) = &section.title {
            let index_page = section
                .items
                .iter()
                .find(|meta| meta.name.to_lowercase() == "index");

            let title_link = match index_page {
                Some(meta) => {
                    let href = format!("{}{}", prefix, meta.rel_html_path);
                    format!(
                        r#"<a href="{}" class="sidebar-title-link">{}</a>"#,
                        href, title
                    )
                }
                None => format!(r#"<span class="sidebar-title-text">{}</span>"#, title),
            };

            // Render toggle button if child items exist
            let toggle_btn = if !child_items.is_empty() {
                r#"<button type="button" class="sidebar-toggle" aria-label="Toggle section">
                    <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m3 4.5 3 3 3-3"/></svg>
                </button>"#
            } else {
                ""
            };

            sections_html.push_str(&format!(
                r#"<div class="sidebar-section">
                    <div class="sidebar-title">
                        {toggle_btn}
                        {title_link}
                    </div>
                    <ul class="sidebar-list">"#,
            ));
        } else {
            sections_html.push_str(r#"<ul class="sidebar-list">"#);
        }

        for meta in child_items {
            let href = format!("{}{}", prefix, meta.rel_html_path);
            sections_html.push_str(&format!(
                r#"<li class="sidebar-item"><a href="{}" class="sidebar-link">{}</a></li>"#,
                href, meta.name
            ));
        }

        if section.title.is_some() {
            sections_html.push_str("</ul></div>");
        } else {
            sections_html.push_str("</ul>");
        }
    }

    sections_html
}
