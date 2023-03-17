use std::error;

use self::token::{MdToken, MdTokenType};


pub mod token;

#[derive(Debug, Clone)]
pub struct MdParser {
    content: String,
    start: usize,
    current: usize,
    line: usize,
}

impl MdParser{
    #[inline]
    pub fn new(content: String) -> Self {
        Self {
            content,
            start: 0,
            current: 0,
            line: 0,
        }
    }
    pub fn parse(&mut self) {
        let tokens = self.tokenize();
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
    */
    pub fn tokenize(&mut self) -> Vec<MdToken> {
        let mut tokens: Vec<MdToken> = Vec::new();
        while self.current < self.content.len() {
            tokens.push(self.get_token());
        }
        tokens
    }
    pub fn get_token(&mut self) -> MdToken {
        self.start = self.current;
        self.advance_char();

        let c = self.peek_char(0, 0);
        if c == '#' {
            self.heading();
        }

        while self.peek_char(0, 0) != '#' || self.current >= self.content.len() {
            self.advance_char();
        }
        self.make_token(MdTokenType::Text)
    }
    pub fn heading(&mut self) -> MdToken {
        let mut level: u8 = 0;
        while self.peek_char(0, 0) == '#' {
            level+=1;
            self.advance_char();
        }
        match level {
            1 => self.make_token(MdTokenType::H1),
            2 => self.make_token(MdTokenType::H2),
            3 => self.make_token(MdTokenType::H3),
            4 => self.make_token(MdTokenType::H4),
            5 => self.make_token(MdTokenType::H5),
            6 => self.make_token(MdTokenType::H6),
            _ => {eprintln!("Not valid heading in markdown (as I know it)"); unreachable!()} 
        }
    }
    fn make_token(&self, ttype: MdTokenType) -> MdToken {
        let mut str = String::new();
        for i in self.start..self.current {
            str.push(self.content.chars().collect::<Vec<_>>()[i]);
        }

        MdToken::new(ttype, str)
    }
    pub fn advance_char(&mut self) -> char{
        self.current+=1;
        self.content.chars().collect::<Vec<_>>()[self.current-1]
    }
    pub fn peek_char(&self, offset: usize, counter_offset: usize) -> char {
        self.content.chars().collect::<Vec<_>>()[self.current+offset-counter_offset]
    }
}

#[derive(Debug, Clone)]
pub struct Slide{
    content: Vec<Hunk>,
}

#[derive(Debug, Clone)]
pub struct Hunk{
    content: String,
    color: crossterm::style::Color,
}
