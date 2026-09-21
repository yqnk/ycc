use std::fmt::Write;

use crate::ast::{AstExpr, AstFunc, AstProgram, AstStatement, Visitor};

#[derive(Default)]
pub struct X86Backend {
    out: String,
}

impl X86Backend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn emit_program(&mut self, program: &AstProgram) -> String {
        program.accept(self)
    }

    fn directive(&mut self, text: &str) {
        writeln!(self.out, "\t{}", text).unwrap();
    }

    fn label(&mut self, name: &str) {
        writeln!(self.out, "{}:", name).unwrap();
    }

    fn instr(&mut self, mnemonic: &str, operands: &str) {
        if operands.is_empty() {
            writeln!(self.out, "\t{}", mnemonic).unwrap();
        } else {
            writeln!(self.out, "\t{}\t{}", mnemonic, operands).unwrap();
        }
    }
}

impl Visitor for X86Backend {
    type Program = String;
    type Func = ();
    type Statement = ();
    type Expr = ();

    fn visit_program(&mut self, program: &AstProgram) -> String {
        self.out.clear();
        program.main.accept(self);

        std::mem::take(&mut self.out)
    }

    fn visit_func(&mut self, func: &AstFunc) {
        self.directive(&format!(".globl {}", func.name));
        self.label(&func.name);
        func.statement.accept(self);
    }

    fn visit_statement(&mut self, statement: &AstStatement) {
        match statement {
            AstStatement::Return { exp } => {
                exp.accept(self);
                self.instr("ret", "");
            }
        }
    }

    fn visit_expr(&mut self, expr: &AstExpr) {
        match expr {
            AstExpr::Int { value } => {
                self.instr("movl", &format!("${}, %eax", value));
            }
            AstExpr::Unop { str, exp } => {
                exp.accept(self);
                match str.as_str() {
                    "-" => self.instr("negl", "%eax"),
                    "~" => self.instr("notl", "%eax"),
                    "!" => {
                        self.instr("cmpl", "$0, %eax");
                        self.instr("movl", "$0, %eax");
                        self.instr("sete", "%al");
                    }
                    o => unimplemented!("unop {}", o),
                }
            }
        }
    }
}
