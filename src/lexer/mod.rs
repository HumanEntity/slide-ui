pub mod md;
pub mod org;
pub use md::*;
pub use org::*;

use crate::parser::token::Token;

pub trait Lexer {
    fn tokenize(&mut self) -> Vec<Token>;
    fn set_src(&mut self, src: &str);
}

impl dyn Lexer {
    fn tokn(&mut self) -> Vec<Token> {
        self.tokenize()
    }
    fn set_source(&mut self, src: &str) {
        self.set_src(src)
    }
}

impl Lexer for Box<dyn Lexer> {
    fn tokenize(&mut self) -> Vec<Token> {
        self.tokn()
    }
    fn set_src(&mut self, src: &str) {
        self.set_source(src)
    }
}
