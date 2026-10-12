use super::{Binop, Type, Unop};

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

// Precedence is encoded by the tree shape, not by one node type per level
pub enum AstExpr {
    Assignment {
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
    Int(i32),
}
