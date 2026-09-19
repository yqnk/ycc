use super::lexer::Lexer;
use super::token::TokenType;

#[derive(Debug)]
pub struct Parser {
    pub cur_token: TokenType,

    pub lexer: Lexer,
}

impl Parser {
    pub fn init(file_path: &str) -> Self {
        let mut parser = Self {
            cur_token: TokenType::None,
            lexer: Lexer::init(file_path),
        };

        parser.advance();
        parser
    }

    pub fn advance(&mut self) {
        self.cur_token = self.lexer.next_token();
    }

    pub fn is_eof(&self) -> bool {
        self.cur_token == TokenType::Eof
    }
}
