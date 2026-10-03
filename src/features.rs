use anyhow::Result;
use inquire::MultiSelect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Features {
    pub search: bool,
    pub next_previous_btns: bool,
    pub view_raw_md: bool,
    pub toc_sidebar: bool,
    /// Adds into footer 'Made with Vault' link
    pub support_vault: bool,
}

impl Features {
    pub fn new() -> Self {
        Self {
            search: true,
            next_previous_btns: true,
            view_raw_md: true,
            toc_sidebar: true,
            support_vault: false,
        }
    }

    pub fn from_cli() -> Result<Self> {
        let opt_search = "Search functionality";
        let opt_nav = "Next/Previous page buttons";
        let opt_toc = "Table of Contents (TOC) sidebar";
        let opt_raw = "View raw Markdown button";
        let opt_vault_support = "Add 'Made with Vault' into footer";

        let options = vec![opt_search, opt_nav, opt_toc, opt_raw, opt_vault_support];
        let selected = MultiSelect::new("Which features do you want inside your Vault?", options)
            .with_default(&[0, 1, 2, 3])
            .prompt()?;

        Ok(Self {
            search: selected.contains(&opt_search),
            next_previous_btns: selected.contains(&opt_nav),
            toc_sidebar: selected.contains(&opt_toc),
            view_raw_md: selected.contains(&opt_raw),
            support_vault: selected.contains(&opt_vault_support),
        })
    }
}

impl Default for Features {
    fn default() -> Self {
        Self::new()
    }
}
