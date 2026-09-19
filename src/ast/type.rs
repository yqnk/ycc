use super::keywords::Keyword;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Type {
    Int,
}

impl Type {
    pub fn from_keyword(kw: &Keyword) -> Option<Self> {
        match kw {
            Keyword::Int => Some(Type::Int),
            _ => None,
        }
    }
}
