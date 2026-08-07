use anyhow::{Result, anyhow};

use crate::html::styling::{
    background::{BackgroundTheme, prompt_background_selection},
    border_radius::{RadiusTheme, prompt_radius_selection},
    theme::{ColorTheme, prompt_color_selection},
};

pub mod background;
pub mod border_radius;
pub mod theme;

pub struct Style {
    main_col: ColorTheme,
    background_col: BackgroundTheme,
    radius: RadiusTheme,
}
impl Style {
    pub fn new() -> Result<Self> {
        let main_col = prompt_color_selection().ok_or(anyhow!("Couldn't find color theme"))?;
        let background_col =
            prompt_background_selection().ok_or(anyhow!("Couldn't find background"))?;
        let radius = prompt_radius_selection().ok_or(anyhow!("Couldn't find radius"))?;

        Ok(Self {
            radius,
            main_col,
            background_col,
        })
    }

    pub fn make_header(&self) -> String {
        let theme = self.main_col.to_string();
        let background = self.background_col.to_string();
        let border_radius = self.radius.to_string();
        format!(
            r#" data-theme="{theme}" data-background-theme="{background}" data-radius-theme="{border_radius}" "#
        )
    }
}
