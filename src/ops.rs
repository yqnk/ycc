#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    LogicalAnd,
    LogicalOr,
    EqualTo,
    NotEqualTo,
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LShift,
    RShift,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Unop {
    Addition,
    Negation,
    LogicalNegation,
    BitwiseComplement,
}

impl Binop {
    pub fn precedence(&self) -> u8 {
        match self {
            Binop::LogicalOr => 3,
            Binop::LogicalAnd => 4,
            Binop::BitwiseOr => 5,
            Binop::BitwiseXor => 6,
            Binop::BitwiseAnd => 7,
            Binop::EqualTo | Binop::NotEqualTo => 8,
            Binop::LessThan
            | Binop::LessThanOrEqualTo
            | Binop::GreaterThan
            | Binop::GreaterThanOrEqualTo => 9,
            Binop::LShift | Binop::RShift => 10,
            Binop::Add | Binop::Sub => 11,
            Binop::Mul | Binop::Div | Binop::Mod => 12,
        }
    }

    pub fn as_unop(&self) -> Option<Unop> {
        match self {
            Binop::Add => Some(Unop::Addition),
            Binop::Sub => Some(Unop::Negation),
            _ => None,
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Binop::Add => "+",
            Binop::Sub => "-",
            Binop::Mul => "*",
            Binop::Div => "/",
            Binop::Mod => "%",
            Binop::LogicalOr => "||",
            Binop::LogicalAnd => "&&",
            Binop::EqualTo => "==",
            Binop::NotEqualTo => "!=",
            Binop::LessThan => "<",
            Binop::LessThanOrEqualTo => "<=",
            Binop::GreaterThan => ">",
            Binop::GreaterThanOrEqualTo => ">=",
            Binop::LShift => "<<",
            Binop::RShift => ">>",
            Binop::BitwiseOr => "|",
            Binop::BitwiseXor => "^",
            Binop::BitwiseAnd => "&",
        }
    }
}

impl Unop {
    pub fn symbol(&self) -> &'static str {
        match self {
            Unop::Addition => "+",
            Unop::Negation => "-",
            Unop::LogicalNegation => "!",
            Unop::BitwiseComplement => "~",
        }
    }
}
