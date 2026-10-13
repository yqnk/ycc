use crate::{
    Binop,
    ast::*,
    lexer::{Keyword, Lexer, TokenType},
};

#[derive(Debug)]
pub struct Parser {
    pub cur_token: TokenType,
    pub lexer: Lexer,
}

impl Parser {
    pub fn init(file_path: &str) -> Self {
        Self {
            cur_token: TokenType::None,
            lexer: Lexer::init(file_path),
        }
    }

    pub fn is_eof(&self) -> bool {
        self.cur_token == TokenType::Eof
    }

    pub fn advance(&mut self) -> TokenType {
        self.cur_token = self.lexer.next_token_pop();

        self.cur_token.clone()
    }

    pub fn advance_match(&mut self, pred: impl Fn(&TokenType) -> bool) -> TokenType {
        self.advance();
        if !pred(&self.cur_token) {
            panic!(
                "{}:{}: could not match cur_token: {:?}",
                file!(),
                line!(),
                self.cur_token
            )
        }

        self.cur_token.clone()
    }

    pub fn parse_program(&mut self) -> AstProgram {
        AstProgram {
            main: self.parse_func(),
        }
    }

    fn parse_type(&mut self) -> Type {
        match self.advance_match(|t| t.is_type_specifier()) {
            TokenType::Keyword(kw) => Type::from_keyword(&kw),
            _ => unreachable!(),
        }
    }

    fn parse_identifier(&mut self) -> String {
        match self.advance_match(|t| t.is_ident()) {
            TokenType::Identifier(str) => str,
            _ => unreachable!(),
        }
    }

    fn parse_func(&mut self) -> AstFunc {
        let ret_type = self.parse_type();
        let token_name = self.advance_match(|t| t.is_ident());
        let _token_lpar = self.advance_match(|t| *t == TokenType::LParen);
        let _token_rpar = self.advance_match(|t| *t == TokenType::RParen);
        let _token_lbracket = self.advance_match(|t| *t == TokenType::LBracket);

        let mut blocks = Vec::new();

        loop {
            match self.lexer.next_token_peek() {
                TokenType::RBracket => break,
                TokenType::Eof => panic!("unterminated function body: missing '}}'"),
                _ => blocks.push(self.parse_block_item()),
            }
        }

        let _token_rbracket = self.advance_match(|t| *t == TokenType::RBracket);

        if let TokenType::Identifier(name) = token_name {
            return AstFunc {
                ret_type,
                name,
                blocks,
            };
        }

        unreachable!()
    }

    fn parse_block_item(&mut self) -> AstBlockItem {
        if self.lexer.next_token_peek().is_type_specifier() {
            return self.parse_vardec();
        }

        AstBlockItem::Statement {
            statement: self.parse_statement(),
        }
    }

    fn parse_vardec(&mut self) -> AstBlockItem {
        let var_type = self.parse_type();
        let name = self.parse_identifier();

        let exp = match self.advance() {
            TokenType::Semi => None,
            TokenType::Assign(None) => {
                let exp = self.parse_expr();
                self.advance_match(|pred| pred.is_semi());

                Some(exp)
            }
            t => panic!(
                "expected '=' or ';' in declaration of {}, got {:?}",
                name, t
            ),
        };

        AstBlockItem::VarDec {
            var_type,
            name,
            exp,
        }
    }

    fn parse_statement(&mut self) -> AstStatement {
        let token = self.lexer.next_token_peek();

        match token {
            TokenType::Keyword(Keyword::Return) => {
                self.advance();

                let exp = self.parse_expr();
                self.advance_match(|pred| pred.is_semi());

                AstStatement::Return { exp }
            }
            _ => {
                let exp = self.parse_expr();
                self.advance_match(|pred| pred.is_semi());

                AstStatement::Expr { exp }
            }
        }
    }

    fn parse_expr(&mut self) -> AstExpr {
        let lhs = self.parse_binary(Binop::LOWEST_PRECEDENCE);

        let TokenType::Assign(op) = self.lexer.next_token_peek() else {
            return lhs;
        };
        self.advance();

        let AstExpr::Var(name) = lhs else {
            panic!(
                "expected variable on the left of an assignment, got: {:?}",
                lhs
            );
        };

        AstExpr::Assignment {
            op,
            lhs: name,
            rhs: Box::new(self.parse_expr()),
        }
    }

    fn parse_binary(&mut self, min_power: u8) -> AstExpr {
        let mut lhs = self.parse_factor();

        while let Some(op) = self.lexer.next_token_peek().as_binop() {
            let power = op.precedence();
            if power < min_power {
                break;
            }
            self.advance();

            // power + 1: every C binary operator is left-associative
            let rhs = self.parse_binary(power + 1);
            lhs = AstExpr::Binary {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }

        lhs
    }

    fn parse_factor(&mut self) -> AstExpr {
        let token = self.advance();

        if let Some(op) = token.as_unop() {
            return AstExpr::Unary {
                op,
                operand: Box::new(self.parse_factor()),
            };
        }

        match token {
            TokenType::LParen => {
                let exp = self.parse_expr();
                let _rpar = self.advance_match(|t| *t == TokenType::RParen);
                exp
            }
            TokenType::Constant(value) => AstExpr::Int(value),
            TokenType::Identifier(name) => AstExpr::Var(name),
            t => {
                unimplemented!("unknown factor token: {:?}", t)
            }
        }
    }
}
