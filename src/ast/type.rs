use crate::lexer::Keyword;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Type {
    Int,
}

pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Unop {
    Negation,
    LogicalNegation,
    BitwiseComplement,
}

impl Type {
    pub fn from_keyword(kw: &Keyword) -> Option<Self> {
        match kw {
            Keyword::Int => Some(Type::Int),
            _ => None,
        }
    }
}

impl Binop {
    pub fn symbol(&self) -> &'static str {
        match self {
            Binop::Add => "+",
            Binop::Sub => "-",
            Binop::Mul => "*",
            Binop::Div => "/",
        }
    }
}

impl Unop {
    pub fn symbol(&self) -> &'static str {
        match self {
            Unop::Negation => "-",
            Unop::LogicalNegation => "!",
            Unop::BitwiseComplement => "~",
        }
    }
}
