use crate::ast::AstBlockItem;

use super::nodes::{AstExpr, AstFunc, AstProgram, AstStatement};
use super::visitor::Visitor;

macro_rules! iprintln {
    ($astpp:ident, $body:expr) => {
        println!("{}{}", " ".repeat($astpp.indent * 4), $body)
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
    type BlockItem = ();

    fn visit_program(&mut self, program: &AstProgram) -> Self::Program {
        iprintln!(self, "PROGRAM");
        self.nested(|pp| program.main.accept(pp));
    }

    fn visit_func(&mut self, func: &AstFunc) -> Self::Func {
        iprintln!(self, format!("FUN {:?} {}", func.ret_type, func.name));
        iprintln!(self, "body:");

        self.nested(|pp| {
            for item in &func.body {
                item.accept(pp);
            }
        });
    }

    fn visit_statement(&mut self, statement: &AstStatement) -> Self::Statement {
        match statement {
            AstStatement::Return { exp } => {
                iprintln!(self, "RETURN (");
                self.nested(|pp| exp.accept(pp));
                iprintln!(self, ")");
            }
            AstStatement::Expr { exp } => {
                iprintln!(self, "EXPR (");
                self.nested(|pp| exp.accept(pp));
                iprintln!(self, ")");
            }
            AstStatement::Cond {
                exp,
                if_body,
                else_body,
            } => {
                iprintln!(self, "COND (");
                self.nested(|pp| {
                    iprintln!(pp, "cond:");
                    exp.accept(pp)
                });
                self.nested(|pp| {
                    iprintln!(pp, "then body:");
                    if_body.accept(pp);
                });
                if let Some(body) = else_body {
                    iprintln!(self, "else body:");
                    self.nested(|pp| body.accept(pp));
                }
                iprintln!(self, ")");
            }
            AstStatement::Block { items } => {
                iprintln!(self, "BLOCK {");
                self.nested(|pp| {
                    items.iter().for_each(|i| i.accept(pp));
                });
                iprintln!(self, "}");
            }
        }
    }

    fn visit_expr(&mut self, expr: &AstExpr) -> Self::Expr {
        match expr {
            AstExpr::Binary { op, lhs, rhs } => {
                iprintln!(self, format!("BINOP {} (", op.symbol()));
                self.nested(|pp| {
                    lhs.accept(pp);
                    rhs.accept(pp);
                });
                iprintln!(self, ")");
            }
            AstExpr::Unary { op, operand } => {
                iprintln!(self, format!("UNOP {} (", op.symbol()));
                self.nested(|pp| operand.accept(pp));
                iprintln!(self, ")");
            }
            AstExpr::Conditional {
                cond,
                then_exp,
                else_exp,
            } => {
                iprintln!(self, "CONDITIONAL (");
                self.nested(|pp| {
                    iprintln!(pp, "cond:");
                    cond.accept(pp);
                    iprintln!(pp, "then:");
                    then_exp.accept(pp);
                    iprintln!(pp, "else:");
                    else_exp.accept(pp);
                });
                iprintln!(self, ")");
            }
            AstExpr::Int(value) => {
                iprintln!(self, format!("INT {}", value));
            }
            AstExpr::Var(name) => {
                iprintln!(self, format!("VAR {}", name));
            }
            AstExpr::Assignment { op, lhs, rhs } => {
                match op {
                    Some(op) => iprintln!(self, format!("ASSIGN {} {}=", lhs, op.symbol())),
                    None => iprintln!(self, format!("ASSIGN {} =", lhs)),
                }
                self.nested(|pp| rhs.accept(pp));
            }
        }
    }

    fn visit_blockitem(&mut self, expr: &super::AstBlockItem) -> Self::BlockItem {
        match expr {
            AstBlockItem::Statement { statement } => statement.accept(self),
            AstBlockItem::VarDec {
                var_type,
                name,
                exp,
            } => {
                iprintln!(self, "VARDEC (");
                self.nested(|pp| {
                    iprintln!(pp, format!("{:?} {}", var_type, name));
                    if let Some(e) = exp {
                        iprintln!(pp, "=");
                        e.accept(pp);
                    }
                });

                iprintln!(self, ")");
            }
        }
    }
}
