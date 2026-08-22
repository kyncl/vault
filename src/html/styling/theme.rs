use colored::*;
use inquire::{Select, Text, validator::Validation};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorTheme {
    Crimson,
    OrangeRed,
    Orange,
    Yellow,
    Green,
    Emerald,
    Teal,
    Cyan,
    Blue,
    Indigo,
    Purple,
    Rose,
    Silver,
    Fuchsia,
    Pink,
    Amber,
    Lime,
    Red,
    Sky,
    Violet,
    Rainbow,
    Custom(String),
}

pub fn parse_color_theme(s: &str) -> Result<ColorTheme, String> {
    match s.to_lowercase().as_str() {
        "crimson" => Ok(ColorTheme::Crimson),
        "orangered" => Ok(ColorTheme::OrangeRed),
        "orange" => Ok(ColorTheme::Orange),
        "yellow" => Ok(ColorTheme::Yellow),
        "green" => Ok(ColorTheme::Green),
        "emerald" => Ok(ColorTheme::Emerald),
        "teal" => Ok(ColorTheme::Teal),
        "cyan" => Ok(ColorTheme::Cyan),
        "blue" => Ok(ColorTheme::Blue),
        "indigo" => Ok(ColorTheme::Indigo),
        "purple" => Ok(ColorTheme::Purple),
        "rose" => Ok(ColorTheme::Rose),
        "silver" => Ok(ColorTheme::Silver),
        "fuchsia" => Ok(ColorTheme::Fuchsia),
        "pink" => Ok(ColorTheme::Pink),
        "amber" => Ok(ColorTheme::Amber),
        "lime" => Ok(ColorTheme::Lime),
        "red" => Ok(ColorTheme::Red),
        "sky" => Ok(ColorTheme::Sky),
        "violet" => Ok(ColorTheme::Violet),
        "rainbow" => Ok(ColorTheme::Rainbow),
        custom => {
            let cleaned = custom.strip_prefix('#').unwrap_or(custom);
            let is_valid_length = cleaned.len() == 3 || cleaned.len() == 6;
            let is_valid_chars = cleaned.chars().all(|c| c.is_ascii_hexdigit());

            if is_valid_length && is_valid_chars {
                if custom.starts_with('#') {
                    Ok(ColorTheme::Custom(custom.to_string()))
                } else {
                    Ok(ColorTheme::Custom(format!("#{custom}")))
                }
            } else {
                Err("Invalid HEX code".to_string())
            }
        }
    }
}

impl Display for ColorTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = format!("{:?}", self);
        let colored_name = match self {
            ColorTheme::Crimson => name.truecolor(251, 44, 54).to_string(),
            ColorTheme::OrangeRed => name.truecolor(245, 73, 0).to_string(),
            ColorTheme::Orange => name.truecolor(225, 113, 0).to_string(),
            ColorTheme::Yellow => name.truecolor(208, 135, 0).to_string(),
            ColorTheme::Green => name.truecolor(94, 165, 0).to_string(),
            ColorTheme::Emerald => name.truecolor(0, 166, 62).to_string(),
            ColorTheme::Teal => name.truecolor(0, 153, 102).to_string(),
            ColorTheme::Cyan => name.truecolor(0, 150, 137).to_string(),
            ColorTheme::Blue => name.truecolor(0, 146, 184).to_string(),
            ColorTheme::Indigo => name.truecolor(113, 91, 255).to_string(),
            ColorTheme::Purple => name.truecolor(110, 17, 176).to_string(),
            ColorTheme::Rose => name.truecolor(236, 0, 63).to_string(),
            ColorTheme::Silver => name.truecolor(249, 250, 251).to_string(),
            ColorTheme::Fuchsia => name.truecolor(200, 0, 222).to_string(),
            ColorTheme::Pink => name.truecolor(230, 0, 118).to_string(),
            ColorTheme::Amber => name.truecolor(151, 60, 0).to_string(),
            ColorTheme::Lime => name.truecolor(124, 207, 0).to_string(),
            ColorTheme::Red => name.truecolor(159, 7, 18).to_string(),
            ColorTheme::Sky => name.truecolor(0, 89, 138).to_string(),
            ColorTheme::Violet => name.truecolor(93, 14, 192).to_string(),
            ColorTheme::Rainbow => {
                let rainbow_text: String = name
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        let s = c.to_string();
                        match i % 6 {
                            0 => s.red().to_string(),
                            1 => s.yellow().to_string(),
                            2 => s.green().to_string(),
                            3 => s.cyan().to_string(),
                            4 => s.blue().to_string(),
                            _ => s.purple().to_string(),
                        }
                    })
                    .collect();
                rainbow_text
            }
            ColorTheme::Custom(hex) => {
                if hex.is_empty() {
                    "Custom HEX".underline().to_string()
                } else {
                    format!("Custom ({hex})").underline().to_string()
                }
            }
        };

        write!(f, "{}", colored_name)
    }
}

impl ColorTheme {
    pub fn all() -> Vec<ColorTheme> {
        vec![
            ColorTheme::Crimson,
            ColorTheme::OrangeRed,
            ColorTheme::Orange,
            ColorTheme::Yellow,
            ColorTheme::Green,
            ColorTheme::Emerald,
            ColorTheme::Teal,
            ColorTheme::Cyan,
            ColorTheme::Blue,
            ColorTheme::Indigo,
            ColorTheme::Purple,
            ColorTheme::Rose,
            ColorTheme::Silver,
            ColorTheme::Fuchsia,
            ColorTheme::Pink,
            ColorTheme::Amber,
            ColorTheme::Lime,
            ColorTheme::Red,
            ColorTheme::Sky,
            ColorTheme::Violet,
            ColorTheme::Rainbow,
            ColorTheme::Custom(String::new()),
        ]
    }

    pub fn as_str(&self) -> &str {
        match self {
            ColorTheme::Crimson => "crimson",
            ColorTheme::OrangeRed => "orange-red",
            ColorTheme::Orange => "orange",
            ColorTheme::Yellow => "yellow",
            ColorTheme::Green => "green",
            ColorTheme::Emerald => "emerald",
            ColorTheme::Teal => "teal",
            ColorTheme::Cyan => "cyan",
            ColorTheme::Blue => "blue",
            ColorTheme::Indigo => "indigo",
            ColorTheme::Purple => "purple",
            ColorTheme::Rose => "rose",
            ColorTheme::Silver => "silver",
            ColorTheme::Fuchsia => "fuchsia",
            ColorTheme::Pink => "pink",
            ColorTheme::Amber => "amber",
            ColorTheme::Lime => "lime",
            ColorTheme::Red => "red",
            ColorTheme::Sky => "sky",
            ColorTheme::Violet => "violet",
            ColorTheme::Rainbow => "rainbow",
            Self::Custom(_) => "custom",
        }
    }
}

pub fn prompt_color_selection() -> Option<ColorTheme> {
    let options = ColorTheme::all();
    let ans = Select::new("Select your primary theme color:", options)
        .with_page_size(10)
        .prompt();

    match ans {
        Ok(choice) => match choice {
            ColorTheme::Custom(_) => {
                let hex_validator = |input: &str| {
                    let cleaned = input.strip_prefix('#').unwrap_or(input);
                    let is_valid_length = cleaned.len() == 3 || cleaned.len() == 6;
                    let is_valid_chars = cleaned.chars().all(|c| c.is_ascii_hexdigit());

                    if is_valid_length && is_valid_chars {
                        Ok(Validation::Valid)
                    } else {
                        Ok(Validation::Invalid(
                            "Please enter a valid 3 or 6-digit hex code (e.g., #ff5733 or ff5733)"
                                .into(),
                        ))
                    }
                };

                match Text::new("Enter your custom HEX color (e.g., #ff5733 or ff5733):")
                    .with_validator(hex_validator)
                    .prompt()
                {
                    Ok(input) => {
                        let hex = if input.starts_with('#') {
                            input
                        } else {
                            format!("#{}", input)
                        };
                        Some(ColorTheme::Custom(hex))
                    }
                    Err(_) => None,
                }
            }
            other => Some(other),
        },
        Err(_) => {
            println!("No color selected.");
            None
        }
    }
}
