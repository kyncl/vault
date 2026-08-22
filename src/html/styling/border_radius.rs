use clap::ValueEnum;
use colored::Colorize;
use inquire::Select;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum RadiusTheme {
    Standard,
    Brutalist,
    Rounded,
}

impl RadiusTheme {
    pub fn all() -> Vec<RadiusTheme> {
        vec![
            RadiusTheme::Standard,
            RadiusTheme::Brutalist,
            RadiusTheme::Rounded,
        ]
    }

    pub fn as_str(&self) -> &str {
        match self {
            RadiusTheme::Standard => "standard",
            RadiusTheme::Brutalist => "brutalist",
            RadiusTheme::Rounded => "rounded",
        }
    }
}

pub struct ColoredRadius(RadiusTheme);
impl fmt::Display for ColoredRadius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_string = match self.0 {
            RadiusTheme::Standard => " ╭───╮ Standard   (8px, 6px, 4px, 2px)",
            RadiusTheme::Brutalist => " ┌───┐ Brutalist  (0px - Sharp edges)",
            RadiusTheme::Rounded => "  ╭─╮  Rounded   (16px, 12px, 8px, 4px)",
        };
        write!(f, "{}", display_string.cyan())
    }
}

pub fn prompt_radius_selection() -> Option<RadiusTheme> {
    let options: Vec<ColoredRadius> = RadiusTheme::all().into_iter().map(ColoredRadius).collect();

    let ans = Select::new("Select your border radius style", options).prompt();

    match ans {
        Ok(choice) => Some(choice.0),
        Err(_) => {
            println!("No radius selected.");
            None
        }
    }
}
