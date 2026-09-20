use super::keyword::Keyword;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    LBracket,
    RBracket,
    LParen,
    RParen,

    Identifier(String),
    Keyword(Keyword),
    Constant(i32),

    Semi,
    Eof,
    None,
}

pub fn is_operator(c: char) -> bool {
    matches!(c, '{' | '}' | '(' | ')' | ';')
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

    pub fn is_semi(&self) -> bool {
        *self == TokenType::Semi
    }
}
