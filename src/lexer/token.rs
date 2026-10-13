use crate::ops::{Binop, Unop};

use super::keyword::Keyword;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    LBracket,
    RBracket,
    LParen,
    RParen,

    Binop(Binop),
    Unop(Unop),
    Assign(Option<Binop>),

    Identifier(String),
    Keyword(Keyword),
    Constant(i32),

    Colon,
    QuestionMark,

    Semi,
    Eof,
    None,
}

impl TokenType {
    pub fn is_keyword(&self) -> bool {
        matches!(self, TokenType::Keyword(_))
    }

    pub fn is_type_specifier(&self) -> bool {
        matches!(self, TokenType::Keyword(Keyword::Int))
    }

    pub fn is_constant(&self) -> bool {
        matches!(self, TokenType::Constant(_))
    }

    pub fn is_ident(&self) -> bool {
        matches!(self, TokenType::Identifier(_))
    }

    pub fn as_binop(&self) -> Option<Binop> {
        match self {
            TokenType::Binop(op) => Some(*op),
            _ => None,
        }
    }

    pub fn as_unop(&self) -> Option<Unop> {
        match self {
            TokenType::Unop(op) => Some(*op),
            TokenType::Binop(op) => op.as_unop(),
            _ => None,
        }
    }
}
