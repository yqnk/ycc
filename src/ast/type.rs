use crate::Keyword;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Type {
    Int,
}

impl Type {
    pub fn from_keyword(kw: &Keyword) -> Self {
        match kw {
            Keyword::Int => Type::Int,
            _ => unreachable!("unknown type was given"),
        }
    }
}
