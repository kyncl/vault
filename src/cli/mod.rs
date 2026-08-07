use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "Vault",
    author = "Kyncl",
    version,
    about = "Static site generator for parsing Markdown into a cohesive HTML documentation vault"
)]
pub struct VaultArgs {
    /// Folder, where your markdowns live
    #[arg(short, long, default_value = "./docs/md")]
    pub md_path: PathBuf,

    /// Folder, where will your HTML files live
    ///
    /// and it's resources
    #[arg(short = 'p', long, default_value = "./docs/html")]
    pub html_path: PathBuf,

    /// Title of your project/app/product
    #[arg(short, long)]
    pub title: String,
}
