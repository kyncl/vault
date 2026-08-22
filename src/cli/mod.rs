use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

use crate::{
    DEFAULT_VAULT_CONFIGS,
    html::styling::{
        background::BackgroundTheme,
        border_radius::RadiusTheme,
        theme::{ColorTheme, parse_color_theme},
    },
};

#[derive(Parser)]
#[command(
    name = "Vault",
    author = "Kyncl",
    version,
    about = "Static site generator for parsing Markdowns into a cohesive HTML documentation vault"
)]
pub struct VaultArgs {
    #[command(subcommand)]
    pub modes: Modes,
}

#[derive(Args, Debug, Clone, PartialEq)]
pub struct FeatureArgs {
    /// Enable search functionality
    #[arg(short, long)]
    pub search: bool,

    /// Enable next and previous navigation buttons
    #[arg(short, long)]
    pub next_previous_btns: bool,

    /// Enable option to view raw markdown
    #[arg(short, long)]
    pub view_raw_md: bool,

    /// Enable table of contents sidebar
    #[arg(short, long)]
    pub toc_sidebar: bool,
}

#[derive(Args, Debug, Clone, PartialEq)]
pub struct StyleArgs {
    /// Main color theme
    ///
    /// You can choose from: Crimson, OrangeRed, Orange, Yellow, Green,
    /// Emerald, Teal, Cyan, Blue, Indigo, Purple, Rose,
    /// Silver, Fuchsia, Pink, Amber, Lime, Red,
    /// Sky, Violet, Rainbow and Custom HEX code
    #[arg(short='a', long, value_parser = parse_color_theme)]
    pub main_col: Option<ColorTheme>,

    /// Background color theme
    #[arg(short, long, value_enum)]
    pub bg_col: Option<BackgroundTheme>,

    /// Border radius theme
    #[arg(short, long, value_enum)]
    pub radius: Option<RadiusTheme>,
}

#[derive(Debug, Subcommand, PartialEq, Clone)]
pub enum Modes {
    /// Parses the docs
    Parse {
        #[command(flatten)]
        flags: Flags,
    },
    /// Create your configuration file
    Init {
        #[command(flatten)]
        flags: Flags,
    },
}

#[derive(Debug, Args, PartialEq, Clone)]
pub struct Flags {
    /// Path where your configuration files reside
    #[arg(short, long, default_value = DEFAULT_VAULT_CONFIGS)]
    pub config_path: PathBuf,

    /// Folder where your markdowns live
    #[arg(short, long, default_value = "./docs/md")]
    pub md_path: PathBuf,

    /// Folder where your HTML files and resources will live
    #[arg(short = 'p', long, default_value = "./docs/html")]
    pub html_path: PathBuf,

    /// Title of your project/app/product
    #[arg(short = 'i', long)]
    pub title: Option<String>,

    /// Optional feature flags group
    #[command(flatten)]
    pub features: Option<FeatureArgs>,

    /// Optional style settings group
    #[command(flatten)]
    pub style: Option<StyleArgs>,
}
