use crossterm::style::Color;

use crate::{
    lexer::Lexer,
    new_slide,
    parser::{
        presentation::{Hunk, Presentation, Slide},
        token::TokenType,
    },
};

use self::token::Token;

pub mod presentation;
pub mod token;

macro_rules! hunk_function {
    ($fn_name:ident, $color_name:literal) => {
        fn $fn_name(&self, content: String) -> Option<Hunk> {
            let (bg_color, fg_color) = self.get_colors($color_name);
            Some(Hunk {
                content,
                bg_color,
                fg_color,
            })
        }
    };
}

pub struct Parser {
    content: String,
    config: toml::Table,
}

impl Parser {
    #[must_use]
    pub const fn new(content: String, config: toml::Table) -> Self {
        Self { content, config }
    }

    pub fn parse(&mut self) -> Presentation {
        let tokens = Lexer::new(self.content.clone()).tokenize();
        let mut slides: Vec<Slide> = Vec::new();
        slides.push(new_slide!());
        let mut current = 0;
        let mut first = true;
        for token in tokens {
            println!("{token:#?}");
            if token.ttype == TokenType::H1 {
                if !first {
                    current += 1;
                    slides.push(new_slide!());
                } else {
                    first = false;
                }
            }
            let hunks = &mut slides[current].content;
            hunks.push(self.hunk(token))
        }
        Presentation::new(slides)
    }

    fn hunk(&self, token: Token) -> Hunk {
        match token.ttype {
            TokenType::H1 => self.h1(token.content).unwrap(),
            TokenType::H2 => self.h2(token.content).unwrap(),
            TokenType::H3 => self.h3(token.content).unwrap(),
            TokenType::H4 => self.h4(token.content).unwrap(),
            TokenType::H5 => self.h5(token.content).unwrap(),
            TokenType::H6 => self.h6(token.content).unwrap(),
            TokenType::Text => self.text(token.content).unwrap(),
        }
    }

    fn match_color(&self, str: &str) -> Color {
        match str {
            "red" => Color::Red,
            "darkred" => Color::DarkRed,
            "yellow" => Color::Yellow,
            "darkyellow" => Color::DarkYellow,
            "blue" => Color::Blue,
            "darkblue" => Color::DarkBlue,
            "green" => Color::Green,
            "darkgreen" => Color::DarkGreen,
            "grey" => Color::Grey,
            "darkgrey" => Color::DarkGrey,
            "cyan" => Color::Cyan,
            "darkcyan" => Color::DarkCyan,
            "magenta" => Color::Magenta,
            "darkmagenta" => Color::DarkMagenta,
            "white" => Color::White,
            "black" => Color::Black,
            _ => {
                eprintln!("It isn't a color {}", str);
                Color::White
            }
        }
    }

    fn get_colors(&self, label: &str) -> (Color, Color) {
        macro_rules! get {
            ($map:expr, $value:ident, $key:ident, $code:block) => {
                if let Some(toml::Value::Table($value)) = $map.get($key) {
                    $code
                }
            };
        }
        let lab = "theme";
        get!(self.config, theme, lab, {
            println!("{:#?}", theme);
            get!(theme, colors, label, {
                let bg_color = if let Some(bg_color) = colors.get("bg") {
                    self.match_color(bg_color.as_str().unwrap())
                } else {
                    Color::Black
                };
                let fg_color = if let Some(fg_color) = colors.get("fg") {
                    self.match_color(fg_color.as_str().unwrap())
                } else {
                    Color::White
                };
                return (bg_color, fg_color);
            })
        });
        (Color::Black, Color::White)
    }

    hunk_function!(h1, "h1");
    hunk_function!(h2, "h2");
    hunk_function!(h3, "h3");
    hunk_function!(h4, "h4");
    hunk_function!(h5, "h5");
    hunk_function!(h6, "h6");
    hunk_function!(text, "text");
}
