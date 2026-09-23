use super::nodes::{AstExpr, AstFactor, AstFunc, AstProgram, AstStatement, AstTerm};

pub trait Visitor {
    type Program;
    type Func;
    type Statement;
    type Expr;
    type Term;
    type Factor;

    fn visit_program(&mut self, program: &AstProgram) -> Self::Program;
    fn visit_func(&mut self, func: &AstFunc) -> Self::Func;
    fn visit_statement(&mut self, statement: &AstStatement) -> Self::Statement;
    fn visit_expr(&mut self, expr: &AstExpr) -> Self::Expr;
    fn visit_term(&mut self, term: &AstTerm) -> Self::Term;
    fn visit_factor(&mut self, factor: &AstFactor) -> Self::Factor;
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

impl AstTerm {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Term {
        visitor.visit_term(self)
    }
}

impl AstFactor {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Factor {
        visitor.visit_factor(self)
    }
}
