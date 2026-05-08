#[allow(dead_code)]
use iris_ast::{
    Spanned,
    expression::{AssignmentExpr, BinaryExpr, CallExpr, ExprKind, Literal, MemberExpr, UnaryExpr},
};
use iris_db::symbol::SymbolQueries;
use iris_hir::expression::{
    ExprId, HirAssign, HirAssignOp, HirBinaryOp, HirExpression, HirIfExpr, HirLiteral, HirMember,
    HirUnaryOp,
};
use iris_interner::SymbolId;

use crate::{HirResult, error::HirError, hir_gen::HirGenerator};

type ExprResult = HirResult<ExprId>;

impl<'a, Ctx> HirGenerator<'a, Ctx>
where
    Ctx: SymbolQueries,
{
    pub(crate) fn gen_expr_hir(&mut self, node: &ExprKind) -> ExprResult {
        match node {
            ExprKind::Literal(literal) => self.gen_literal_hir(literal),
            ExprKind::Ident(ident) => self.gen_ident_hir(ident),
            ExprKind::Binary(binary_expr) => self.gen_binary_hir(binary_expr),
            ExprKind::Unary(unary_expr) => self.gen_unary_hir(unary_expr),
            ExprKind::Call(call_expr) => self.gen_call_hir(call_expr),
            ExprKind::List(elements) => self.gen_array_hir(elements),
            ExprKind::If(if_expr) => self.gen_if_expr_hir(if_expr),
            ExprKind::Grouping(group) => self.gen_group_hir(group),
            ExprKind::Lambda(_) => todo!(),
            ExprKind::Assign(assignment_expr) => self.gen_assignment_hir(assignment_expr),
            ExprKind::Member(member_expr) => self.gen_member_hir(member_expr),
        }
    }

    fn gen_literal_hir(&mut self, expr: &Literal) -> ExprResult {
        let hir_literal = match expr {
            Literal::Int(i) => HirLiteral::Int(*i),
            Literal::Float(f) => HirLiteral::Float(*f),
            Literal::String(s) => HirLiteral::String(s.clone()),
            Literal::Bool(b) => HirLiteral::Bool(*b),
        };

        Some(self.allocate_expr(HirExpression::Literal(hir_literal)))
    }

    pub(crate) fn gen_ident_hir(&mut self, ident: &Spanned<String>) -> ExprResult {
        let symbol = self.ctx.intern_symbol(&ident.node);

        Some(self.allocate_expr(HirExpression::Ident(symbol)))
    }

    pub(crate) fn gen_binary_hir(&mut self, binary_expr: &BinaryExpr) -> ExprResult {
        let lhs = self.gen_expr_hir(&binary_expr.left.node)?;
        let op = HirBinaryOp::from_ast_op(&binary_expr.op);
        let rhs = self.gen_expr_hir(&binary_expr.right.node)?;

        let hir_bin_expr = HirExpression::Binary { op, lhs, rhs };

        Some(self.allocate_expr(hir_bin_expr))
    }

    pub(crate) fn gen_unary_hir(&mut self, unary_expr: &UnaryExpr) -> ExprResult {
        let op = HirUnaryOp::from_ast_unary_op(unary_expr.op);
        let expr = self.gen_expr_hir(&unary_expr.expr.node)?;

        let hir_unary_expr = HirExpression::Unary { op, expr };
        Some(self.allocate_expr(hir_unary_expr))
    }

    pub(crate) fn gen_call_hir(&mut self, call_expr: &CallExpr) -> ExprResult {
        let callee = match &call_expr.callee.node {
            ExprKind::Ident(ident) => self.gen_ident_hir(ident),
            ExprKind::Member(member_expr) => self.gen_member_hir(member_expr),
            _ => {
                let span = call_expr.callee.span;
                self.report_error(HirError::InvalidAssignTarget, span);
                Some(self.allocate_expr(HirExpression::Error))
            }
        }?;

        let args: Vec<ExprId> = call_expr
            .args
            .iter()
            .map(|expr| {
                self.gen_expr_hir(&expr.node)
                    .unwrap_or_else(|| self.allocate_expr(HirExpression::Error))
            })
            .collect();

        let hir_call = HirExpression::Call { callee, args };

        Some(self.allocate_expr(hir_call))
    }

    pub(crate) fn gen_member_hir(&mut self, member_expr: &MemberExpr) -> ExprResult {
        let mut members: Vec<SymbolId> = Vec::new();

        let mut current_member = &member_expr.object;
        let mut current_symbol = self.ctx.intern_symbol(&member_expr.member.node);

        members.push(current_symbol);

        while let ExprKind::Member(m) = &current_member.node {
            current_member = &m.object;
            current_symbol = self.ctx.intern_symbol(&m.member.node);
            members.push(current_symbol);
        }

        let base_expr = self.gen_expr_hir(&current_member.node)?;

        members.reverse();

        let hir_member = HirExpression::Member(HirMember {
            members,
            base: base_expr,
        });

        Some(self.allocate_expr(hir_member))
    }

    pub(crate) fn gen_array_hir(&mut self, elements: &[Spanned<ExprKind>]) -> ExprResult {
        let elements: Vec<ExprId> = elements
            .iter()
            .map(|e| {
                self.gen_expr_hir(&e.node)
                    .unwrap_or_else(|| self.allocate_expr(HirExpression::Error))
            })
            .collect();

        let hir_array = HirExpression::List { elements };
        Some(self.allocate_expr(hir_array))
    }

    pub(crate) fn gen_group_hir(&mut self, group: &Spanned<ExprKind>) -> ExprResult {
        let hir_expr = self.gen_expr_hir(&group.node)?;
        Some(hir_expr)
    }

    pub(crate) fn gen_if_expr_hir(&mut self, if_expr: &iris_ast::expression::IfExpr) -> ExprResult {
        let condition = self.gen_expr_hir(&if_expr.condition.node)?;
        let then_branch = self.gen_expr_hir(&if_expr.then_branch.node)?;
        let else_branch = self.gen_expr_hir(&if_expr.else_branch.node)?;

        let hir_if_expr = HirExpression::If(HirIfExpr {
            condition,
            then_branch,
            else_branch,
        });

        Some(self.allocate_expr(hir_if_expr))
    }

    pub(crate) fn gen_assignment_hir(&mut self, assignment_expr: &AssignmentExpr) -> ExprResult {
        let target = self.gen_expr_hir(&assignment_expr.assignee.node)?;
        let op = HirAssignOp::from_ast_assign_op(assignment_expr.op);
        let value = self.gen_expr_hir(&assignment_expr.value.node)?;

        let hir_assignment = HirExpression::Assign(HirAssign { target, op, value });

        Some(self.allocate_expr(hir_assignment))
    }
}
