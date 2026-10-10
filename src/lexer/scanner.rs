use std::collections::VecDeque;
use std::fs;

use super::keyword::Keyword;
use super::token::{TokenType, is_operator};

#[derive(Debug)]
pub struct Lexer {
    pub content: Vec<u8>,
    pub pos: usize,

    pub token_queue: VecDeque<TokenType>,
}

impl Lexer {
    pub fn init(file_path: &str) -> Lexer {
        let content = fs::read(file_path).expect("failed to read input file");
        let token_queue = VecDeque::new();

        Lexer {
            content,
            pos: 0,
            token_queue,
        }
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

    fn consume_if(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.pos += 1;
            true
        } else {
            false
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

            // Unop
            Some('-') => TokenType::Sub,
            Some('~') => TokenType::BitwiseComplement,
            Some('!') => {
                if self.consume_if('=') {
                    TokenType::NotEqualTo
                } else {
                    TokenType::LogicalNegation
                }
            }

            // Binop
            Some('+') => TokenType::Add,
            Some('*') => TokenType::Mul,
            Some('/') => TokenType::Div,
            Some('%') => TokenType::Mod,

            Some('<') => {
                if self.consume_if('=') {
                    TokenType::LessThanOrEqualTo
                } else if self.consume_if('<') {
                    TokenType::LShift
                } else {
                    TokenType::LessThan
                }
            }
            Some('>') => {
                if self.consume_if('=') {
                    TokenType::GreaterThanOrEqualTo
                } else if self.consume_if('>') {
                    TokenType::RShift
                } else {
                    TokenType::GreaterThan
                }
            }

            Some('&') => {
                if self.consume_if('&') {
                    return TokenType::LogicalAnd;
                } else {
                    return TokenType::BitwiseAnd;
                }
            }

            Some('|') => {
                if self.consume_if('|') {
                    return TokenType::LogicalOr;
                } else {
                    return TokenType::BitwiseOr;
                }
            }

            Some('^') => TokenType::BitwiseXor,

            Some('=') if self.consume_if('=') => TokenType::EqualTo,

            Some(c) => panic!("unexpected operator '{}' at offset {}", c, self.pos - 1),
            None => unreachable!("next_token_operator called at end of input"),
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

    pub fn next_token_peek(&mut self) -> TokenType {
        if self.token_queue.len() > 0 {
            if let Some(f) = self.token_queue.front() {
                return f.clone();
            }
        }

        self.skip_whitespace();

        let token_type = match self.peek() {
            None => TokenType::Eof,
            Some(c) if is_operator(c) => self.next_token_operator(),
            Some(c) if c.is_ascii_digit() => self.next_token_int(),
            Some(c) if c.is_ascii_alphabetic() || c == '_' => self.next_token_word(),
            Some(c) => panic!("unexpected character '{}' at offset {}", c, self.pos),
        };

        self.token_queue.push_back(token_type.clone());
        token_type
    }

    pub fn next_token_pop(&mut self) -> TokenType {
        let _ = self.next_token_peek();

        if let Some(t) = self.token_queue.pop_front() {
            return t;
        }

        panic!("lexer went too far")
    }
}
