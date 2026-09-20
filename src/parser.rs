use crate::ast::{AstExpr, AstFunc, AstProgram, AstStatement, Type};
use crate::lexer::{Keyword, Lexer, TokenType};

#[derive(Debug)]
pub struct Parser {
    pub cur_token: TokenType,

    pub lexer: Lexer,
}

impl Parser {
    pub fn init(file_path: &str) -> Self {
        let parser = Self {
            cur_token: TokenType::None,
            lexer: Lexer::init(file_path),
        };

        parser
    }

    pub fn is_eof(&self) -> bool {
        self.cur_token == TokenType::Eof
    }

    pub fn advance(&mut self) {
        self.cur_token = self.lexer.next_token();
    }

    pub fn advance_match(&mut self, pred: impl Fn(&TokenType) -> bool) -> TokenType {
        self.advance();
        if !pred(&self.cur_token) {
            panic!("{}:{}: could not match cur_token: {:?}" , file!(), line!(), self.cur_token)
        }

        self.cur_token.clone()
    }

    pub fn parse_program(&mut self) -> AstProgram {
        let program = AstProgram {
            main: self.parse_func(),
        };

        program
    }

    fn parse_type(&mut self) -> Type {
        match self.advance_match(|t| t.is_type_specifier()) {
            TokenType::Keyword(kw) => Type::from_keyword(&kw).unwrap(),
            _ => unreachable!(),
        }
    }

    fn parse_func(&mut self) -> AstFunc {
        let ret_type = self.parse_type();
        let token_name = self.advance_match(|t| t.is_ident());
        let _token_lpar = self.advance_match(|t| *t == TokenType::LParen);
        let _token_rpar = self.advance_match(|t| *t == TokenType::RParen);
        let _token_lbracket = self.advance_match(|t| *t == TokenType::LBracket);
        let statement = self.parse_statement();
        let _token_rbracket = self.advance_match(|t| *t == TokenType::RBracket);

        if let TokenType::Identifier(name) = token_name {
            return AstFunc { ret_type, name, statement };
        }

        unreachable!()
    }

    // this only works for Return(Int);
    fn parse_statement(&mut self) -> AstStatement {
        let _token_return = self.advance_match(|t| *t == TokenType::Keyword(Keyword::Return));
        let exp = self.parse_exp();
        let _token_semi = self.advance_match(|t| t.is_semi());

        AstStatement::Return { exp }
    }

    fn parse_exp(&mut self) -> AstExpr {
        match self.advance_match(|t| t.is_constant()) {
            TokenType::Constant(value) => AstExpr::Int { value },
            _ => unreachable!(),
        }
    }
}
