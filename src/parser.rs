use self::{token::{MdToken, MdTokenType}, presentation::Presentation};


pub mod token;
pub mod presentation;

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
    pub fn parse(&mut self) -> Presentation {
        let tokens = self.tokenize();
        for x in tokens {
            println!("{:#?}", x);
        }
        unreachable!()
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
            println!("New Token");
            tokens.push(self.get_token());
        }
        tokens
    }
    pub fn get_token(&mut self) -> MdToken {
        self.skip_whitespace();
        self.start = self.current;

        let c = self.advance_char();
        println!("CHAR: {c}");
        if c == '#' {
            return self.heading();
        }


        while self.peek_cnow() != '#' && self.current < self.content.len() {
            self.advance_char();
        }
        self.make_token(MdTokenType::Text)


        // if self.current >= self.content.len() {
        //     return self.make_token(MdTokenType::Text);
        // }
    }
    pub fn heading(&mut self) -> MdToken {
        let mut level: u8 = 1;
        while self.peek_cnow() == '#' {
            level+=1;
            self.advance_char();
        }
        while self.peek_char(1, 0) != '\n' {
            println!("char {}", self.advance_char());
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
    pub fn peek_char(&self, offset: usize, counter_offset: usize) -> char {
        if self.current+offset-counter_offset == 0{
            return self.content.chars().collect::<Vec<_>>()[0];
        }
        *self.content.chars().collect::<Vec<_>>().get(self.current-1+offset-counter_offset).unwrap_or(&'\0')
    }
    pub fn peek_cnow(&self) -> char {
        if self.current >= self.content.chars().collect::<Vec<_>>().len(){
            '\0'
        } else {
            self.content.chars().collect::<Vec<_>>()[self.current]
        }
    }
    pub fn skip_whitespace(&mut self) {
        loop {
            println!("sw {:?}", self.peek_cnow());
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

#[derive(Debug, Clone)]
pub struct Slide{
    content: Vec<Hunk>,
}

#[derive(Debug, Clone)]
pub struct Hunk{
    content: String,
    color: crossterm::style::Color,
}
