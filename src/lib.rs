pub mod cli;
pub mod config;
pub mod features;
pub mod global_props;
pub mod html;
pub mod ignore;
pub mod modes;
pub mod page;
pub mod page_ordering;
pub mod parsing;
pub mod utils;
pub mod vault;
pub mod vault_pages;

pub const CONFIGURATION_FOLDER: &str = ".vault";
pub const DEFAULT_VAULT_CONFIGS: &str = "docs/.vault";
pub const IGNORE_FILE: &str = "vault-ignore";
pub const CONFIG_FILE: &str = "config.toml";
pub const ORDERING_FILE: &str = "order";
