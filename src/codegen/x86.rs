use std::fmt::Write;

use crate::ast::{AstExpr, AstFunc, AstProgram, AstStatement, Binop, Unop, Visitor};

#[derive(Default)]
pub struct X86Backend {
    out: String,

    label_count: u32,
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

    fn next_label_id(&mut self) -> u32 {
        let id = self.label_count;
        self.label_count += 1;
        id
    }

    fn instr(&mut self, mnemonic: &str, operands: &str) {
        if operands.is_empty() {
            writeln!(self.out, "\t{}", mnemonic).unwrap();
        } else {
            writeln!(self.out, "\t{}\t{}", mnemonic, operands).unwrap();
        }
    }

    // && and || are lazy evaluated
    fn short_circuit(&mut self, is_or: bool, lhs: &AstExpr, rhs: &AstExpr) {
        let id = self.next_label_id();
        let (name, jump, shortcut_value) = if is_or {
            ("or", "jne", 1)
        } else {
            ("and", "je", 0)
        };
        let shortcut_label = format!(".L{}_shortcut_{}", name, id);
        let end_label = format!(".L{}_end_{}", name, id);

        lhs.accept(self);
        self.instr("cmpq", "$0, %rax");
        self.instr(jump, &shortcut_label);

        rhs.accept(self);
        self.instr("cmpq", "$0, %rax");
        self.instr(jump, &shortcut_label);

        // not lazy evaluated, so right most exp is the result
        self.instr("movq", &format!("${}, %rax", 1 - shortcut_value));
        self.instr("jmp", &end_label);

        self.label(&shortcut_label);
        self.instr("movq", &format!("${}, %rax", shortcut_value));

        self.label(&end_label);
    }

    fn emit_binop(&mut self, op: &Binop) {
        match op {
            Binop::Add => self.instr("addq", "%rcx, %rax"),
            Binop::Sub => self.instr("subq", "%rcx, %rax"),
            Binop::BitwiseOr => self.instr("orq", "%rcx, %rax"),
            Binop::BitwiseXor => self.instr("xorq", "%rcx, %rax"),
            Binop::BitwiseAnd => self.instr("andq", "%rcx, %rax"),
            Binop::LShift => self.instr("shlq", "%cl, %rax"),
            Binop::RShift => self.instr("sarq", "%cl, %rax"),
            Binop::Mul => self.instr("imulq", "%rcx"),
            Binop::Div => {
                self.instr("cqto", "");
                self.instr("idivq", "%rcx");
            }
            Binop::Mod => {
                self.instr("cqto", "");
                self.instr("idivq", "%rcx");
                self.instr("movq", "%rdx, %rax");
            }
            Binop::LogicalAnd | Binop::LogicalOr => {
                unreachable!("short-circuiting operators are emitted by short_circuit")
            }
            cmp => {
                self.instr("cmpq", "%rcx, %rax");
                let setcc = match cmp {
                    Binop::EqualTo => "sete",
                    Binop::NotEqualTo => "setne",
                    Binop::LessThan => "setl",
                    Binop::LessThanOrEqualTo => "setle",
                    Binop::GreaterThan => "setg",
                    Binop::GreaterThanOrEqualTo => "setge",
                    _ => unreachable!(),
                };
                self.instr(setcc, "%al");
                self.instr("movzbl", "%al, %eax");
            }
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

    fn visit_expr(&mut self, expr: &AstExpr) -> Self::Expr {
        match expr {
            AstExpr::Binary {
                op: op @ (Binop::LogicalAnd | Binop::LogicalOr),
                lhs,
                rhs,
            } => self.short_circuit(*op == Binop::LogicalOr, lhs, rhs),

            AstExpr::Binary { op, lhs, rhs } => {
                lhs.accept(self);
                self.instr("pushq", "%rax"); // push lhs
                rhs.accept(self); // rhs in rax
                self.instr("movq", "%rax, %rcx"); // move rhs in rcx
                self.instr("popq", "%rax"); // get lhs back in rax
                self.emit_binop(op);
            }

            AstExpr::Unary { op, operand } => {
                operand.accept(self);

                match op {
                    Unop::Addition => {} // :>
                    Unop::Negation => self.instr("negq", "%rax"),
                    Unop::LogicalNegation => {
                        self.instr("cmpq", "$0, %rax");
                        self.instr("sete", "%al");
                        self.instr("movzbl", "%al, %eax");
                    }
                    Unop::BitwiseComplement => self.instr("notq", "%rax"),
                }
            }

            AstExpr::Int(v) => {
                self.instr("movq", &format!("${}, %rax", v));
            }
        }
    }
}
