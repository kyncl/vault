use colored::*;
use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Violet,
    Fuchsia,
    Pink,
}
impl Display for ColorTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = format!("{:?}", self);
        let colored_name = match self {
            ColorTheme::Crimson => name.truecolor(254, 36, 40),
            ColorTheme::OrangeRed => name.truecolor(240, 100, 70),
            ColorTheme::Orange => name.truecolor(255, 165, 0),
            ColorTheme::Yellow => name.truecolor(250, 200, 50),
            ColorTheme::Green => name.truecolor(100, 220, 130),
            ColorTheme::Emerald => name.truecolor(80, 200, 150),
            ColorTheme::Teal => name.truecolor(70, 180, 180),
            ColorTheme::Cyan => name.truecolor(0, 255, 255),
            ColorTheme::Blue => name.truecolor(100, 150, 255),
            ColorTheme::Indigo => name.truecolor(130, 100, 240),
            ColorTheme::Purple => name.truecolor(180, 100, 220),
            ColorTheme::Rose => name.truecolor(240, 120, 150),
            ColorTheme::Silver => name.truecolor(230, 230, 230),
            ColorTheme::Violet => name.truecolor(200, 130, 255),
            ColorTheme::Fuchsia => name.truecolor(255, 80, 200),
            ColorTheme::Pink => name.truecolor(255, 150, 180),
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
            ColorTheme::Violet,
            ColorTheme::Fuchsia,
            ColorTheme::Pink,
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
            ColorTheme::Violet => "violet",
            ColorTheme::Fuchsia => "fuchsia",
            ColorTheme::Pink => "pink",
        }
    }
}

pub fn prompt_color_selection() -> Option<ColorTheme> {
    let options: Vec<ColorTheme> = ColorTheme::all();
    let ans = inquire::Select::new("Select your primary theme color:", options)
        .with_page_size(10)
        .prompt();

    match ans {
        Ok(choice) => Some(choice),
        Err(_) => {
            println!("No color selected.");
            None
        }
    }
}
