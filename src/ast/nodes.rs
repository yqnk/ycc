use core::fmt;

use super::{Binop, Type, Unop};

pub struct AstProgram {
    pub main: AstFunc,
}

pub struct AstFunc {
    pub ret_type: Type,
    pub name: String,
    pub statement: AstStatement,
}

pub enum AstStatement {
    Return { exp: AstExpr },
}

pub struct AstExpr {
    pub first: AstTerm,
    pub rest: Vec<(Binop, AstTerm)>,
}

pub struct AstTerm {
    pub first: AstFactor,
    pub rest: Vec<(Binop, AstFactor)>,
}

pub enum AstFactor {
    Paren(Box<AstExpr>),
    Unary { op: Unop, factor: Box<AstFactor> },
    Int(i32),
}
