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
    pub first: AstLogicalAndExpr,
    pub rest: Vec<(Binop, AstLogicalAndExpr)>, // binop can only be ||
}

pub struct AstLogicalAndExpr {
    pub first: AstEqualityExpr,
    pub rest: Vec<(Binop, AstEqualityExpr)>, // binop can only be &&
}

pub struct AstEqualityExpr {
    pub first: AstRelationalExpr,
    pub rest: Vec<(Binop, AstRelationalExpr)>, // binop can only be != ==
}

pub struct AstRelationalExpr {
    pub first: AstAdditiveExpr,
    pub rest: Vec<(Binop, AstAdditiveExpr)>, // binop can only be < > <= >=
}

pub struct AstAdditiveExpr {
    pub first: AstTerm,
    pub rest: Vec<(Binop, AstTerm)>, // binop can only be * /
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
