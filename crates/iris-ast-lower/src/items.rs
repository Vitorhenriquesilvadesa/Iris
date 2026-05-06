use iris_ast::{
    Spanned,
    item::{AstTypeBase, AstTypeInfo, AstTypeModifier, FunctionDef, ItemKind, TypeDef, TypeName},
};
use iris_db::symbol::SymbolQueries;
use iris_hir::{
    item::{HirFunction, HirItem, HirParam, HirType, HirTypeInfo, ItemId},
    statement::StmtId,
};
use iris_interner::SymbolId;

use crate::{HirResult, hir_gen::HirGenerator};

impl<'a, Ctx> HirGenerator<'a, Ctx>
where
    Ctx: SymbolQueries,
{
    pub(super) fn gen_hir_for(&mut self, item: &Spanned<ItemKind>) -> HirResult<ItemId> {
        match &item.node {
            ItemKind::Import(import_def) => todo!(),
            ItemKind::GlobalLet(let_stmt) => todo!(),
            ItemKind::Type(type_def) => self.gen_type_ir(type_def),
            ItemKind::Stmt(stmt_kind) => {
                let stmt = self.gen_stmt_ir(stmt_kind)?;
                Some(self.allocate_item(HirItem::Stmt(stmt)))
            }
            ItemKind::Function(function_def) => self.gen_function_ir(function_def),
            ItemKind::Metadata(meta_data_usage) => todo!(),
            ItemKind::Impl(impl_def) => todo!(),
        }
    }

    pub(crate) fn gen_function_ir(
        &mut self,
        FunctionDef {
            body,
            name,
            params,
            return_kind,
        }: &FunctionDef,
    ) -> HirResult<ItemId> {
        let name = self.ctx.intern_symbol(&name.node);
        let return_type = self.get_type_info(&Some(return_kind));
        let mut hir_body: Vec<StmtId> = Vec::new();
        let mut params: Vec<HirParam> = Vec::new();

        for s in &body.stmts {
            if let Some(stmt) = self.gen_stmt_ir(&s.node) {
                hir_body.push(stmt);
            }
        }

        let hir_function = HirItem::Function(HirFunction {
            name,
            return_type,
            body: hir_body,
            params,
        });

        Some(self.allocate_item(hir_function))
    }

    pub(crate) fn gen_type_ir(&mut self, type_def: &TypeDef) -> HirResult<ItemId> {
        let name = self.ctx.intern_symbol(&type_def.name.node);

        let fields: Vec<(SymbolId, Option<HirTypeInfo>)> = type_def
            .fields
            .iter()
            .map(|f| {
                let symbol_id = self.ctx.intern_symbol(&f.node.name.node);
                let type_info = self.get_type_info(&f.node.kind);

                (symbol_id, type_info)
            })
            .collect();

        let hir_type: HirItem = HirItem::Type(HirType { name, fields });

        Some(self.allocate_item(hir_type))
    }

    pub(crate) fn get_type_info(&self, kind: &Option<Spanned<AstTypeInfo>>) -> Option<HirTypeInfo> {
        if let Some(k) = &kind {
            let info = self.lower_base(&k.node.base.node);
            Some(self.apply_modifier(info, &k.node.modifier.node))
        } else {
            None
        }
    }

    fn lower_base(&self, base: &AstTypeBase) -> HirTypeInfo {
        match base {
            AstTypeBase::Named(TypeName(name)) => HirTypeInfo::Named(self.ctx.intern_symbol(name)),
            AstTypeBase::Array(inner) => HirTypeInfo::Array(Box::new(self.lower_base(&inner.node))),
        }
    }

    fn apply_modifier(&self, info: HirTypeInfo, modifier: &AstTypeModifier) -> HirTypeInfo {
        match modifier {
            AstTypeModifier::None => info,
            AstTypeModifier::Optional => HirTypeInfo::Optional(Box::new(info)),
            AstTypeModifier::Fallible => HirTypeInfo::Fallible(Box::new(info)),
            AstTypeModifier::FallibleOptional => {
                HirTypeInfo::Fallible(Box::new(HirTypeInfo::Optional(Box::new(info))))
            }
        }
    }
}
