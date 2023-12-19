pub mod md;
pub use md::*;

use crate::parser::token::Token;

pub trait Lexer {
    fn tokenize(&mut self) -> Vec<Token>;
    fn set_src(&mut self, src: &str);
}

