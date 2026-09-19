use super::keywords::Keyword;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    LBracket,
    RBracket,
    LParen,
    RParen,

    Int(i32),
    Identifier(String),
    Keyword(Keyword),

    Semi,
    Eof,
    None,
}

pub fn is_operator(c: char) -> bool {
    matches!(c, '{' | '}' | '(' | ')' | ';')
}
