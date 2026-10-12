use crate::{Binop, Unop};

use super::Type;

pub struct AstProgram {
    pub main: AstFunc,
}

pub struct AstFunc {
    pub ret_type: Type,
    pub name: String,
    pub statements: Vec<AstStatement>,
}

pub enum AstStatement {
    Return {
        exp: AstExpr,
    },
    Expr {
        exp: AstExpr,
    },
    VarDec {
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
    Var(String),
    Int(i32),
}
