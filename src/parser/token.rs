#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Text,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub content: String,
}

impl Token {
    #[must_use]
    pub const fn new(ttype: TokenType, content: String) -> Self {
        Self { ttype, content }
    }
}
