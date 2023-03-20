use crossterm::style::Color;

use crate::new_slide;

use self::{token::{MdToken, MdTokenType}, presentation::*};


pub mod token;
pub mod presentation;

#[derive(Debug, Clone)]
pub struct MdLexer {
    content: String,
    start: usize,
    current: usize,
    line: usize,
}

impl MdLexer{
    #[inline]
    pub fn new(content: String) -> Self {
        Self {
            content,
            start: 0,
            current: 0,
            line: 0,
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
    */
    pub fn tokenize(&mut self) -> Vec<MdToken> {
        let mut tokens: Vec<MdToken> = Vec::new();
        while self.current < self.content.len() - 1 {
            tokens.push(self.get_token());
        }
        tokens
    }
    pub fn get_token(&mut self) -> MdToken {
        self.skip_whitespace();
        self.start = self.current;

        let c = self.advance_char();
        if c == '#' {
            return self.heading();
        }

        if c == '<' {
        if let Some(token) = self.new_slide() {
            return token;
        }
        }

        while self.peek_cnow() != '#' && self.peek_cnow() != '\n' && self.current < self.content.len() {
            self.advance_char();
        }
        self.make_token(MdTokenType::Text)


        // if self.current >= self.content.len() {
        //     return self.make_token(MdTokenType::Text);
        // }
    }
    pub fn new_slide(&mut self) -> Option<MdToken> {
        //if self.peek_cnow() == '<' {
            if self.peek_cnow() == ':' {
                self.advance_char();
                if self.peek_cnow() == ')' {
                    self.advance_char();
                    if self.peek_cnow() == '>' {
                        self.advance_char();
                        return Some(self.make_token(MdTokenType::NewSlide));
                    }
                }
            }
        //}
        None
    }
    pub fn heading(&mut self) -> MdToken {
        let mut level: u8 = 1;
        while self.peek_cnow() == '#' {
            level+=1;
            self.advance_char();
        }
        while self.peek_cnow() != '\n' {
            self.advance_char();
        }
        println!("level: {level}");
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
        *self.content.chars().collect::<Vec<_>>().get(self.current-1).unwrap_or(&'\0')
    }
    /* pub fn peek_char(&self, offset: usize, counter_offset: usize) -> char {
        if self.current+offset-counter_offset == 0 || self.current+offset-counter_offset >= self.content.chars().collect::<Vec<_>>().len(){
            return self.content.chars().collect::<Vec<_>>()[0];
        }
        self.content.chars().collect::<Vec<_>>()[self.current+offset-counter_offset]
    }*/
    pub fn peek_cnow(&self) -> char {
        if self.current >= self.content.chars().collect::<Vec<_>>().len(){
            '\0'
        } else {
            self.content.chars().collect::<Vec<_>>()[self.current]
        }
    }
    pub fn peek_offset(&self, offset: usize, counter_offset: usize) -> char{
        self.content.chars().collect::<Vec<_>>()[self.current+offset-counter_offset]
    }
    pub fn skip_whitespace(&mut self) {
        loop {
            // println!("sw {:?}", self.peek_cnow());
            match self.peek_cnow() {
                ' ' | '\r'  => {self.advance_char();},
                '\n' => {
                    self.line += 1;
                    self.advance_char();
                }
                _ => return,
            }
        }
    }
}

pub struct MdParser{
    content: String,
}

impl MdParser {
    pub fn new(content: String) -> Self {
        Self {
            content
        }
    }

    pub fn parse(&mut self) -> Presentation {
        let tokens = MdLexer::new(self.content.clone()).tokenize();
        let mut slides: Vec<Slide> = Vec::new(); slides.push(new_slide!());
        let mut current = 0;
        for token in tokens {
            println!("{token:#?}");
            let mut hunks = &mut slides[current].content;
            match token.ttype {
                MdTokenType::H1 => hunks.push(Hunk {
                    content: token.content,
                    bg_color: Color::Black,
                    fg_color: Color::Red,
                }),
                MdTokenType::H2 => hunks.push(Hunk {
                    content: token.content,
                    bg_color: Color::Black,
                    fg_color: Color::Green,
                }),
                MdTokenType::H3 => hunks.push(Hunk {
                    content: token.content,
                    bg_color: Color::Black,
                    fg_color: Color::Yellow,
                }),
                MdTokenType::H4 => hunks.push(Hunk {
                    content: token.content,
                    bg_color: Color::Black,
                    fg_color: Color::Blue,
                }),
                MdTokenType::H5 => hunks.push(Hunk {
                    content: token.content,
                    bg_color: Color::Black,
                    fg_color: Color::Magenta,
                }),
                MdTokenType::H6 => hunks.push(Hunk {
                    content: token.content,
                    bg_color: Color::Black,
                    fg_color: Color::Cyan,
                }),
                MdTokenType::Text => hunks.push(Hunk {
                    content: token.content,
                    bg_color: Color::Black,
                    fg_color: Color::White,
                }),
                MdTokenType::NewSlide => {
                    current+=1;
                    slides.push(new_slide!());
                }
            }
        }
        Presentation::new(slides)
    }
}
