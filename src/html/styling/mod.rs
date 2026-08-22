use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::html::styling::{
    background::{BackgroundTheme, prompt_background_selection},
    border_radius::{RadiusTheme, prompt_radius_selection},
    theme::{ColorTheme, prompt_color_selection},
};

pub mod background;
pub mod border_radius;
pub mod theme;

#[derive(Serialize, Deserialize)]
pub struct Style {
    pub main_col: ColorTheme,
    pub background_col: BackgroundTheme,
    pub radius: RadiusTheme,
}
impl Style {
    pub fn from_cli() -> Result<Self> {
        let main_col = prompt_color_selection().ok_or(anyhow!("Couldn't find color theme"))?;
        let background_col =
            prompt_background_selection(&main_col).ok_or(anyhow!("Couldn't find background"))?;
        let radius = prompt_radius_selection().ok_or(anyhow!("Couldn't find radius"))?;

        Ok(Self {
            radius,
            main_col,
            background_col,
        })
    }
    pub fn new(main_col: ColorTheme, background_col: BackgroundTheme, radius: RadiusTheme) -> Self {
        Self {
            radius,
            main_col,
            background_col,
        }
    }

    pub fn make_header(&self) -> String {
        let background = self.background_col.as_str();
        let border_radius = self.radius.as_str();

        match &self.main_col {
            ColorTheme::Custom(hex) => {
                format!(
                    r#" data-theme="custom" data-background-theme="{background}" data-radius-theme="{border_radius}" style="--main: {hex}; --main-alt: {hex}; --active-bg: {hex}26;" "#
                )
            }
            _ => {
                let theme = self.main_col.as_str();
                format!(
                    r#" data-theme="{theme}" data-background-theme="{background}" data-radius-theme="{border_radius}" "#
                )
            }
        }
    }
}
