use crate::html::{
    general::searching::{SEARCH_BOX, SEARCH_ICON},
    sidebar::SIDEBAR_ICON,
};

pub fn generate_navbar(title: &str) -> String {
    format!(
        r#"
        <div class="back-link nav-bar">
            {SIDEBAR_ICON}
            <a href="%__HOME_HREF__%index.html" class="home-btn" aria-label="Home">{title}</a>
            {SEARCH_ICON}
            {SEARCH_BOX}
        </div>
        "#
    )
}
