use crate::lexer::Keyword;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Type {
    Int,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    LogicalAnd,
    LogicalOr,
    EqualTo,
    NotEqualTo,
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
}

pub enum Unop {
    Addition,
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
            Binop::LogicalOr => "||",
            Binop::LogicalAnd => "&&",
            Binop::EqualTo => "==",
            Binop::NotEqualTo => "!=",
            Binop::LessThan => "<",
            Binop::LessThanOrEqualTo => "<=",
            Binop::GreaterThan => ">",
            Binop::GreaterThanOrEqualTo => ">=",
        }
    }
}

impl Unop {
    pub fn symbol(&self) -> &'static str {
        match self {
            Unop::Addition => "+",
            Unop::Negation => "-",
            Unop::LogicalNegation => "!",
            Unop::BitwiseComplement => "~",
        }
    }
}
