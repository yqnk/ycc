use crate::ast::{AstBitwiseAndExpr, AstBitwiseOrExpr, AstBitwiseXorExpr, AstShiftExpr};

use super::nodes::{
    AstAdditiveExpr, AstEqualityExpr, AstExpr, AstFactor, AstFunc, AstLogicalAndExpr, AstProgram,
    AstRelationalExpr, AstStatement, AstTerm,
};

pub trait Visitor {
    type Program;
    type Func;
    type Statement;

    type Expr;
    type LogicalAndExpr;
    type BitwiseOrExpr;
    type BitwiseXorExpr;
    type BitwiseAndExpr;
    type EqualityExpr;
    type RelationalExpr;
    type AdditiveExpr;
    type ShiftExpr;

    type Term;
    type Factor;

    fn visit_program(&mut self, program: &AstProgram) -> Self::Program;
    fn visit_func(&mut self, func: &AstFunc) -> Self::Func;
    fn visit_statement(&mut self, statement: &AstStatement) -> Self::Statement;
    fn visit_expr(&mut self, expr: &AstExpr) -> Self::Expr;
    fn visit_logical_and_expr(&mut self, expr: &AstLogicalAndExpr) -> Self::LogicalAndExpr;
    fn visit_bitwise_or_expr(&mut self, expr: &AstBitwiseOrExpr) -> Self::BitwiseOrExpr;
    fn visit_bitwise_xor_expr(&mut self, expr: &AstBitwiseXorExpr) -> Self::BitwiseXorExpr;
    fn visit_bitwise_and_expr(&mut self, expr: &AstBitwiseAndExpr) -> Self::BitwiseAndExpr;
    fn visit_equality_expr(&mut self, expr: &AstEqualityExpr) -> Self::EqualityExpr;
    fn visit_relational_expr(&mut self, expr: &AstRelationalExpr) -> Self::RelationalExpr;
    fn visit_additive_expr(&mut self, expr: &AstAdditiveExpr) -> Self::AdditiveExpr;
    fn visit_shift_expr(&mut self, expr: &AstShiftExpr) -> Self::ShiftExpr;
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

impl AstLogicalAndExpr {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::LogicalAndExpr {
        visitor.visit_logical_and_expr(self)
    }
}

impl AstBitwiseOrExpr {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::BitwiseOrExpr {
        visitor.visit_bitwise_or_expr(self)
    }
}

impl AstBitwiseXorExpr {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::BitwiseXorExpr {
        visitor.visit_bitwise_xor_expr(self)
    }
}

impl AstBitwiseAndExpr {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::BitwiseAndExpr {
        visitor.visit_bitwise_and_expr(self)
    }
}

impl AstEqualityExpr {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::EqualityExpr {
        visitor.visit_equality_expr(self)
    }
}

impl AstRelationalExpr {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::RelationalExpr {
        visitor.visit_relational_expr(self)
    }
}
impl AstAdditiveExpr {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::AdditiveExpr {
        visitor.visit_additive_expr(self)
    }
}

impl AstShiftExpr {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::ShiftExpr {
        visitor.visit_shift_expr(self)
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
