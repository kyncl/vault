use colored::Colorize;
use inquire::Select;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundTheme {
    Standard,
    Comfy,
    DeepBlack,
    Zen,
}

impl BackgroundTheme {
    pub fn all() -> Vec<BackgroundTheme> {
        vec![
            BackgroundTheme::Standard,
            BackgroundTheme::Comfy,
            BackgroundTheme::DeepBlack,
            BackgroundTheme::Zen,
        ]
    }
    pub fn to_string(&self) -> String {
        match self {
            BackgroundTheme::Standard => "standard",
            BackgroundTheme::Comfy => "comfy",
            BackgroundTheme::DeepBlack => "deep-black",
            BackgroundTheme::Zen => "zen",
        }
        .to_string()
    }
}

pub struct ColoredBackground(BackgroundTheme);
impl fmt::Display for ColoredBackground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted = match self.0 {
            BackgroundTheme::Standard => " Standard   (Light: #fff | Dark: #121212) "
                .white()
                .on_truecolor(18, 18, 18),
            BackgroundTheme::Comfy => " Comfy      (Light: #f4f | Dark: #18181b) "
                .white()
                .on_truecolor(24, 24, 27),
            BackgroundTheme::DeepBlack => " Deep Black (Light: #fff | Dark: #000000) "
                .white()
                .on_truecolor(0, 0, 0),
            BackgroundTheme::Zen => " Zen Dark   (Light: #fff | Dark: #242424) "
                .white()
                .on_truecolor(36, 36, 36),
        };

        write!(f, "{}", formatted)
    }
}

pub fn prompt_background_selection() -> Option<BackgroundTheme> {
    let options: Vec<ColoredBackground> = BackgroundTheme::all()
        .into_iter()
        .map(ColoredBackground)
        .collect();

    let ans = Select::new("Select your background profile", options).prompt();

    match ans {
        Ok(choice) => Some(choice.0),
        Err(_) => {
            println!("No background selected.");
            None
        }
    }
}
