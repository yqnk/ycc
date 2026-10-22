use crate::{Binop, Unop};

use super::Type;

pub struct AstProgram {
    pub main: AstFunc,
}

pub struct AstFunc {
    pub ret_type: Type,
    pub name: String,
    pub body: Vec<AstStatement>,
}

pub enum AstStatement {
    Return {
        exp: AstExpr,
    },
    Simple {
        exp: AstExpr,
    },
    If {
        cond: AstExpr,
        then_stmt: Box<AstStatement>,
        else_stmt: Option<Box<AstStatement>>,
    },
    Compound {
        statements: Vec<AstStatement>,
    },
    Declaration {
        var_type: Type,
        name: String,
        exp: Option<AstExpr>,
    },
}

#[derive(Debug)]
pub enum AstExpr {
    Assignment {
        op: Option<Binop>,
        lhs: String,
        rhs: Box<AstExpr>,
    },
    Binary {
        op: Binop,
        lhs: Box<AstExpr>,
        rhs: Box<AstExpr>,
    },
    Unary {
        op: Unop,
        operand: Box<AstExpr>,
    },
    Conditional {
        cond: Box<AstExpr>,
        then_exp: Box<AstExpr>,
        else_exp: Box<AstExpr>,
    },
    Var(String),
    Int(i32),
}
