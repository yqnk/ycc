use std::collections::VecDeque;
use std::fs;

use crate::ops::{Binop, Unop};

use super::keyword::Keyword;
use super::token::TokenType;

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

    fn maybe_assign(&mut self, op: Binop) -> TokenType {
        if self.consume_if('=') {
            TokenType::Assign(Some(op))
        } else {
            TokenType::Binop(op)
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
        let c = self
            .lexer_getc()
            .expect("next_token_operator called at end of input");

        match c {
            '{' => TokenType::LBracket,
            '}' => TokenType::RBracket,
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            ';' => TokenType::Semi,

            '~' => TokenType::Unop(Unop::BitwiseComplement),

            '+' => self.maybe_assign(Binop::Add),
            '-' => self.maybe_assign(Binop::Sub),
            '*' => self.maybe_assign(Binop::Mul),
            '/' => self.maybe_assign(Binop::Div),
            '%' => self.maybe_assign(Binop::Mod),
            '^' => self.maybe_assign(Binop::BitwiseXor),

            '!' if self.consume_if('=') => TokenType::Binop(Binop::NotEqualTo),
            '!' => TokenType::Unop(Unop::LogicalNegation),

            '=' if self.consume_if('=') => TokenType::Binop(Binop::EqualTo),
            '=' => TokenType::Assign(None),

            '<' if self.consume_if('=') => TokenType::Binop(Binop::LessThanOrEqualTo),
            '<' if self.consume_if('<') => self.maybe_assign(Binop::LShift),
            '<' => TokenType::Binop(Binop::LessThan),

            '>' if self.consume_if('=') => TokenType::Binop(Binop::GreaterThanOrEqualTo),
            '>' if self.consume_if('>') => self.maybe_assign(Binop::RShift),
            '>' => TokenType::Binop(Binop::GreaterThan),

            '&' if self.consume_if('&') => TokenType::Binop(Binop::LogicalAnd),
            '&' => self.maybe_assign(Binop::BitwiseAnd),

            '|' if self.consume_if('|') => TokenType::Binop(Binop::LogicalOr),
            '|' => self.maybe_assign(Binop::BitwiseOr),

            c => panic!("unexpected character '{}' at offset {}", c, self.pos - 1),
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
        if let Some(front) = self.token_queue.front() {
            return front.clone();
        }

        self.skip_whitespace();

        let token_type = match self.peek() {
            None => TokenType::Eof,
            Some(c) if c.is_ascii_digit() => self.next_token_int(),
            Some(c) if c.is_ascii_alphabetic() || c == '_' => self.next_token_word(),
            Some(_) => self.next_token_operator(),
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
