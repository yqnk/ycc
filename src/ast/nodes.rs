use crate::{Binop, Unop};

use super::Type;

pub struct AstProgram {
    pub main: AstFunc,
}

pub enum AstBlockItem {
    Statement {
        statement: AstStatement,
    },
    VarDec {
        var_type: Type,
        name: String,
        exp: Option<AstExpr>,
    },
}

pub struct AstFunc {
    pub ret_type: Type,
    pub name: String,
    pub body: Vec<AstBlockItem>,
}

pub enum AstStatement {
    Return {
        exp: AstExpr,
    },
    Expr {
        exp: AstExpr,
    },
    Cond {
        exp: AstExpr,
        if_body: Box<AstStatement>,
        else_body: Option<Box<AstStatement>>,
    },
    Block {
        items: Vec<AstBlockItem>,
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
