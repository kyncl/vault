use crate::page::PageMetadata;

pub const SIDEBAR_ICON: &str = r#"
<button type="button" class="menu-toggle" id="sidebarToggle" aria-label="Toggle Sidebar">
    <span class="icon-line"></span>
    <span class="icon-line"></span>
    <span class="icon-line"></span>
</button>
"#;

pub fn generate_sidebar() -> String {
    format!(
        r#"
        <div class="sidebar-backdrop" id="sidebarBackdrop"></div>
        <aside class="sidebar">
            %__SIDEBAR_SECTIONS__%
        </aside>
        "#
    )
}

pub struct SidebarSection {
    pub title: Option<String>,
    pub items: Vec<PageMetadata>,
}
