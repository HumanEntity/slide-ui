// #[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    #[default]
    Text,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Link,
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
