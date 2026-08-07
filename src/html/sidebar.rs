use crate::{page::PageMetadata, vault::Vault};

pub const SIDEBAR_ICON: &str = r#"
<button type="button" class="menu-toggle" id="sidebarToggle" aria-label="Toggle Sidebar">
    <span class="icon-line"></span>
    <span class="icon-line"></span>
    <span class="icon-line"></span>
</button>
"#;

pub fn generate_sidebar() -> String {
    r#"
        <div class="sidebar-backdrop" id="sidebarBackdrop"></div>
        <aside class="sidebar">
            %__SIDEBAR_SECTIONS__%
        </aside>
        "#
    .to_string()
}

pub struct SidebarSection {
    pub title: Option<String>,
    pub items: Vec<PageMetadata>,
}

impl Vault {
    /// Group metadata into sections
    pub fn set_sidebar_sections(&mut self) -> &mut Self {
        if self.pages.is_empty() {
            eprintln!("Pages are empty. Did you chain correctly?");
        }
        let mut sections: Vec<SidebarSection> = Vec::new();
        for page in &self.pages {
            if let Some(sec) = sections
                .iter_mut()
                .find(|s| s.title == page.metadata.category)
            {
                sec.items.push(page.metadata.clone());
            } else {
                sections.push(SidebarSection {
                    title: page.metadata.category.clone(),
                    items: vec![page.metadata.clone()],
                });
            }
        }
        self.sidebar_sections = sections;
        self
    }
}
