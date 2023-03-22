#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MdTokenType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Text,

    NewSlide,
}

#[derive(Debug, Clone)]
pub struct MdToken {
    pub ttype: MdTokenType,
    pub content: String,
}

impl MdToken {
    #[must_use]
    pub const fn new(ttype: MdTokenType, content: String) -> Self {
        Self { ttype, content }
    }
}
