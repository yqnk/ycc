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
    pub const ASSIGNMENT_PRECEDENCE: u8 = 1;
    pub const LOWEST_PRECEDENCE: u8 = Self::ASSIGNMENT_PRECEDENCE + 1;

    pub fn precedence(&self) -> u8 {
        match self {
            Binop::LogicalOr => 2,
            Binop::LogicalAnd => 3,
            Binop::BitwiseOr => 4,
            Binop::BitwiseXor => 5,
            Binop::BitwiseAnd => 6,
            Binop::EqualTo | Binop::NotEqualTo => 7,
            Binop::LessThan
            | Binop::LessThanOrEqualTo
            | Binop::GreaterThan
            | Binop::GreaterThanOrEqualTo => 8,
            Binop::LShift | Binop::RShift => 9,
            Binop::Add | Binop::Sub => 10,
            Binop::Mul | Binop::Div | Binop::Mod => 11,
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
