use anyhow::Result;
use inquire::MultiSelect;

pub struct Features {
    pub search: bool,
    pub next_previous_btns: bool,
    pub view_raw_md: bool,
    pub toc_sidebar: bool,
}

impl Features {
    pub fn new() -> Self {
        Self {
            search: true,
            next_previous_btns: true,
            view_raw_md: true,
            toc_sidebar: true,
        }
    }

    pub fn from_cli() -> Result<Self> {
        let opt_search = "Search functionality";
        let opt_nav = "Next/Previous page buttons";
        let opt_toc = "Table of Contents (TOC) sidebar";
        let opt_raw = "View raw Markdown button";

        let options = vec![opt_search, opt_nav, opt_toc, opt_raw];
        let selected = MultiSelect::new("Which features do you want inside your Vault?", options)
            .with_all_selected_by_default()
            .prompt()?;

        Ok(Self {
            search: selected.contains(&opt_search),
            next_previous_btns: selected.contains(&opt_nav),
            toc_sidebar: selected.contains(&opt_toc),
            view_raw_md: selected.contains(&opt_raw),
        })
    }
}

impl Default for Features {
    fn default() -> Self {
        Self::new()
    }
}
