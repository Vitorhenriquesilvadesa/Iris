use iris_ast::{
    Spanned,
    expression::{BinaryExpr, ExprKind, Literal, UnaryExpr},
};
use iris_db::symbol::SymbolQueries;
use iris_hir::expression::{ExprId, HirBinaryOp, HirExpression, HirLiteral, HirUnaryOp};

use crate::{HirResult, hir_gen::HirGenerator};

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
            ExprKind::Call(call_expr) => todo!(),
            ExprKind::List(elements) => todo!(),
            ExprKind::If(if_expr) => todo!(),
            ExprKind::Grouping(group) => todo!(),
            ExprKind::Lambda(lambda_expr) => todo!(),
            ExprKind::Assign(assignment_expr) => todo!(),
            ExprKind::Member(member_expr) => todo!(),
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
}
