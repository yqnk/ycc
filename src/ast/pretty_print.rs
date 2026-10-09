use crate::ast::Binop;

use super::nodes::{AstExpr, AstFactor, AstFunc, AstProgram, AstStatement, AstTerm};
use super::visitor::Visitor;

macro_rules! iprintln {
    ($astpp:ident, $body:expr) => {
        println!("{}{}", " ".repeat($astpp.indent * 2), $body)
    };
}

#[derive(Default)]
pub struct AstPrettyPrinter {
    indent: usize,
}

impl AstPrettyPrinter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pretty_print(&mut self, program: &AstProgram) {
        self.indent = 0;

        program.accept(self);
    }

    fn nested(&mut self, body: impl FnOnce(&mut Self)) {
        self.indent += 1;
        body(self);
        self.indent -= 1;
    }
}

impl Visitor for AstPrettyPrinter {
    type Program = ();
    type Func = ();
    type Statement = ();
    type Expr = ();
    type LogicalAndExpr = ();
    type EqualityExpr = ();
    type RelationalExpr = ();
    type AdditiveExpr = ();
    type Term = ();
    type Factor = ();

    fn visit_program(&mut self, program: &AstProgram) -> Self::Program {
        iprintln!(self, "PROGRAM");
        self.nested(|pp| program.main.accept(pp));
    }

    fn visit_func(&mut self, func: &AstFunc) -> Self::Func {
        iprintln!(self, format!("FUN {:?} {}", func.ret_type, func.name));
        iprintln!(self, "body:");

        self.nested(|pp| func.statement.accept(pp));
    }

    fn visit_statement(&mut self, statement: &AstStatement) -> Self::Statement {
        match statement {
            AstStatement::Return { exp } => {
                iprintln!(self, "RETURN (");
                self.nested(|pp| exp.accept(pp));
                iprintln!(self, ")");
            }
        }
    }

    fn visit_expr(&mut self, expr: &AstExpr) -> Self::Expr {
        iprintln!(self, "EXPR (");
        self.nested(|pp| {
            expr.first.accept(pp);
            for (op, laexp) in &expr.rest {
                assert!(op.eq(&Binop::LogicalOr));
                iprintln!(pp, format!("BINOP {}", op.symbol()));
                laexp.accept(pp);
            }
        });
        iprintln!(self, ")");
    }

    fn visit_logical_and_expr(
        &mut self,
        expr: &super::nodes::AstLogicalAndExpr,
    ) -> Self::LogicalAndExpr {
        iprintln!(self, "LOGICAL_AND_EXPR (");
        self.nested(|pp| {
            expr.first.accept(pp);
            for (op, eqexpr) in &expr.rest {
                assert!(op.eq(&Binop::LogicalAnd));
                iprintln!(pp, format!("BINOP {}", op.symbol()));
                eqexpr.accept(pp);
            }
        });
        iprintln!(self, ")");
    }

    fn visit_equality_expr(&mut self, expr: &super::nodes::AstEqualityExpr) -> Self::EqualityExpr {
        iprintln!(self, "EQUALITY_EXPR (");
        self.nested(|pp| {
            expr.first.accept(pp);
            for (op, relexpr) in &expr.rest {
                assert!(matches!(op, Binop::EqualTo | Binop::NotEqualTo));
                iprintln!(pp, format!("BINOP {}", op.symbol()));
                relexpr.accept(pp);
            }
        });
        iprintln!(self, ")");
    }

    fn visit_relational_expr(
        &mut self,
        expr: &super::nodes::AstRelationalExpr,
    ) -> Self::RelationalExpr {
        iprintln!(self, "RELATIONAL_EXPR (");
        self.nested(|pp| {
            expr.first.accept(pp);
            for (op, addexpr) in &expr.rest {
                assert!(matches!(
                    op,
                    Binop::LessThan
                        | Binop::GreaterThan
                        | Binop::LessThanOrEqualTo
                        | Binop::GreaterThanOrEqualTo
                ));
                iprintln!(pp, format!("BINOP {}", op.symbol()));
                addexpr.accept(pp);
            }
        });
        iprintln!(self, ")");
    }

    fn visit_additive_expr(&mut self, expr: &super::nodes::AstAdditiveExpr) -> Self::AdditiveExpr {
        iprintln!(self, "ADDITIVE_EXPR (");
        self.nested(|pp| {
            expr.first.accept(pp);
            for (op, term) in &expr.rest {
                assert!(matches!(op, Binop::Add | Binop::Sub));
                iprintln!(pp, format!("BINOP {}", op.symbol()));
                term.accept(pp);
            }
        });
        iprintln!(self, ")");
    }

    fn visit_term(&mut self, term: &AstTerm) -> Self::Term {
        iprintln!(self, "TERM (");
        self.nested(|pp| {
            term.first.accept(pp);
            for (op, factor) in &term.rest {
                iprintln!(pp, format!("BINOP {}", op.symbol()));
                factor.accept(pp);
            }
        });
        iprintln!(self, ")");
    }

    fn visit_factor(&mut self, factor: &AstFactor) -> Self::Factor {
        match factor {
            AstFactor::Paren(exp) => {
                iprintln!(self, "PAREN (");
                self.nested(|pp| exp.accept(pp));
                iprintln!(self, ")");
            }
            AstFactor::Unary { op, factor } => {
                iprintln!(self, format!("UNOP {} (", op.symbol()));
                self.nested(|pp| factor.accept(pp));
                iprintln!(self, ")");
            }
            AstFactor::Int(value) => {
                iprintln!(self, format!("INT {}", value));
            }
        }
    }
}
