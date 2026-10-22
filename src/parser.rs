use crate::{
    ast::*,
    lexer::{Keyword, LOWEST_PRECEDENCE, Lexer, Token, TokenType},
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
        self.lexer.peek_token().r#type
    }

    pub fn loc(&self, token: &Token) -> String {
        self.lexer.source.fmt_span(token.span)
    }

    pub fn eat(&mut self, expected: &TokenType) -> Token {
        let token = self.lexer.token_pop();
        if &token.r#type != expected {
            panic!(
                "{}: syntax error: expected {expected:?} but got {:?}",
                self.loc(&token),
                token.r#type
            );
        }

        token
    }

    pub fn eat_matching(&mut self, what: &str, pred: impl Fn(&TokenType) -> bool) -> Token {
        let token = self.lexer.token_pop();
        if !pred(&token.r#type) {
            panic!(
                "{}: syntax error: expected {what} but got {:?}",
                self.loc(&token),
                token.r#type
            );
        }

        token
    }

    pub fn eat_if(&mut self, expected: &TokenType) -> bool {
        if &self.peek() == expected {
            self.lexer.token_pop();
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
        match self
            .eat_matching("a type specifier", |t| t.is_type_specifier())
            .r#type
        {
            TokenType::Keyword(kw) => Type::from_keyword(&kw),
            _ => unreachable!(),
        }
    }

    fn parse_identifier(&mut self) -> String {
        match self.eat_matching("an identifier", |t| t.is_ident()).r#type {
            TokenType::Identifier(str) => str,
            _ => unreachable!(),
        }
    }

    fn parse_func(&mut self) -> AstFunc {
        let ret_type = self.parse_type();
        let name = self.parse_identifier();

        self.eat(&TokenType::LParen);
        // parse params
        self.eat(&TokenType::RParen);

        let body = self.parse_block_items("function body");

        AstFunc {
            ret_type,
            name,
            body,
        }
    }

    fn parse_block_items(&mut self, what: &str) -> Vec<AstBlockItem> {
        self.eat(&TokenType::LBrace);

        let mut items = Vec::new();

        loop {
            match self.peek() {
                TokenType::RBrace => break,
                TokenType::Eof => {
                    let eof = self.lexer.peek_token();
                    panic!("{}: unterminated {what}: missing '}}'", self.loc(&eof))
                }
                _ => items.push(self.parse_block_item()),
            }
        }

        self.eat(&TokenType::RBrace);

        items
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
            Some(self.parse_expr(LOWEST_PRECEDENCE))
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

    fn parse_block_statement(&mut self) -> AstStatement {
        AstStatement::Block {
            items: self.parse_block_items("block"),
        }
    }

    fn parse_statement(&mut self) -> AstStatement {
        match self.peek() {
            TokenType::Keyword(Keyword::Return) => {
                self.eat(&TokenType::Keyword(Keyword::Return));

                let exp = self.parse_expr(LOWEST_PRECEDENCE);
                self.eat(&TokenType::Semi);

                AstStatement::Return { exp }
            }
            TokenType::Keyword(Keyword::If) => {
                self.eat(&TokenType::Keyword(Keyword::If));

                self.eat(&TokenType::LParen);
                let exp = self.parse_expr(LOWEST_PRECEDENCE);
                self.eat(&TokenType::RParen);

                let if_body = self.parse_statement();

                let else_body = if self.eat_if(&TokenType::Keyword(Keyword::Else)) {
                    Some(Box::new(self.parse_statement()))
                } else {
                    None
                };

                AstStatement::Cond {
                    exp,
                    if_body: Box::new(if_body),
                    else_body,
                }
            }
            TokenType::LBrace => self.parse_block_statement(),
            _ => {
                let exp = self.parse_expr(LOWEST_PRECEDENCE);
                self.eat(&TokenType::Semi);

                AstStatement::Expr { exp }
            }
        }
    }

    fn parse_expr(&mut self, min_power: u8) -> AstExpr {
        let mut lhs = self.parse_factor();

        while let Some(power) = self.peek().infix_precedence() {
            if power < min_power {
                break;
            }

            let token = self.lexer.token_pop();
            lhs = match token.r#type {
                TokenType::Assign(op) => {
                    let AstExpr::Var(name) = lhs else {
                        panic!("expected variable on the left of an assignment, got: {lhs:?}",);
                    };

                    AstExpr::Assignment {
                        op,
                        lhs: name,
                        rhs: Box::new(self.parse_expr(power)),
                    }
                }
                TokenType::QuestionMark => {
                    let then_exp = self.parse_expr(LOWEST_PRECEDENCE);
                    self.eat(&TokenType::Colon);

                    AstExpr::Conditional {
                        cond: Box::new(lhs),
                        then_exp: Box::new(then_exp),
                        else_exp: Box::new(self.parse_expr(power)),
                    }
                }
                TokenType::Binop(op) => AstExpr::Binary {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expr(power + 1)),
                },
                _ => unreachable!(),
            };
        }

        lhs
    }

    fn parse_factor(&mut self) -> AstExpr {
        let token = self.lexer.token_pop();

        if let Some(op) = token.r#type.as_unop() {
            return AstExpr::Unary {
                op,
                operand: Box::new(self.parse_factor()),
            };
        }

        match token.r#type {
            TokenType::LParen => {
                let exp = self.parse_expr(LOWEST_PRECEDENCE);
                self.eat(&TokenType::RParen);
                exp
            }
            TokenType::Constant(value) => AstExpr::Int(value),
            TokenType::Identifier(name) => AstExpr::Var(name),
            ref t => {
                unimplemented!(
                    "{}: unknown factor token: {:?}",
                    self.lexer.source.fmt_span(token.span),
                    t
                )
            }
        }
    }
}
