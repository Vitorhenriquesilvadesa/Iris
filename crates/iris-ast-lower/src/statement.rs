use iris_ast::statement::{LetStmt, StmtKind};
use iris_db::symbol::SymbolQueries;
use iris_hir::{
    expression::ExprId,
    item::{HirItem, ItemId},
    statement::{HirStatement, StmtId},
};

use crate::{HirResult, hir_gen::HirGenerator};

impl<'a, Ctx> HirGenerator<'a, Ctx>
where
    Ctx: SymbolQueries,
{
    pub(crate) fn gen_stmt_ir(&mut self, stmt_kind: &StmtKind) -> HirResult<StmtId> {
        let stmt = match stmt_kind {
            StmtKind::Let(let_stmt) => self.gen_let_hir(let_stmt),
            StmtKind::Block(spanneds) => todo!(),
            StmtKind::If {
                condition,
                if_branch,
                else_branch,
            } => todo!(),
            StmtKind::Expr(spanned) => todo!(),
        };

        // let item = HirItem::Stmt(stmt?);
        Some(stmt?)
    }

    pub(crate) fn gen_let_hir(&mut self, let_stmt: &LetStmt) -> HirResult<StmtId> {
        let symbol = self.ctx.intern_symbol(&let_stmt.name.node);
        let initializer: Option<ExprId> = if let Some(ini) = &let_stmt.initializer {
            Some(self.gen_expr_hir(&ini.node)?)
        } else {
            None
        };

        Some(self.allocate_stmt(HirStatement::Let {
            symbol,
            initializer,
        }))
    }
}
