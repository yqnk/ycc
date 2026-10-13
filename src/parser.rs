use crate::{
    Binop,
    ast::*,
    lexer::{Keyword, Lexer, TokenType},
};

#[derive(Debug)]
pub struct Parser {
    pub lexer: Lexer,
}

impl Parser {
    pub fn init(file_path: &str) -> Self {
        Self {
            lexer: Lexer::init(file_path),
        }
    }

    pub fn peek(&mut self) -> TokenType {
        self.lexer.next_token_peek()
    }

    pub fn eat(&mut self, expected: &TokenType) -> TokenType {
        let token = self.lexer.next_token_pop();
        if &token != expected {
            panic!("syntax error: expected {expected:?} but got {token:?} while parsing");
        }

        token
    }

    pub fn eat_matching(&mut self, what: &str, pred: impl Fn(&TokenType) -> bool) -> TokenType {
        let token = self.lexer.next_token_pop();
        if !pred(&token) {
            panic!("syntax error: expected {what} but got {token:?} while parsing");
        }

        token
    }

    pub fn eat_if(&mut self, expected: &TokenType) -> bool {
        if &self.peek() == expected {
            self.lexer.next_token_pop();
            return true;
        }

        false
    }

    pub fn parse_program(&mut self) -> AstProgram {
        AstProgram {
            main: self.parse_func(),
        }
    }

    fn parse_type(&mut self) -> Type {
        match self.eat_matching("a type specifier", |t| t.is_type_specifier()) {
            TokenType::Keyword(kw) => Type::from_keyword(&kw),
            _ => unreachable!(),
        }
    }

    fn parse_identifier(&mut self) -> String {
        match self.eat_matching("an identifier", |t| t.is_ident()) {
            TokenType::Identifier(str) => str,
            _ => unreachable!(),
        }
    }

    fn parse_func(&mut self) -> AstFunc {
        let ret_type = self.parse_type();
        let name = self.parse_identifier();

        self.eat(&TokenType::LParen);
        self.eat(&TokenType::RParen);
        self.eat(&TokenType::LBracket);

        let mut blocks = Vec::new();

        loop {
            match self.peek() {
                TokenType::RBracket => break,
                TokenType::Eof => panic!("unterminated function body: missing '}}'"),
                _ => blocks.push(self.parse_block_item()),
            }
        }

        self.eat(&TokenType::RBracket);

        AstFunc {
            ret_type,
            name,
            blocks,
        }
    }

    fn parse_block_item(&mut self) -> AstBlockItem {
        if self.peek().is_type_specifier() {
            return self.parse_vardec();
        }

        AstBlockItem::Statement {
            statement: self.parse_statement(),
        }
    }

    fn parse_vardec(&mut self) -> AstBlockItem {
        let var_type = self.parse_type();
        let name = self.parse_identifier();

        let exp = if self.eat_if(&TokenType::Assign(None)) {
            Some(self.parse_expr())
        } else {
            None
        };

        self.eat(&TokenType::Semi);

        AstBlockItem::VarDec {
            var_type,
            name,
            exp,
        }
    }

    fn parse_statement(&mut self) -> AstStatement {
        match self.peek() {
            TokenType::Keyword(Keyword::Return) => {
                self.eat(&TokenType::Keyword(Keyword::Return));

                let exp = self.parse_expr();
                self.eat(&TokenType::Semi);

                AstStatement::Return { exp }
            }
            TokenType::Keyword(Keyword::If) => {
                self.eat(&TokenType::Keyword(Keyword::If));

                self.eat(&TokenType::LParen);
                let exp = self.parse_expr();
                self.eat(&TokenType::RParen);

                let if_body = self.parse_braced_block_item();

                let else_body = if self.eat_if(&TokenType::Keyword(Keyword::Else)) {
                    Some(Box::new(match self.peek() {
                        TokenType::Keyword(Keyword::If) => AstBlockItem::Statement {
                            statement: self.parse_statement(),
                        },
                        _ => self.parse_braced_block_item(),
                    }))
                } else {
                    None
                };

                AstStatement::Cond {
                    exp,
                    if_body: Box::new(if_body),
                    else_body,
                }
            }
            _ => {
                let exp = self.parse_expr();
                self.eat(&TokenType::Semi);

                AstStatement::Expr { exp }
            }
        }
    }

    fn parse_braced_block_item(&mut self) -> AstBlockItem {
        self.eat(&TokenType::LBracket);
        let body = self.parse_block_item();
        self.eat(&TokenType::RBracket);

        body
    }

    fn parse_expr(&mut self) -> AstExpr {
        let lhs = self.parse_binary(Binop::LOWEST_PRECEDENCE);

        let TokenType::Assign(op) = self.peek() else {
            return lhs;
        };

        self.lexer.next_token_pop();

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

        while let Some(op) = self.peek().as_binop() {
            let power = op.precedence();
            if power < min_power {
                break;
            }
            self.lexer.next_token_pop();

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
        let token = self.lexer.next_token_pop();

        if let Some(op) = token.as_unop() {
            return AstExpr::Unary {
                op,
                operand: Box::new(self.parse_factor()),
            };
        }

        match token {
            TokenType::LParen => {
                let exp = self.parse_expr();
                self.eat(&TokenType::RParen);
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
