#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Int,
    Return,
}

impl Keyword {
    pub fn from_word(s: &str) -> Option<Self> {
        match s {
            "return" => Some(Keyword::Return),
            "int" => Some(Keyword::Int),
            _ => None,
        }
    }
}
