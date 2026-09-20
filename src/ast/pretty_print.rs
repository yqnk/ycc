use super::nodes::{AstExpr, AstFunc, AstProgram, AstStatement};
use super::visitor::Visitor;

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

    pub fn iprintln(&self, str: &str) {
        println!("{}{}", "\t".repeat(self.indent), str);
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
        self.iprintln(&format!("FUN {:?} {}", func.ret_type, func.name));
        self.iprintln(&format!("body:"));
        self.indent += 1;
        func.statement.accept(self);
        self.indent -= 1;
    }

    fn visit_statement(&mut self, statement: &AstStatement) -> Self::Statement {
        match statement {
            AstStatement::Return { exp } => {
                let value = exp.accept(self);
                self.iprintln(&format!("RETURN {}", value));
            }
        }
    }

    fn visit_expr(&mut self, expr: &AstExpr) -> Self::Expr {
        match expr {
            AstExpr::Int { value } => format!("{}", value),
        }
    }
}
