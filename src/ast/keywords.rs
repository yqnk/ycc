#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Int,
    Return,
}

impl Keyword {
    pub fn from_word(s: &str) -> Option<Self> {
        match s {
            "int" => Some(Keyword::Int),
            "return" => Some(Keyword::Return),
            _ => None,
        }
    }
}
