use std::fs;

use super::keyword::Keyword;
use super::token::{TokenType, is_operator};

#[derive(Debug)]
pub struct Lexer {
    pub content: Vec<u8>,
    pub pos: usize,
}

impl Lexer {
    pub fn init(file_path: &str) -> Self {
        let content = fs::read(file_path).expect("failed to read input file");

        Self { content, pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.content.get(self.pos).map(|&b| b as char)
    }

    pub fn lexer_getc(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += 1;

        Some(c)
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|c| c.is_ascii_whitespace()) {
            self.pos += 1;
        }
    }

    fn lex_while(&mut self, pred: impl Fn(char) -> bool) -> String {
        let start = self.pos;

        while self.peek().is_some_and(&pred) {
            self.pos += 1;
        }

        String::from_utf8_lossy(&self.content[start..self.pos]).into_owned()
    }

    fn next_token_operator(&mut self) -> TokenType {
        match self.lexer_getc() {
            Some('{') => TokenType::LBracket,
            Some('}') => TokenType::RBracket,
            Some('(') => TokenType::LParen,
            Some(')') => TokenType::RParen,
            Some(';') => TokenType::Semi,
            _ => unreachable!("next_token_operator called on a non-operator"),
        }
    }

    fn next_token_int(&mut self) -> TokenType {
        let digits = self.lex_while(|c| c.is_ascii_digit());

        match digits.parse::<i32>() {
            Ok(n) => TokenType::Constant(n),
            Err(e) => panic!("invalid integer literal '{}': {}", digits, e),
        }
    }

    fn next_token_word(&mut self) -> TokenType {
        let word = self.lex_while(|c| c.is_ascii_alphanumeric() || c == '_');

        match Keyword::from_word(&word) {
            Some(kw) => TokenType::Keyword(kw),
            None => TokenType::Identifier(word),
        }
    }

    pub fn next_token(&mut self) -> TokenType {
        self.skip_whitespace();

        match self.peek() {
            None => TokenType::Eof,
            Some(c) if is_operator(c) => self.next_token_operator(),
            Some(c) if c.is_ascii_digit() => self.next_token_int(),
            Some(c) if c.is_ascii_alphabetic() || c == '_' => self.next_token_word(),
            Some(c) => panic!("unexpected character '{}' at offset {}", c, self.pos),
        }
    }
}
