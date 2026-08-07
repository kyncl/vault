use crate::html::{navbar::generate_navbar, sidebar::generate_sidebar};

pub mod general;
pub mod navbar;
pub mod sidebar;
pub mod sidebar_items;
pub mod toc;

pub fn generate_global_elem(title: &str) -> String {
    let mut nav = generate_navbar(title);
    nav.push_str(&generate_sidebar());
    nav
}
