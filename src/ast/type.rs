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
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Unop {
    Addition,
    Negation,
    LogicalNegation,
    BitwiseComplement,
}

impl Type {
    pub fn from_keyword(kw: &Keyword) -> Self {
        match kw {
            Keyword::Int => Type::Int,
            _ => unreachable!("unknown type was given"),
        }
    }
}

impl Binop {
    pub fn from_token(token: &TokenType) -> Option<(Binop, u8)> {
        use TokenType as T;

        Some(match token {
            T::LogicalOr => (Binop::LogicalOr, 1),
            T::LogicalAnd => (Binop::LogicalAnd, 2),
            T::BitwiseOr => (Binop::BitwiseOr, 3),
            T::BitwiseXor => (Binop::BitwiseXor, 4),
            T::BitwiseAnd => (Binop::BitwiseAnd, 5),
            T::EqualTo => (Binop::EqualTo, 6),
            T::NotEqualTo => (Binop::NotEqualTo, 6),
            T::LessThan => (Binop::LessThan, 7),
            T::LessThanOrEqualTo => (Binop::LessThanOrEqualTo, 7),
            T::GreaterThan => (Binop::GreaterThan, 7),
            T::GreaterThanOrEqualTo => (Binop::GreaterThanOrEqualTo, 7),
            T::LShift => (Binop::LShift, 8),
            T::RShift => (Binop::RShift, 8),
            T::Add => (Binop::Add, 9),
            T::Sub => (Binop::Sub, 9),
            T::Mul => (Binop::Mul, 10),
            T::Div => (Binop::Div, 10),
            T::Mod => (Binop::Mod, 10),
            _ => return None,
        })
    }

    pub fn token(&self) -> TokenType {
        match &self {
            Binop::Add => TokenType::Add,
            Binop::Sub => TokenType::Sub,
            Binop::Mul => TokenType::Mul,
            Binop::Div => TokenType::Div,
            Binop::Mod => TokenType::Mod,
            Binop::LShift => TokenType::LShift,
            Binop::RShift => TokenType::RShift,
            Binop::BitwiseOr => TokenType::BitwiseOr,
            Binop::BitwiseXor => TokenType::BitwiseXor,
            Binop::BitwiseAnd => TokenType::BitwiseAnd,
            op => unreachable!("no single token for {:?}", op),
        }
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
