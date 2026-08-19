use crate::html::{navbar::generate_navbar, sidebar::generate_sidebar};

pub mod general;
pub mod navbar;
pub mod sidebar;
pub mod sidebar_items;
pub mod styling;
pub mod toc;

pub fn generate_global_elem(title: &str, use_searching: bool) -> String {
    let mut nav = generate_navbar(title, use_searching);
    nav.push_str(&generate_sidebar());
    nav
}
