#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Int,
    Return,
    If,
    Else,

    For,
    While,
    Do,
    Break,
    Continue,
}

impl Keyword {
    pub fn from_word(s: &str) -> Option<Self> {
        match s {
            "return" => Some(Keyword::Return),
            "int" => Some(Keyword::Int),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "for" => Some(Keyword::For),
            "while" => Some(Keyword::While),
            "do" => Some(Keyword::Do),
            "break" => Some(Keyword::Break),
            "continue" => Some(Keyword::Continue),
            _ => None,
        }
    }
}
