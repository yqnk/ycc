use super::nodes::{AstExpr, AstFunc, AstProgram, AstStatement};

pub trait Visitor {
    type Program;
    type Func;
    type Statement;
    type Expr;

    fn visit_program(&mut self, program: &AstProgram) -> Self::Program;
    fn visit_func(&mut self, func: &AstFunc) -> Self::Func;
    fn visit_statement(&mut self, statement: &AstStatement) -> Self::Statement;
    fn visit_expr(&mut self, expr: &AstExpr) -> Self::Expr;
}

impl AstProgram {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Program {
        visitor.visit_program(self)
    }
}

impl AstFunc {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Func {
        visitor.visit_func(self)
    }
}

impl AstStatement {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Statement {
        visitor.visit_statement(self)
    }
}

impl AstExpr {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Expr {
        visitor.visit_expr(self)
    }
}
