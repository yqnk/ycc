use std::{collections::HashMap, fmt::Write};

use crate::ast::{AstBlockItem, AstExpr, AstFunc, AstProgram, AstStatement, Type, Visitor};
use crate::{Binop, Unop};

// Default int value
const DEFAULT_VALUE: i32 = 0;

#[derive(Default)]
pub struct X86Backend {
    out: String,

    label_count: u32,

    stack_index: i64,
    var_map: HashMap<String, i64>,
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

    pub fn prologue(&mut self) {
        self.instr("pushq", "%rbp");
        self.instr("movq", "%rsp, %rbp");
    }

    pub fn epilogue(&mut self) {
        self.instr("movq", "%rbp, %rsp");
        self.instr("popq", "%rbp");
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
    type BlockItem = ();

    fn visit_program(&mut self, program: &AstProgram) -> String {
        self.out.clear();
        program.main.accept(self);

        std::mem::take(&mut self.out)
    }

    fn visit_func(&mut self, func: &AstFunc) {
        self.directive(&format!(".globl {}", func.name));
        self.label(&func.name);
        self.prologue();

        for block in &func.blocks {
            block.accept(self);
        }

        if let Some(AstBlockItem::Statement { statement }) = func.blocks.last()
            && &func.name == "main"
            && func.ret_type == Type::Int
        {
            match statement {
                AstStatement::Return { exp: _ } => (),
                _ => {
                    self.instr("movq", &format!("${}, %rax", DEFAULT_VALUE));
                    self.epilogue();
                    self.instr("ret", "");
                }
            }
        }
    }

    fn visit_statement(&mut self, statement: &AstStatement) {
        match statement {
            AstStatement::Return { exp } => {
                exp.accept(self);
                self.epilogue();
                self.instr("ret", "");
            }
            AstStatement::Expr { exp } => exp.accept(self),
            AstStatement::Cond {
                exp,
                if_body,
                else_body,
            } => {
                let id = self.next_label_id();
                let else_label = format!(".L_else_{}", id);
                let post_label = format!(".L_post_{}", id);

                exp.accept(self);

                self.instr("cmpq", "$0, %rax");
                self.instr("je", &format!("{}", else_label));
                if_body.accept(self);
                self.instr("jmp", &format!("{}", post_label));
                self.label(&else_label);
                if let Some(e) = else_body {
                    e.accept(self);
                }
                self.label(&post_label);
            }
            AstStatement::Block { items } => {
                items.iter().for_each(|i| i.accept(self));
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
            AstExpr::Var(name) => {
                let Some(&offset) = self.var_map.get(name) else {
                    panic!("Use of undeclared indentifier: {}", name);
                };

                self.instr("movq", &format!("{}(%rbp), %rax", offset));
            }
            AstExpr::Assignment { op, lhs, rhs } => {
                let Some(&offset) = self.var_map.get(lhs) else {
                    panic!("Use of undeclared identifier: {}", lhs);
                };

                rhs.accept(self);

                if let Some(op) = op {
                    // rhs is in %rax but emit_binop wants lhs in %rax and rhs in %rcx
                    self.instr("movq", "%rax, %rcx");
                    self.instr("movq", &format!("{}(%rbp), %rax", offset));
                    self.emit_binop(op);
                }

                self.instr("movq", &format!("%rax, {}(%rbp)", offset));
            }
            AstExpr::Conditional {
                cond,
                then_exp,
                else_exp,
            } => {
                let id = self.next_label_id();
                let else_label = format!(".L_else_{}", id);
                let post_label = format!(".L_post_{}", id);

                cond.accept(self);

                self.instr("cmpq", "$0, %rax");
                self.instr("je", &format!("{}", else_label));
                then_exp.accept(self);
                self.instr("jmp", &format!("{}", post_label));
                self.label(&else_label);
                else_exp.accept(self);
                self.label(&post_label);
            }
        }
    }

    fn visit_blockitem(&mut self, expr: &crate::ast::AstBlockItem) -> Self::BlockItem {
        match expr {
            AstBlockItem::Statement { statement } => {
                statement.accept(self);
            }
            AstBlockItem::VarDec {
                var_type: _,
                name,
                exp,
            } => {
                if self.var_map.contains_key(name) {
                    panic!("variable declared twice: {}", name);
                }

                if let Some(rhs) = exp {
                    rhs.accept(self);
                } else {
                    self.instr("movq", &format!("${}, %rax", DEFAULT_VALUE));
                }

                self.instr("pushq", "%rax");
                self.stack_index -= 8;
                self.var_map.insert(name.to_string(), self.stack_index);
            }
        }
    }
}
