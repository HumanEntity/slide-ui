
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MdTokenType{
    H1, H2, H3, H4, H5, H6,
    Text,
}

pub struct MdToken{
    ttype: MdTokenType,
    content: String,
}

impl MdToken{
    pub fn new(ttype: MdTokenType, content: String) -> Self {
        Self {
            ttype,
            content,
        }
    }
}
