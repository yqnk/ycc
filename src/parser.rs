use crate::ast::*;
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
            return AstFunc {
                ret_type,
                name,
                statement,
            };
        }

        unreachable!()
    }

    // this only works for Return(Int);
    fn parse_statement(&mut self) -> AstStatement {
        let _token_return = self.advance_match(|t| *t == TokenType::Keyword(Keyword::Return));
        let exp = self.parse_expr();
        let _token_semi = self.advance_match(|t| t.is_semi());

        AstStatement::Return { exp }
    }

    fn parse_expr(&mut self) -> AstExpr {
        let first = self.parse_logical_and_expr();
        let mut rest = Vec::new();

        let mut next = self.lexer.next_token_peek();
        while matches!(next, TokenType::LogicalOr) {
            self.advance();

            let next_expr = self.parse_logical_and_expr();
            rest.push((Binop::LogicalOr, next_expr));

            next = self.lexer.next_token_peek();
        }

        AstExpr { first, rest }
    }

    fn parse_logical_and_expr(&mut self) -> AstLogicalAndExpr {
        let first = self.parse_bitwise_or_expr();
        let mut rest = Vec::new();

        let mut next = self.lexer.next_token_peek();
        while matches!(next, TokenType::LogicalAnd) {
            self.advance();

            let next_expr = self.parse_bitwise_or_expr();
            rest.push((Binop::LogicalAnd, next_expr));

            next = self.lexer.next_token_peek();
        }

        AstLogicalAndExpr { first, rest }
    }

    fn parse_bitwise_or_expr(&mut self) -> AstBitwiseOrExpr {
        let first = self.parse_bitwise_xor_expr();
        let mut rest = Vec::new();

        let mut next = self.lexer.next_token_peek();
        while matches!(next, TokenType::BitwiseOr) {
            self.advance();

            let next_expr = self.parse_bitwise_xor_expr();
            rest.push((Binop::BitwiseOr, next_expr));

            next = self.lexer.next_token_peek();
        }

        AstBitwiseOrExpr { first, rest }
    }

    fn parse_bitwise_xor_expr(&mut self) -> AstBitwiseXorExpr {
        let first = self.parse_bitwise_and_expr();
        let mut rest = Vec::new();

        let mut next = self.lexer.next_token_peek();
        while matches!(next, TokenType::BitwiseXor) {
            self.advance();

            let next_expr = self.parse_bitwise_and_expr();
            rest.push((Binop::BitwiseXor, next_expr));

            next = self.lexer.next_token_peek();
        }

        AstBitwiseXorExpr { first, rest }
    }

    fn parse_bitwise_and_expr(&mut self) -> AstBitwiseAndExpr {
        let first = self.parse_equality_expr();
        let mut rest = Vec::new();

        let mut next = self.lexer.next_token_peek();
        while matches!(next, TokenType::BitwiseAnd) {
            self.advance();

            let next_expr = self.parse_equality_expr();
            rest.push((Binop::BitwiseAnd, next_expr));

            next = self.lexer.next_token_peek();
        }

        AstBitwiseAndExpr { first, rest }
    }

    fn parse_equality_expr(&mut self) -> AstEqualityExpr {
        let first = self.parse_relational_expr();
        let mut rest = Vec::new();

        let mut next = self.lexer.next_token_peek();
        while matches!(next, TokenType::EqualTo | TokenType::NotEqualTo) {
            self.advance();

            let binop = match next {
                TokenType::EqualTo => Binop::EqualTo,
                _ => Binop::NotEqualTo,
            };

            let next_expr = self.parse_relational_expr();
            rest.push((binop, next_expr));

            next = self.lexer.next_token_peek();
        }

        AstEqualityExpr { first, rest }
    }

    fn parse_relational_expr(&mut self) -> AstRelationalExpr {
        let first = self.parse_additive_expr();
        let mut rest = Vec::new();

        let mut next = self.lexer.next_token_peek();
        while matches!(
            next,
            TokenType::LessThan
                | TokenType::LessThanOrEqualTo
                | TokenType::GreaterThan
                | TokenType::GreaterThanOrEqualTo
        ) {
            self.advance();

            let binop = match next {
                TokenType::LessThan => Binop::LessThan,
                TokenType::LessThanOrEqualTo => Binop::LessThanOrEqualTo,
                TokenType::GreaterThan => Binop::GreaterThan,
                _ => Binop::GreaterThanOrEqualTo,
            };

            let next_expr = self.parse_additive_expr();
            rest.push((binop, next_expr));

            next = self.lexer.next_token_peek();
        }

        AstRelationalExpr { first, rest }
    }

    fn parse_additive_expr(&mut self) -> AstAdditiveExpr {
        let first = self.parse_shift_expr();
        let mut rest = Vec::new();

        let mut next = self.lexer.next_token_peek();
        while matches!(next, TokenType::Add | TokenType::Sub) {
            self.advance();

            let binop = match next {
                TokenType::Add => Binop::Add,
                _ => Binop::Sub,
            };

            let next_exp = self.parse_shift_expr();
            rest.push((binop, next_exp));

            next = self.lexer.next_token_peek();
        }

        AstAdditiveExpr { first, rest }
    }

    fn parse_shift_expr(&mut self) -> AstShiftExpr {
        let first = self.parse_term();
        let mut rest = Vec::new();

        let mut next = self.lexer.next_token_peek();
        while matches!(next, TokenType::LShift | TokenType::RShift) {
            self.advance();

            let binop = match next {
                TokenType::LShift => Binop::LShift,
                _ => Binop::RShift,
            };

            let next_exp = self.parse_term();
            rest.push((binop, next_exp));

            next = self.lexer.next_token_peek();
        }

        AstShiftExpr { first, rest }
    }

    fn parse_term(&mut self) -> AstTerm {
        let first = self.parse_factor();
        let mut rest = Vec::new();

        let mut next = self.lexer.next_token_peek();
        while matches!(next, TokenType::Mul | TokenType::Div | TokenType::Mod) {
            self.advance();

            let binop = match next {
                TokenType::Mul => Binop::Mul,
                TokenType::Mod => Binop::Mod,
                _ => Binop::Div,
            };

            let next_factor = self.parse_factor();
            rest.push((binop, next_factor));

            next = self.lexer.next_token_peek();
        }

        AstTerm { first, rest }
    }

    fn parse_factor(&mut self) -> AstFactor {
        match self.advance() {
            TokenType::LParen => {
                let exp = self.parse_expr();
                let _rpar = self.advance_match(|t| *t == TokenType::RParen);
                AstFactor::Paren(Box::new(exp))
            }
            TokenType::Sub => {
                let factor = self.parse_factor();
                AstFactor::Unary {
                    op: Unop::Negation,
                    factor: Box::new(factor),
                }
            }
            TokenType::Add => {
                let factor = self.parse_factor();
                AstFactor::Unary {
                    op: Unop::Addition,
                    factor: Box::new(factor),
                }
            }
            TokenType::BitwiseComplement => {
                let factor = self.parse_factor();
                AstFactor::Unary {
                    op: Unop::BitwiseComplement,
                    factor: Box::new(factor),
                }
            }
            TokenType::LogicalNegation => {
                let factor = self.parse_factor();
                AstFactor::Unary {
                    op: Unop::LogicalNegation,
                    factor: Box::new(factor),
                }
            }
            TokenType::Constant(value) => AstFactor::Int(value),
            _ => unimplemented!("unknown factor token"),
        }
    }
}
