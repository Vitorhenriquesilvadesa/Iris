// #![allow(dead_code)]

// use std::collections::HashMap;

// use common::{
//     ast::{
//         Item, Spanned,
//         expression::{
//             AssignmentExpr, BinaryExpr, BinaryOp, CallExpr, ExprKind, LambdaExpr, UnaryExpr,
//         },
//         item::{ExtendDef, ItemKind, ModelDef},
//         statement::{LetStmt, StmtKind},
//     },
//     diagnostic::Diagnostic,
//     hir::{
//         self, Hir,
//         expression::{
//             ExprId, Expression, HirAssignOp, HirBinaryOp, HirLambdaParam, HirUnaryOp, Literal,
//             Resolution,
//         },
//         globals::GlobalScope,
//         item::{HirMethod, HirModel},
//         statement::{Statement, StmtId},
//     },
//     interner::SymbolId,
//     source::SourceFileId,
//     span::Span,
// };
// use compiler_api::queries::{scope::ScopeQueries, symbol::SymbolQueries};

// use crate::lowering::{diagnostic::map_hir_error, error::HirError, resolver::Resolver};

// pub(crate) struct HirOutput {
//     pub hir: Hir,
//     pub diagnostics: Vec<Diagnostic>,
// }

// #[derive(Debug, Clone)]
// pub struct ModuleDefinitions {
//     pub models: HashMap<SymbolId, HirModel>,
//     pub extensions: HashMap<SymbolId, Vec<HirMethod>>,
//     pub globals: HashMap<SymbolId, ExprId>,
//     pub statements: Vec<hir::item::HirItem>,
// }

// #[derive(Debug, Clone)]
// pub struct HirGenerator<'a, Ctx>
// where
//     Ctx: ScopeQueries + SymbolQueries,
// {
//     hir: Hir,
//     items: &'a [Item],
//     ctx: &'a Ctx,
//     source_file_id: SourceFileId,
//     diagnostics: Vec<Diagnostic>,
//     definitions: ModuleDefinitions,
//     resolver: Resolver<'a>,
// }

// impl<'a, Ctx> HirGenerator<'a, Ctx>
// where
//     Ctx: ScopeQueries + SymbolQueries,
// {
//     pub fn new(
//         items: &'a [Item],
//         source_file_id: SourceFileId,
//         ctx: &'a Ctx,
//         globals: &'a GlobalScope,
//     ) -> Self {
//         Self {
//             items,
//             definitions: ModuleDefinitions {
//                 models: HashMap::new(),
//                 extensions: HashMap::new(),
//                 globals: HashMap::new(),
//                 statements: Vec::new(),
//             },
//             hir: Hir::new(),
//             source_file_id,
//             diagnostics: Vec::new(),
//             ctx,
//             resolver: Resolver::new(globals),
//         }
//     }

//     pub fn generate(mut self) -> HirOutput {
//         let definitions = self.collect_definitions();

//         self.merge_extensions(definitions);

//         self.resolver.begin_scope();
//         for item in &self.definitions.statements {
//             self.hir.allocate_item(item.clone());
//         }
//         self.resolver.end_scope();

//         for (id, model) in &self.definitions.models {
//             self.hir.allocate_model(*id, model.clone());
//         }

//         for (symbol, expr_id) in &self.definitions.globals {
//             self.hir.globals.insert(*symbol, *expr_id);
//         }

//         HirOutput {
//             hir: self.hir,
//             diagnostics: self.diagnostics,
//         }
//     }

//     fn gen_stmt_hir(&mut self, stmt: &StmtKind) -> StmtId {
//         let stmt = match stmt {
//             StmtKind::Let(let_stmt) => self.gen_let_hir(let_stmt),
//             StmtKind::Block(stmts) => self.gen_scope_hir(stmts),
//             StmtKind::Expr(expr) => Statement::Expression(self.gen_expr_hir(&expr)),
//             StmtKind::If {
//                 condition,
//                 if_branch,
//                 else_branch,
//             } => self.gen_if_hir(condition, if_branch, else_branch),
//         };

//         self.hir.allocate_stmt(stmt)
//     }

//     fn gen_expr_hir(&mut self, expr: &Spanned<ExprKind>) -> ExprId {
//         let expr = match &expr.node {
//             ExprKind::Literal(literal) => self.gen_literal(literal),
//             ExprKind::Ident(id) => self.gen_ident_hir(id),
//             ExprKind::Binary(expr) => self.gen_binary(expr),
//             ExprKind::Unary(unary_expr) => self.gen_unary(unary_expr),
//             ExprKind::Call(call_expr) => self.gen_call(call_expr),
//             ExprKind::List(elements) => self.gen_list(elements),
//             ExprKind::If(if_expr) => todo!(),
//             ExprKind::Grouping(spanned) => self.gen_expr_hir(&spanned),
//             ExprKind::Lambda(lambda_expr) => self.gen_lambda(lambda_expr),
//             ExprKind::Assign(expr) => self.gen_assign_hir(expr),
//         };

//         expr
//     }

//     fn gen_binary(&mut self, binary: &BinaryExpr) -> ExprId {
//         let expr = match binary.op {
//             BinaryOp::Comma => {
//                 self.report_error(
//                     HirError::TupleExpression,
//                     binary.left.span.merge(&binary.right.span),
//                 );
//                 let expr = Expression::Error;
//                 return self.hir.allocate_expr(expr);
//             }
//             BinaryOp::Pipe => todo!(),
//             BinaryOp::Range => self.gen_range(&binary.left, &binary.right),
//             BinaryOp::Add
//             | BinaryOp::Sub
//             | BinaryOp::Mul
//             | BinaryOp::Div
//             | BinaryOp::Eq
//             | BinaryOp::Neq
//             | BinaryOp::Lt
//             | BinaryOp::Gt
//             | BinaryOp::Leq
//             | BinaryOp::Geq
//             | BinaryOp::And
//             | BinaryOp::Or
//             | BinaryOp::Mod
//             | BinaryOp::LShift
//             | BinaryOp::RShift
//             | BinaryOp::BitwiseAnd
//             | BinaryOp::BitwiseOr => {
//                 let lhs = self.gen_expr_hir(&binary.left);
//                 let rhs = self.gen_expr_hir(&binary.right);

//                 Expression::Binary {
//                     op: HirBinaryOp::from_ast_op(&binary.op),
//                     lhs,
//                     rhs,
//                 }
//             }
//         };

//         self.hir.allocate_expr(expr)
//     }

//     fn gen_literal(&mut self, literal: &Literal) -> ExprId {
//         let expr = Expression::Literal(literal.clone());
//         self.hir.allocate_expr(expr)
//     }

//     pub(super) fn report_error(&mut self, error: HirError, span: Span) {
//         let diagnostic = map_hir_error(error, self.source_file_id, span);
//         self.diagnostics.push(diagnostic);
//     }

//     fn gen_assign_hir(&mut self, expr: &AssignmentExpr) -> ExprId {
//         println!("{:#?}", self.resolver);
//         match &expr.assignee.node {
//             ExprKind::Ident(_) => {
//                 let value = self.gen_expr_hir(&expr.value);
//                 let op = HirAssignOp::from_ast_assign_op(expr.op);
//                 let assignee = self.gen_expr_hir(&expr.assignee);

//                 let expr = Expression::Assign {
//                     assignee,
//                     op,
//                     value,
//                 };
//                 return self.hir.allocate_expr(expr);
//             }
//             _ => {
//                 self.report_error(HirError::InvalidAssignTarget, expr.assignee.span);
//                 return self.hir.allocate_expr(Expression::Error);
//             }
//         }
//     }

//     fn gen_ident_hir(&mut self, ident: &Spanned<String>) -> ExprId {
//         let id = self.ctx.intern_symbol(&ident.node);
//         if let Some(id) = self.resolver.resolve_local(&id) {
//             let expr = Expression::Variable(Resolution::Local(id));
//             return self.hir.allocate_expr(expr);
//         } else if let Some(id) = self.resolver.resolve_global(&id) {
//             let expr = Expression::Variable(Resolution::Global(id));
//             return self.hir.allocate_expr(expr);
//         } else {
//             self.report_error(HirError::SymbolNotFound(ident.node.clone()), ident.span);
//             return self.hir.allocate_expr(Expression::Error);
//         }
//     }

//     fn gen_let_hir(&mut self, let_stmt: &LetStmt) -> Statement {
//         let initializer = self.gen_expr_hir(&let_stmt.initializer);

//         let name_str = let_stmt.name.node.clone();
//         let symbol = self.ctx.intern_symbol(&name_str);
//         let new_id = self.resolver.declare(symbol);

//         Statement::Let {
//             initializer,
//             symbol,
//             name: Resolution::Local(new_id),
//         }
//     }

//     fn gen_unary(&mut self, unary_expr: &UnaryExpr) -> ExprId {
//         let expr = self.gen_expr_hir(&unary_expr.expr);
//         let op = HirUnaryOp::from_ast_unary_op(unary_expr.op);

//         return self.hir.allocate_expr(Expression::Unary { op, expr });
//     }

//     fn gen_lambda(&mut self, lambda_expr: &LambdaExpr) -> ExprId {
//         let mut params = vec![];

//         self.resolver.begin_scope();

//         for p in &lambda_expr.params {
//             let symbol = self.ctx.intern_symbol(&p.name);
//             let local = self.resolver.declare(symbol);
//             let param = HirLambdaParam { symbol, local };

//             params.push(param);
//         }

//         let body = self.gen_stmt_hir(&lambda_expr.body.node);

//         self.resolver.end_scope();

//         let expr = Expression::Lambda { params, body };

//         self.hir.allocate_expr(expr)
//     }

//     fn gen_scope_hir(&mut self, stmts: &[Spanned<StmtKind>]) -> Statement {
//         let mut hir_stmts = vec![];

//         for s in stmts {
//             let id = self.gen_stmt_hir(&s.node);
//             hir_stmts.push(id);
//         }

//         let stmt = Statement::Scope {
//             statements: hir_stmts,
//         };

//         stmt
//     }

//     fn gen_list(&mut self, elements: &[Spanned<ExprKind>]) -> ExprId {
//         let mut hir_elements = vec![];

//         for e in elements {
//             let element = self.gen_expr_hir(e);
//             hir_elements.push(element);
//         }

//         let expr = Expression::List {
//             elements: hir_elements,
//         };

//         self.hir.allocate_expr(expr)
//     }

//     fn gen_range(&mut self, left: &Spanned<ExprKind>, right: &Spanned<ExprKind>) -> Expression {
//         let start = self.gen_expr_hir(left);
//         let end = self.gen_expr_hir(right);
//         let expr = Expression::Range { start, end };

//         expr
//     }

//     fn gen_if_hir(
//         &mut self,
//         condition: &Spanned<ExprKind>,
//         if_branch: &Spanned<StmtKind>,
//         else_branch: &Option<Box<Spanned<StmtKind>>>,
//     ) -> Statement {
//         let cond = self.gen_expr_hir(condition);
//         let if_branch = self.gen_stmt_hir(&if_branch.node);
//         let mut hir_else_branch: Option<StmtId> = None;

//         if let Some(branch) = else_branch {
//             hir_else_branch = Some(self.gen_stmt_hir(&branch.node));
//         }

//         return Statement::If {
//             condition: cond,
//             if_branch,
//             else_branch: hir_else_branch,
//         };
//     }

//     fn collect_definitions(&mut self) -> ModuleDefinitions {
//         for item in self.items {
//             match &item.node {
//                 ItemKind::Import(import_def) => {
//                     println!("{:#?}", import_def);
//                     todo!();
//                 }
//                 ItemKind::Model(model_def) => {
//                     self.collect_model(model_def);
//                 }
//                 ItemKind::Extend(extend_def) => {
//                     self.collect_extend(extend_def);
//                 }
//                 ItemKind::GlobalLet(let_stmt) => {
//                     let name_str = let_stmt.name.node.clone();
//                     let symbol = self.ctx.intern_symbol(&name_str);
//                     self.resolver.declare_global(symbol);
//                 }
//                 ItemKind::Stmt(_) => {}
//             }
//         }

//         for item in self.items {
//             if let ItemKind::Stmt(stmt) = &item.node {
//                 let hir_item = hir::item::HirItem::Stmt(self.gen_stmt_hir(stmt));
//                 self.definitions.statements.push(hir_item);
//             }
//         }

//         for item in self.items {
//             if let ItemKind::GlobalLet(let_stmt) = &item.node {
//                 self.collect_global_let(let_stmt);
//             }
//         }

//         self.definitions.clone()
//     }

//     fn merge_extensions(&mut self, definitions: ModuleDefinitions) {
//         for (target_id, methods) in &definitions.extensions {
//             if let Some(model) = self.definitions.models.get_mut(target_id) {
//                 for method in methods {
//                     model.methods.insert(method.name, method.clone());
//                 }
//             }
//         }
//     }

//     fn collect_model(&mut self, model_def: &ModelDef) {
//         let name = self.ctx.intern_symbol(&model_def.name.node);

//         let mut fields = vec![];
//         for f in &model_def.fields {
//             fields.push(self.ctx.intern_symbol(&f.node));
//         }

//         let mut methods = HashMap::new();
//         for m in &model_def.methods {
//             let mut body = vec![];
//             for stmt in &m.body.stmts {
//                 body.push(self.gen_stmt_hir(&stmt.node));
//             }

//             let mut params = vec![];
//             for p in &m.params {
//                 params.push(self.ctx.intern_symbol(&p.node));
//             }

//             methods.insert(name, HirMethod { body, name, params });
//         }

//         let model = HirModel {
//             name,
//             fields,
//             methods,
//         };

//         self.definitions.models.insert(name, model);
//     }

//     fn collect_extend(&mut self, extend_def: &ExtendDef) {
//         let target_id = self.ctx.intern_symbol(&extend_def.target);

//         let mut methods = vec![];
//         for m in &extend_def.methods {
//             let mut body = vec![];
//             for s in &m.body.stmts {
//                 body.push(self.gen_stmt_hir(&s.node));
//             }

//             let mut params = vec![];
//             for p in &m.params {
//                 params.push(self.ctx.intern_symbol(&p.node));
//             }

//             let method = HirMethod {
//                 body,
//                 name: target_id,
//                 params,
//             };
//             methods.push(method);
//         }

//         self.definitions
//             .extensions
//             .entry(target_id)
//             .or_insert_with(Vec::new)
//             .extend(methods);
//     }

//     fn collect_global_let(&mut self, let_stmt: &LetStmt) {
//         let name_str = let_stmt.name.node.clone();
//         let symbol = self.ctx.intern_symbol(&name_str);

//         let initializer = self.gen_expr_hir(&let_stmt.initializer);

//         self.definitions.globals.insert(symbol, initializer);
//     }

//     fn gen_call(&mut self, call_expr: &CallExpr) -> ExprId {
//         let callee = self.gen_expr_hir(&call_expr.callee);
//         let mut args = vec![];

//         for arg in &call_expr.args {
//             args.push(self.gen_expr_hir(arg));
//         }

//         let expr = Expression::Call { callee, args };

//         self.hir.allocate_expr(expr)
//     }
// }
