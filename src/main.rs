use anyhow::Result;
use clap::Parser;
use inquire::Confirm;
use std::{
    fs::{self},
    path::PathBuf,
};
use vault::{
    cli::{Flags, VaultArgs},
    config::Configuration,
    modes::{
        init::{init_config, make_order_conf},
        parse::parse_docs,
    },
    utils::config_path::resolve_config_paths,
};

fn main() -> Result<()> {
    let args = VaultArgs::parse();

    match args.modes {
        vault::cli::Modes::Init { flags } => {
            let (config, config_dir) = init_config(flags)?;
            if Confirm::new("Do you want to parse the docs?")
                .with_default(true)
                .prompt()?
            {
                parse_docs(config, config_dir)?;
            }
        }
        vault::cli::Modes::Parse { flags } => {
            let (config, config_dir) = resolve_config(flags)?;
            parse_docs(config, config_dir)?;
        }
        vault::cli::Modes::DefaultOrder { flags } => {
            let (config, config_dir) = resolve_config(flags)?;
            make_order_conf(&config, &config_dir)?;
            if Confirm::new("Do you want to parse the docs?")
                .with_default(true)
                .prompt()?
            {
                parse_docs(config, config_dir)?;
            }
        }
    }

    Ok(())
}

fn resolve_config(flags: Flags) -> Result<(Configuration, PathBuf)> {
    let (config, config_dir) =
        if let Ok((config_file, config_dir)) = resolve_config_paths(Some(&flags.config_path)) {
            let mut config = Configuration::from_string(fs::read_to_string(config_file)?)?;
            if let Some(title) = flags.title {
                config.title = title;
            }
            // This is pretty much only useful, when user has in config false values in
            // features. Because not passing the flags is taken as false. This can mean user doesn't
            // want the property or they're just lazy to pass the flag. Clap doesn't support
            // something, like `--search=false`.
            if let Some(features) = flags.features {
                if features.search {
                    config.features.search = features.search;
                }
                if features.view_raw_md {
                    config.features.view_raw_md = features.view_raw_md;
                }
                if features.toc_sidebar {
                    config.features.toc_sidebar = features.toc_sidebar;
                }
                if features.next_previous_btns {
                    config.features.next_previous_btns = features.next_previous_btns;
                }
            }
            if let Some(style) = &flags.style {
                if let Some(ref radius) = style.radius {
                    config.styling.radius = *radius;
                }
                if let Some(ref theme_col) = style.main_col {
                    config.styling.main_col = theme_col.clone();
                }
                if let Some(ref bg_col) = style.bg_col {
                    config.styling.background_col = *bg_col;
                }
            }
            (config, config_dir)
        } else {
            init_config(flags)?
        };
    Ok((config, config_dir))
}
