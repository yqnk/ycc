use std::fmt::Write;

use crate::ast::{
    AstExpr, AstFactor, AstFunc, AstProgram, AstStatement, AstTerm, Binop, Unop, Visitor,
};

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
}

impl Visitor for X86Backend {
    type Program = String;
    type Func = ();
    type Statement = ();

    type Expr = ();
    type LogicalAndExpr = ();
    type EqualityExpr = ();
    type RelationalExpr = ();
    type AdditiveExpr = ();

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

        if expr.rest.is_empty() {
            return;
        }

        let id = self.next_label_id();
        let true_label = format!(".Lor_true_{}", id);
        let end_label = format!(".Lor_end_{}", id);

        for (op, laexpr) in &expr.rest {
            assert!(matches!(op, Binop::LogicalOr));

            // lhs already in rax: if it's true, || is true and rhs is never evaluated
            self.instr("cmpq", "$0, %rax");
            self.instr("jne", &true_label);
            laexpr.accept(self);
        }

        // last operand decides, normalized to 0 or 1
        self.instr("cmpq", "$0, %rax");
        self.instr("setne", "%al");
        self.instr("movzbl", "%al, %eax");
        self.instr("jmp", &end_label);

        self.label(&true_label);
        self.instr("movq", "$1, %rax");

        self.label(&end_label);
    }

    fn visit_logical_and_expr(
        &mut self,
        expr: &crate::ast::AstLogicalAndExpr,
    ) -> Self::LogicalAndExpr {
        expr.first.accept(self); // lhs in rax

        if expr.rest.is_empty() {
            return;
        }

        let id = self.next_label_id();
        let false_label = format!(".Land_false_{}", id);
        let end_label = format!(".Land_end_{}", id);

        for (op, eqexpr) in &expr.rest {
            assert!(matches!(op, Binop::LogicalAnd));

            // lhs already in rax: if it's false, && is false and rhs is never evaluated
            self.instr("cmpq", "$0, %rax");
            self.instr("je", &false_label);
            eqexpr.accept(self);
        }

        // last operand decides, normalized to 0 or 1
        self.instr("cmpq", "$0, %rax");
        self.instr("setne", "%al");
        self.instr("movzbl", "%al, %eax");
        self.instr("jmp", &end_label);

        self.label(&false_label);
        self.instr("movq", "$0, %rax");

        self.label(&end_label);
    }

    fn visit_equality_expr(&mut self, expr: &crate::ast::AstEqualityExpr) -> Self::EqualityExpr {
        expr.first.accept(self);

        if expr.rest.is_empty() {
            return;
        }

        for (op, relexpr) in &expr.rest {
            self.instr("push", "%rax");
            relexpr.accept(self);
            self.instr("pop", "%rcx");
            self.instr("cmpq", "%rax, %rcx");

            match op {
                Binop::EqualTo => self.instr("sete", "%al"),
                Binop::NotEqualTo => self.instr("setne", "%al"),
                _ => unreachable!(),
            }

            self.instr("movzbl", "%al, %eax");
        }
    }

    fn visit_relational_expr(
        &mut self,
        expr: &crate::ast::AstRelationalExpr,
    ) -> Self::RelationalExpr {
        expr.first.accept(self);

        if expr.rest.is_empty() {
            return;
        }

        for (op, relexpr) in &expr.rest {
            self.instr("push", "%rax");
            relexpr.accept(self);
            self.instr("pop", "%rcx");
            self.instr("cmpq", "%rcx, %rax");

            match op {
                Binop::LessThan => self.instr("setl", "%al"),
                Binop::GreaterThan => self.instr("setg", "%al"),
                Binop::GreaterThanOrEqualTo => self.instr("setge", "%al"),
                Binop::LessThanOrEqualTo => self.instr("setle", "%al"),
                _ => unreachable!(),
            }

            self.instr("movzbl", "%al, %eax");
        }
    }

    fn visit_additive_expr(&mut self, expr: &crate::ast::AstAdditiveExpr) -> Self::AdditiveExpr {
        expr.first.accept(self);

        if expr.rest.is_empty() {
            return;
        }

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

        if term.rest.is_empty() {
            return;
        }

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
                        self.instr("sete", "%al");
                        self.instr("movzbl", "%al, %eax");
                    }
                    Unop::BitwiseComplement => self.instr("notq", "%rax"),
                    _ => unreachable!(),
                }
            }
            AstFactor::Int(v) => {
                self.instr("movq", &format!("${}, %rax", v));
            }
        }
    }
}
