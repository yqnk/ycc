use super::Type;

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

pub enum AstExpr {
    Int { value: i32 },
}
