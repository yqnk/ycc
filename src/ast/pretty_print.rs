use super::nodes::{AstExpr, AstFunc, AstProgram, AstStatement};
use super::visitor::Visitor;

macro_rules! iprintln {
    ($astpp:ident, $body:expr) => {
        println!("{}{}", " ".repeat($astpp.indent * 2), $body);
    };
}

#[derive(Default)]
pub struct AstPrettyPrinter {
    indent: usize,
}

impl AstPrettyPrinter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pretty_print(&mut self, program: &AstProgram) {
        self.indent = 0;

        program.accept(self);
    }
}

impl Visitor for AstPrettyPrinter {
    type Program = ();
    type Func = ();
    type Statement = ();
    type Expr = String;

    fn visit_program(&mut self, program: &AstProgram) -> Self::Program {
        program.main.accept(self)
    }

    fn visit_func(&mut self, func: &AstFunc) -> Self::Func {
        iprintln!(self, &format!("FUN {:?} {}", func.ret_type, func.name));
        iprintln!(self, &format!("body:"));

        self.indent += 1;
        func.statement.accept(self);
        self.indent -= 1;
    }

    fn visit_statement(&mut self, statement: &AstStatement) -> Self::Statement {
        match statement {
            AstStatement::Return { exp } => {
                let value = exp.accept(self);
                iprintln!(self, &format!("RETURN {}", value));
            }
        }
    }

    fn visit_expr(&mut self, expr: &AstExpr) -> Self::Expr {
        match expr {
            AstExpr::Int { value } => format!("Int({})", value),
            AstExpr::Unop { str, exp } => {
                let value = exp.accept(self);
                format!("Unop({}, {})", str, value)
            }
        }
    }
}
