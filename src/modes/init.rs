use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow};
use inquire::{Confirm, Text};

use crate::{
    CONFIG_FILE, CONFIGURATION_FOLDER, IGNORE_FILE, ORDERING_FILE,
    cli::Flags,
    config::Configuration,
    features::Features,
    html::styling::{
        Style, background::prompt_background_selection, border_radius::prompt_radius_selection,
        theme::prompt_color_selection,
    },
    ignore::make_git_ignore,
    utils::{autocomplete::path::FilePathCompleter, crawler::collect_files},
    vault::Vault,
};

pub fn init_config(flags: Flags) -> Result<(Configuration, PathBuf)> {
    let md_root = if !flags.md_path.exists() {
        if Confirm::new(&format!(
            "Hmm, it seems {} doesn't exist. Do you want to use it for storing your markdowns?",
            flags.md_path.display()
        ))
        .with_default(true)
        .prompt()?
        {
            fs::create_dir_all(&flags.md_path)?;
            flags.md_path.clone()
        } else {
            PathBuf::from(
                Text::new("Please insert path to markdown folder")
                    .with_autocomplete(FilePathCompleter::new(false))
                    .prompt()?,
            )
        }
    } else {
        flags.md_path.clone()
    };

    let html_root = if !flags.html_path.exists() {
        if Confirm::new(&format!(
            "Hmm, it seems {} doesn't exist. Do you want to use it for storing your HTML, CSS and JS?",
            flags.html_path.display()
        ))
        .with_default(true)
        .prompt()?
        {
            fs::create_dir_all(&flags.html_path)?;
            flags.html_path.clone()
        } else {
            PathBuf::from(
                Text::new("Please insert path to HTML folder")
                    .with_autocomplete(FilePathCompleter::new(false))
                    .prompt()?,
            )
        }
    } else {
        flags.html_path.clone()
    };

    let config_dir = if !flags.config_path.exists() {
        if Confirm::new(&format!(
            "Hmm, it seems {} doesn't exist. Do you still to use it for storing your configuration files?",
            flags.config_path.display()
        ))
        .with_default(true)
        .prompt()?
        {
            fs::create_dir_all(&flags.config_path)?;
            flags.config_path.clone()
        } else {
            PathBuf::from(
                Text::new("Please insert path to configuration folder")
                    .with_autocomplete(FilePathCompleter::new(false))
                    .prompt()?,
            )
        }
    } else {
        flags.config_path.clone()
    };

    let features = if let Some(ref feat_args) = flags.features {
        Features {
            search: feat_args.search,
            next_previous_btns: feat_args.next_previous_btns,
            view_raw_md: feat_args.view_raw_md,
            toc_sidebar: feat_args.toc_sidebar,
        }
    } else {
        Features::from_cli()?
    };

    let style = if let Some(style_args) = flags.style {
        let main_col = if let Some(main_col) = style_args.main_col {
            main_col
        } else {
            prompt_color_selection().ok_or(anyhow!("Couldn't find color theme"))?
        };
        let background_col = if let Some(bg_col) = style_args.bg_col {
            bg_col
        } else {
            prompt_background_selection(&main_col).ok_or(anyhow!("Couldn't find background"))?
        };
        let radius = if let Some(rad) = style_args.radius {
            rad
        } else {
            prompt_radius_selection().ok_or(anyhow!("Couldn't find radius"))?
        };
        Style::new(main_col, background_col, radius)
    } else {
        Style::from_cli()?
    };

    let title = if let Some(ref title) = flags.title {
        title.to_string()
    } else {
        Text::new("Title of your vault")
            .with_placeholder("Title")
            .prompt()?
    };

    let config = Configuration::new(&md_root, html_root, features, style, title);

    let config_folder = md_root
        .parent()
        .unwrap_or(Path::new("."))
        .join(CONFIGURATION_FOLDER);
    let config_path = config_folder.join(CONFIG_FILE);

    let msg = format!(
        "Do you want to save configuration into {}",
        config_folder.display()
    );
    if Confirm::new(&msg).with_default(true).prompt()? {
        fs::create_dir_all(&config_folder)?;
        fs::write(&config_path, config.to_toml()?)?;
        if !fs::exists(config_folder.join(IGNORE_FILE))? {
            fs::File::create(config_folder.join(IGNORE_FILE))?;
        }
        make_order_conf(&config, &config_folder)?;
    }

    Ok((config, config_dir))
}

pub fn make_order_conf(config: &Configuration, config_dir: &Path) -> Result<()> {
    if fs::exists(config_dir.join(ORDERING_FILE))?
        && !fs::read_to_string(config_dir.join(ORDERING_FILE))?.is_empty()
        && !Confirm::new("Do you want to update your order file?")
            .with_default(true)
            .prompt()?
    {
        return Ok(());
    }

    let mut patterns = if let Ok(data) = fs::read_to_string(config_dir.join(IGNORE_FILE)) {
        data.lines()
            .filter_map(|l| {
                if !l.starts_with("#") && !l.is_empty() {
                    Some(l.to_string())
                } else {
                    None
                }
            })
            .collect()
    } else {
        vec![]
    };
    patterns.push(CONFIGURATION_FOLDER.to_string());
    let ignorer = make_git_ignore(
        config_dir.parent().unwrap_or(&PathBuf::from(".")),
        &patterns,
    )?;
    let mut md_files = Vec::new();
    collect_files(
        &config.md_path,
        &config.md_path,
        &mut md_files,
        "md",
        &ignorer,
    )?;
    let mut vault = Vault::new();
    vault
        .set_pages(md_files, &config.md_path, &config.html_path)?
        .sort_pages(None)
        .set_neighbors()
        .set_sidebar_sections();
    fs::write(
        config_dir.join(ORDERING_FILE),
        vault.generate_static_manifest(),
    )?;
    Ok(())
}
