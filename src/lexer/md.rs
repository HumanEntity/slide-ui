use crate::parser::token::{Token, TokenType};

use super::Lexer;

#[derive(Debug, Default, Clone)]
pub struct MdLexer {
    content: String,
    start: usize,
    current: usize,
    line: usize,

    heading_level: u8,
}

impl Lexer for MdLexer {
    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.current < self.content.len() - 1 {
            tokens.push(self.get_token());
        }
        tokens
    }

    fn set_src(&mut self, src: &str) {
        self.content = src.into();
    }
}

impl MdLexer {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            content: String::new(),
            start: 0,
            current: 0,
            line: 0,
            heading_level: 0,
        }
    }
    //  Tokens
    /*
     *  Possible tokens:
     *      H1
     *      H2
     *      H3
     *      H4
     *      H5
     *      H6
     *      Text
     *      Link
     */
    pub fn get_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        let c = self.advance_char();
        if c == '#' {
            let i = self.current;
            while self.peek_cnow() == '#' {
                self.advance_char();
            }
            if self.peek_cnow() == ' ' {
                self.current = i;
                return self.heading();
            }
            self.current = i;
        } else if c == '[' {
            return self.link();
        }

        while self.peek_cnow() != '#'
            && self.peek_cnow() != '['
            && self.peek_offset(0, 1) != '\n'
            && self.current < self.content.len()
        {
            self.advance_char();
        }
        self.make_token(TokenType::Text)

        // if self.current >= self.content.len() {
        //     return self.make_token(MdTokenType::Text);
        // }
    }
    pub fn heading(&mut self) -> Token {
        let mut level: u8 = 1;
        while self.peek_cnow() == '#' {
            level += 1;
            self.advance_char();
        }
        while self.peek_offset(0, 1) != '\n' {
            self.advance_char();
        }
        println!("level: {level}");
        self.heading_level = level;
        match level {
            1 => self.make_token(TokenType::H1),
            2 => self.make_token(TokenType::H2),
            3 => self.make_token(TokenType::H3),
            4 => self.make_token(TokenType::H4),
            5 => self.make_token(TokenType::H5),
            6 => self.make_token(TokenType::H6),
            _ => {
                eprintln!("Not valid heading in markdown (as I know it)");
                unreachable!()
            }
        }
    }
    pub fn link(&mut self) -> Token {
        while self.peek_cnow() != ']' {
            self.advance_char();
        }
        self.advance_char();
        if self.peek_cnow() == '(' {
            while self.peek_cnow() != ')' {
                self.advance_char();
            }
            self.advance_char();
        }
        self.make_token(TokenType::Link)
    }
    fn make_token(&self, ttype: TokenType) -> Token {
        let mut str = String::new();
        let start;
        let mut is_heading = 1;
        let mut is_text = 0;
        match ttype {
            TokenType::H1 => {
                start = 0;
                is_heading = 0
            }
            TokenType::H2 => start = 1,
            TokenType::H3 => start = 2,
            TokenType::H4 => start = 3,
            TokenType::H5 => start = 4,
            TokenType::H6 => start = 5,
            _ => {
                start = 0;
                is_heading = 0;
                is_text = 2;
            }
        }
        for _ in 0..self.heading_level + start + is_text - 1 - is_heading {
            str.push(' ');
        }
        for i in self.start + (start as usize)..self.current {
            str.push(self.content.chars().collect::<Vec<_>>()[i]);
        }

        Token::new(ttype, str)
    }
    pub fn advance_char(&mut self) -> char {
        self.current += 1;
        *self
            .content
            .chars()
            .collect::<Vec<_>>()
            .get(self.current - 1)
            .unwrap_or(&'\0')
    }
    /* pub fn peek_char(&self, offset: usize, counter_offset: usize) -> char {
    if self.current+offset-counter_offset == 0 || self.current+offset-counter_offset >= self.content.chars().collect::<Vec<_>>().len(){
    return self.content.chars().collect::<Vec<_>>()[0];
    }
    self.content.chars().collect::<Vec<_>>()[self.current+offset-counter_offset]
    }*/
    #[must_use]
    pub fn peek_cnow(&self) -> char {
        if self.current >= self.content.chars().count() {
            '\0'
        } else {
            self.content.chars().collect::<Vec<_>>()[self.current]
        }
    }
    #[must_use]
    pub fn peek_offset(&self, offset: usize, counter_offset: usize) -> char {
        self.content.chars().collect::<Vec<_>>()[self.current + offset - counter_offset]
    }
    pub fn skip_whitespace(&mut self) {
        loop {
            // println!("sw {:?}", self.peek_cnow());
            match self.peek_cnow() {
                ' ' | '\r' => {
                    self.advance_char();
                }
                '\n' => {
                    self.line += 1;
                    self.advance_char();
                }
                _ => return,
            }
        }
    }
}
