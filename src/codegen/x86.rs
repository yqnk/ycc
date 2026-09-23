use std::fmt::Write;

use crate::ast::{
    AstExpr, AstFactor, AstFunc, AstProgram, AstStatement, AstTerm, Binop, Unop, Visitor,
};

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
    type Term = ();
    type Factor = ();

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

    fn visit_expr(&mut self, expr: &AstExpr) -> Self::Expr {
        expr.first.accept(self); // lhs in rax

        for (op, term) in &expr.rest {
            self.instr("pushq", "%rax"); // push lhs
            term.accept(self); // save rhs in rax
            self.instr("movq", "%rax, %rcx"); // move rhs in rcx
            self.instr("popq", "%rax"); // get lhs back in rax

            match op {
                Binop::Add => self.instr("add", "%rcx, %rax"),
                Binop::Sub => self.instr("sub", "%rcx, %rax"),
                _ => unreachable!(),
            }
        }
    }

    fn visit_term(&mut self, term: &AstTerm) -> Self::Term {
        term.first.accept(self);

        for (op, factor) in &term.rest {
            self.instr("pushq", "%rax");
            factor.accept(self);
            self.instr("movq", "%rax, %rcx");
            self.instr("popq", "%rax");

            match op {
                Binop::Mul => self.instr("imul", "%rcx"),
                Binop::Div => self.instr("idiv", "%rcx"),
                _ => unreachable!(),
            }
        }
    }

    fn visit_factor(&mut self, factor: &AstFactor) -> Self::Factor {
        match factor {
            AstFactor::Paren(exp) => exp.accept(self),
            AstFactor::Unary { op, factor } => {
                factor.accept(self);

                match op {
                    Unop::Negation => self.instr("negq", "%rax"),
                    Unop::LogicalNegation => {
                        self.instr("cmpq", "$0, %rax");
                        self.instr("movq", "$0, %rax");
                        self.instr("sete", "%al");
                    }
                    Unop::BitwiseComplement => self.instr("notq", "%rax"),
                    _ => (),
                }
            }
            AstFactor::Int(v) => {
                self.instr("movq", &format!("${}, %rax", v));
            }
        }
    }
}
