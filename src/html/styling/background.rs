use crate::html::styling::theme::ColorTheme;
use clap::ValueEnum;
use colored::Colorize;
use inquire::Select;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum BackgroundTheme {
    Standard,
    Comfy,
    DeepBlack,
    Zen,
    Stone,
    Mauve,
    Olive,
    Mist,
    Taupe,
}

impl BackgroundTheme {
    pub fn all() -> Vec<BackgroundTheme> {
        vec![
            BackgroundTheme::Standard,
            BackgroundTheme::Comfy,
            BackgroundTheme::DeepBlack,
            BackgroundTheme::Zen,
            BackgroundTheme::Stone,
            BackgroundTheme::Mauve,
            BackgroundTheme::Olive,
            BackgroundTheme::Mist,
            BackgroundTheme::Taupe,
        ]
    }

    pub fn as_str(&self) -> &str {
        match self {
            BackgroundTheme::Standard => "standard",
            BackgroundTheme::Comfy => "comfy",
            BackgroundTheme::DeepBlack => "deep-black",
            BackgroundTheme::Zen => "zen",
            BackgroundTheme::Stone => "stone",
            BackgroundTheme::Mauve => "mauve",
            BackgroundTheme::Olive => "olive",
            BackgroundTheme::Mist => "mist",
            BackgroundTheme::Taupe => "taupe",
        }
    }
}

pub struct ColoredBackground {
    theme: BackgroundTheme,
    recommendate: bool,
}

impl fmt::Display for ColoredBackground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (name, bg_rgb) = match self.theme {
            BackgroundTheme::Standard => (" Standard    ", (18, 18, 18)),
            BackgroundTheme::Comfy => (" Comfy       ", (24, 24, 27)),
            BackgroundTheme::DeepBlack => (" Deep Black  ", (0, 0, 0)),
            BackgroundTheme::Zen => (" Zen         ", (36, 36, 36)),
            BackgroundTheme::Stone => (" Stone       ", (28, 25, 23)),
            BackgroundTheme::Mauve => (" Mauve       ", (29, 22, 30)),
            BackgroundTheme::Olive => (" Olive       ", (29, 29, 22)),
            BackgroundTheme::Mist => (" Mist        ", (22, 27, 29)),
            BackgroundTheme::Taupe => (" Taupe       ", (29, 24, 22)),
        };

        let mut display_text = format!("{} (Dark: {})", name, self.theme.as_str());

        if self.recommendate {
            display_text = format!("{} ➔ Recommended for your color", display_text);
        }

        let colored = display_text
            .white()
            .on_truecolor(bg_rgb.0, bg_rgb.1, bg_rgb.2);
        write!(f, "{}", colored)
    }
}

pub fn prompt_background_selection(selected_color: &ColorTheme) -> Option<BackgroundTheme> {
    let options: Vec<ColoredBackground> = BackgroundTheme::all()
        .into_iter()
        .map(|theme| {
            let is_recommended = matches!(
                (selected_color, theme),
                (
                    ColorTheme::Green | ColorTheme::Emerald | ColorTheme::Lime,
                    BackgroundTheme::Olive,
                ) | (
                    ColorTheme::Blue | ColorTheme::Cyan | ColorTheme::Teal | ColorTheme::Sky,
                    BackgroundTheme::Mist,
                ) | (
                    ColorTheme::Purple
                        | ColorTheme::Fuchsia
                        | ColorTheme::Pink
                        | ColorTheme::Indigo
                        | ColorTheme::Violet,
                    BackgroundTheme::Mauve,
                ) | (
                    ColorTheme::Crimson | ColorTheme::Rose | ColorTheme::Red,
                    BackgroundTheme::Taupe,
                ) | (
                    ColorTheme::Orange
                        | ColorTheme::OrangeRed
                        | ColorTheme::Yellow
                        | ColorTheme::Amber,
                    BackgroundTheme::Stone,
                )
            );

            ColoredBackground {
                theme,
                recommendate: is_recommended,
            }
        })
        .collect();

    let ans = Select::new("Select your background profile", options).prompt();

    match ans {
        Ok(choice) => Some(choice.theme),
        Err(_) => {
            println!("No background selected.");
            None
        }
    }
}
