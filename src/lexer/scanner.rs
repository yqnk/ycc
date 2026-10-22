use std::collections::VecDeque;

use crate::lexer::source::{SourceFile, Span};
use crate::lexer::token::Token;
use crate::ops::{Binop, Unop};

use super::keyword::Keyword;
use super::token::TokenType;

#[derive(Debug)]
pub struct Lexer {
    pub source: SourceFile,
    pub pos: usize,

    tok_start: usize, // token start, token end will be `pos`

    pub token_queue: VecDeque<Token>,
}

impl Lexer {
    pub fn init(file_path: &str) -> Lexer {
        Lexer {
            source: SourceFile::read(file_path),
            pos: 0,

            tok_start: 0,

            token_queue: VecDeque::new(),
        }
    }

    fn tok_span(&self) -> Span {
        Span::new(self.tok_start, self.pos)
    }

    fn tok_loc(&self) -> String {
        self.source.fmt_span(self.tok_span())
    }

    fn token(&self, r#type: TokenType) -> Token {
        Token {
            span: self.tok_span(),
            r#type,
        }
    }

    fn peek_raw(&self) -> Option<char> {
        self.source.content.get(self.pos).map(|&b| b as char)
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.peek_raw()?;
        self.pos += 1;

        Some(c)
    }

    fn skip_whitespace(&mut self) {
        while self.peek_raw().is_some_and(|c| c.is_ascii_whitespace()) {
            self.bump();
        }
    }

    fn consume_if(&mut self, expected: char) -> bool {
        if self.peek_raw() == Some(expected) {
            self.bump();

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

        while self.peek_raw().is_some_and(&pred) {
            self.bump();
        }

        String::from_utf8_lossy(&self.source.content[start..self.pos]).into_owned()
    }

    fn next_token_operator(&mut self) -> TokenType {
        let c = self
            .bump()
            .expect("next_token_operator called at end of input");

        match c {
            '{' => TokenType::LBrace,
            '}' => TokenType::RBrace,
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

            ':' => TokenType::Colon,
            '?' => TokenType::QuestionMark,

            c => panic!(
                "{}: unexpected character '{}':\n{}",
                self.tok_loc(),
                c,
                self.source.snippet(self.tok_span())
            ),
        }
    }

    fn next_token_int(&mut self) -> TokenType {
        let digits = self.lex_while(|c| c.is_ascii_digit());

        match digits.parse::<i32>() {
            Ok(n) => TokenType::Constant(n),
            Err(e) => panic!(
                "{}: invalid integer literal '{}': {}",
                self.tok_loc(),
                digits,
                e
            ),
        }
    }

    fn next_token_word(&mut self) -> TokenType {
        let word = self.lex_while(|c| c.is_ascii_alphanumeric() || c == '_');

        match Keyword::from_word(&word) {
            Some(kw) => TokenType::Keyword(kw),
            None => TokenType::Identifier(word),
        }
    }

    pub fn peek_token(&mut self) -> Token {
        if let Some(front) = self.token_queue.front() {
            return front.clone();
        }

        self.skip_whitespace();

        self.tok_start = self.pos;

        let token_type = match self.peek_raw() {
            None => TokenType::Eof,
            Some(c) if c.is_ascii_digit() => self.next_token_int(),
            Some(c) if c.is_ascii_alphabetic() || c == '_' => self.next_token_word(),
            Some(_) => self.next_token_operator(),
        };

        let token = self.token(token_type);

        self.token_queue.push_back(token.clone());
        token
    }

    pub fn token_pop(&mut self) -> Token {
        let _ = self.peek_token();

        if let Some(t) = self.token_queue.pop_front() {
            return t;
        }

        panic!("lexer went too far")
    }
}
