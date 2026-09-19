use crate::ast::Type;

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

pub trait AstPrettyPrint {
    fn pretty_print(&self, indent: usize);

    fn print(&self) {
        self.pretty_print(0);
    }
}

impl AstPrettyPrint for AstProgram {
    fn pretty_print(&self, indent: usize) {
        self.main.pretty_print(indent);
    }
}

impl AstPrettyPrint for AstFunc {
    fn pretty_print(&self, indent: usize) {
        println!("{}FUN {:?} {}:", "\t".repeat(indent), self.ret_type, self.name);
        println!("{}body:", "\t".repeat(indent + 1));

        self.statement.pretty_print(indent + 2);
    }
}

impl AstPrettyPrint for AstStatement {
    fn pretty_print(&self, indent: usize) {
        match self {
            AstStatement::Return { exp } => {
                print!("{}RETURN ", "\t".repeat(indent));
                exp.pretty_print(indent);
            }
        }
    }
}

impl AstPrettyPrint for AstExpr {
    fn pretty_print(&self, _indent: usize) {
        match self {
            AstExpr::Int { value } => {
                print!("Int<{}>", value)
            }
        }
    }
}
