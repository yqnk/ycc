use crate::lexer::{Keyword, TokenType};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Type {
    Int,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    LogicalAnd,
    LogicalOr,
    EqualTo,
    NotEqualTo,
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LShift,
    RShift,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Assignment,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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
    pub fn from_token(token: &TokenType) -> Option<(Binop, u8)> {
        use TokenType as T;

        Some(match token {
            T::Assignment => (Binop::Assignment, 1),
            T::LogicalOr => (Binop::LogicalOr, 2),
            T::LogicalAnd => (Binop::LogicalAnd, 3),
            T::BitwiseOr => (Binop::BitwiseOr, 4),
            T::BitwiseXor => (Binop::BitwiseXor, 5),
            T::BitwiseAnd => (Binop::BitwiseAnd, 6),
            T::EqualTo => (Binop::EqualTo, 7),
            T::NotEqualTo => (Binop::NotEqualTo, 7),
            T::LessThan => (Binop::LessThan, 8),
            T::LessThanOrEqualTo => (Binop::LessThanOrEqualTo, 8),
            T::GreaterThan => (Binop::GreaterThan, 8),
            T::GreaterThanOrEqualTo => (Binop::GreaterThanOrEqualTo, 8),
            T::LShift => (Binop::LShift, 9),
            T::RShift => (Binop::RShift, 9),
            T::Add => (Binop::Add, 10),
            T::Sub => (Binop::Sub, 10),
            T::Mul => (Binop::Mul, 11),
            T::Div => (Binop::Div, 11),
            T::Mod => (Binop::Mod, 11),
            _ => return None,
        })
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Binop::Add => "+",
            Binop::Sub => "-",
            Binop::Mul => "*",
            Binop::Div => "/",
            Binop::Mod => "%",
            Binop::LogicalOr => "||",
            Binop::LogicalAnd => "&&",
            Binop::EqualTo => "==",
            Binop::NotEqualTo => "!=",
            Binop::LessThan => "<",
            Binop::LessThanOrEqualTo => "<=",
            Binop::GreaterThan => ">",
            Binop::GreaterThanOrEqualTo => ">=",
            Binop::LShift => "<<",
            Binop::RShift => ">>",
            Binop::BitwiseOr => "|",
            Binop::BitwiseXor => "^",
            Binop::BitwiseAnd => "&",
            Binop::Assignment => "=",
        }
    }
}

impl Unop {
    pub fn from_token(token: &TokenType) -> Option<Self> {
        match token {
            TokenType::Add => Some(Unop::Addition),
            TokenType::Sub => Some(Unop::Negation),
            TokenType::LogicalNegation => Some(Unop::LogicalNegation),
            TokenType::BitwiseComplement => Some(Unop::BitwiseComplement),
            _ => None,
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Unop::Addition => "+",
            Unop::Negation => "-",
            Unop::LogicalNegation => "!",
            Unop::BitwiseComplement => "~",
        }
    }
}
