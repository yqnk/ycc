use super::nodes::{AstExpr, AstFunc, AstProgram, AstStatement};
use super::visitor::Visitor;
use std::io::Write;

#[derive(Default)]
pub struct AstDotPrinter {
    dot: Vec<u8>,
}

impl AstDotPrinter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pretty_print(&mut self, program: &AstProgram) {
        program.accept(self);
    }
}

impl Visitor for AstDotPrinter {
    type Program = ();
    type Func = ();
    type Statement = ();
    type Expr = ();

    fn visit_program(&mut self, program: &AstProgram) -> Self::Program {
        let _ = writeln!(self.dot, "graph program {{");
        program.main.accept(self);
        let _ = writeln!(self.dot, "}}");
    }

    fn visit_func(&mut self, func: &AstFunc) -> Self::Func {
        todo!()
    }

    fn visit_statement(&mut self, statement: &AstStatement) -> Self::Statement {
        todo!()
    }

    fn visit_expr(&mut self, expr: &AstExpr) -> Self::Expr {
        todo!()
    }
}
