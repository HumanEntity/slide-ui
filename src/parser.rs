use crossterm::style::Color;

use crate::{
    lexer::Lexer,
    new_slide,
    parser::{
        presentation::{Hunk, Presentation, Slide},
        token::TokenType,
    },
};

pub mod presentation;
pub mod token;

pub struct Parser {
    content: String,
}

impl Parser {
    #[must_use]
    pub const fn new(content: String) -> Self {
        Self { content }
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
            match token.ttype {
                TokenType::H1 => hunks.push(Hunk {
                    content: token.content,
                    bg_color: Color::Black,
                    fg_color: Color::Red,
                }),
                TokenType::H2 => hunks.push(Hunk {
                    content: token.content,
                    bg_color: Color::Black,
                    fg_color: Color::Green,
                }),
                TokenType::H3 => hunks.push(Hunk {
                    content: token.content,
                    bg_color: Color::Black,
                    fg_color: Color::Yellow,
                }),
                TokenType::H4 => hunks.push(Hunk {
                    content: token.content,
                    bg_color: Color::Black,
                    fg_color: Color::Blue,
                }),
                TokenType::H5 => hunks.push(Hunk {
                    content: token.content,
                    bg_color: Color::Black,
                    fg_color: Color::Magenta,
                }),
                TokenType::H6 => hunks.push(Hunk {
                    content: token.content,
                    bg_color: Color::Black,
                    fg_color: Color::Cyan,
                }),
                TokenType::Text => hunks.push(Hunk {
                    content: token.content,
                    bg_color: Color::Black,
                    fg_color: Color::White,
                }),
            }
        }
        Presentation::new(slides)
    }
}
