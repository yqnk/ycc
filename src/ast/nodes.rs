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
    pub first: AstBitwiseOrExpr,
    pub rest: Vec<(Binop, AstBitwiseOrExpr)>, // binop can only be &&
}

pub struct AstBitwiseOrExpr {
    pub first: AstBitwiseXorExpr,
    pub rest: Vec<(Binop, AstBitwiseXorExpr)>,
}

pub struct AstBitwiseXorExpr {
    pub first: AstBitwiseAndExpr,
    pub rest: Vec<(Binop, AstBitwiseAndExpr)>,
}

pub struct AstBitwiseAndExpr {
    pub first: AstEqualityExpr,
    pub rest: Vec<(Binop, AstEqualityExpr)>,
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
    pub first: AstShiftExpr,
    pub rest: Vec<(Binop, AstShiftExpr)>, // binop can only be * /
}

pub struct AstShiftExpr {
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
