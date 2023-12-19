pub mod md;
pub mod org;
pub use md::*;
pub use org::*;

use crate::parser::token::Token;

pub trait Lexer {
    fn tokenize(&mut self) -> Vec<Token>;
    fn set_src(&mut self, src: &str);
}

