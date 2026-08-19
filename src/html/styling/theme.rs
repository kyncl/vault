use colored::*;
use inquire::{Select, Text, validator::Validation};
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq)]
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
    Custom(String),
}

impl Display for ColorTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = format!("{:?}", self);
        let colored_name = match self {
            ColorTheme::Crimson => name.truecolor(251, 44, 54),
            ColorTheme::OrangeRed => name.truecolor(245, 73, 0),
            ColorTheme::Orange => name.truecolor(225, 113, 0),
            ColorTheme::Yellow => name.truecolor(208, 135, 0),
            ColorTheme::Green => name.truecolor(94, 165, 0),
            ColorTheme::Emerald => name.truecolor(0, 166, 62),
            ColorTheme::Teal => name.truecolor(0, 153, 102),
            ColorTheme::Cyan => name.truecolor(0, 150, 137),
            ColorTheme::Blue => name.truecolor(0, 146, 184),
            ColorTheme::Indigo => name.truecolor(113, 91, 255),
            ColorTheme::Purple => name.truecolor(110, 17, 176),
            ColorTheme::Rose => name.truecolor(236, 0, 63),
            ColorTheme::Silver => name.truecolor(249, 250, 251),
            ColorTheme::Fuchsia => name.truecolor(200, 0, 222),
            ColorTheme::Pink => name.truecolor(230, 0, 118),
            ColorTheme::Amber => name.truecolor(151, 60, 0),
            ColorTheme::Lime => name.truecolor(124, 207, 0),
            ColorTheme::Red => name.truecolor(159, 7, 18),
            ColorTheme::Sky => name.truecolor(0, 89, 138),
            ColorTheme::Violet => name.truecolor(93, 14, 192),
            ColorTheme::Custom(hex) => {
                if hex.is_empty() {
                    "Custom HEX".underline()
                } else {
                    format!("Custom ({hex})").underline()
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
