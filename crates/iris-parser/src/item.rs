use iris_ast::{
    Item, Spanned,
    expression::{Block, ExprKind, Param},
    item::{
        AstTypeBase, AstTypeInfo, AstTypeModifier, FunctionDef, GenericParam, ImplDef, ItemKind,
        MetaArgument, MetaDataUsage, TypeDef, TypeName,
    },
    statement::LetStmt,
};
use iris_span::Span;
use iris_syntax::TokenKind;

use crate::{ParseResult, Parser, error::ParseError};

impl<'a> Parser<'a> {
    pub(super) fn parse_item(&mut self) -> ParseResult<Spanned<ItemKind>> {
        if let Some(tok) = self.stream().peek_kind() {
            match tok {
                TokenKind::Import => self.parse_import(),
                TokenKind::Let => self.parse_global_let(),
                TokenKind::Fn => self.parse_function(),
                TokenKind::Type => self.parse_type(),
                TokenKind::At => self.parse_meta(),
                TokenKind::Impl => self.parse_impl(),
                _ => {
                    let stmt = self.parse_stmt()?;
                    Some(Spanned::new(ItemKind::Stmt(Box::new(stmt.node)), stmt.span))
                }
            }
        } else {
            None
        }
    }

    fn parse_function(&mut self) -> ParseResult<Item> {
        let keyword_span = self.expect_bump(TokenKind::Fn)?.span;
        let name_span = self.expect_bump(TokenKind::Ident)?.span;

        self.expect_bump(TokenKind::LParen)?;
        let params = if !self.check(TokenKind::RParen) {
            self.parse_separated_elements(TokenKind::Comma, |p| p.parse_param())?
        } else {
            vec![]
        };
        self.expect_bump(TokenKind::RParen)?;
        self.expect_bump(TokenKind::Arrow)?;
        let return_ty = self.parse_type_info()?;

        let mut statements = Vec::new();

        self.expect_bump(TokenKind::LBrace)?;

        while !self.check(TokenKind::RBrace) && !self.stream.is_eof() {
            let stmt = self.parse_stmt()?;
            statements.push(stmt);
        }

        let end_span = self.expect_bump(TokenKind::RBrace)?.span;

        Some(Spanned::new(
            ItemKind::Function(FunctionDef {
                name: Spanned::new(name_span.slice(self.source_text).to_string(), name_span),
                return_kind: return_ty,
                body: Block { stmts: statements },
                params,
            }),
            keyword_span.merge(&end_span),
        ))
    }

    fn parse_import(&mut self) -> ParseResult<Spanned<ItemKind>> {
        todo!()
    }

    fn parse_type_info(&mut self) -> ParseResult<Spanned<AstTypeInfo>> {
        let mut array_pairs: Vec<Span> = Vec::new();

        while self.check(TokenKind::LBracket) {
            let lb = self.expect_bump(TokenKind::LBracket)?.span;
            let rb = self.expect_bump(TokenKind::RBracket)?.span;
            array_pairs.push(lb.merge(&rb));
        }

        let name_tok = self.expect_bump(TokenKind::Ident)?;
        let name_span = name_tok.span;
        let name_str = name_span.slice(self.source_text).to_string();

        let mut generics = vec![];
        let mut generics_span: Option<Span> = None;

        if self.check(TokenKind::LBracket) {
            let lb = self.expect_bump(TokenKind::LBracket)?.span;

            generics = self.parse_separated_elements(TokenKind::Comma, |p| p.parse_type_info())?;

            let rb = self.expect_bump(TokenKind::RBracket)?.span;

            let mut gs = lb.merge(&rb);

            for g in &generics {
                gs = gs.merge(&g.span);
            }

            generics_span = Some(gs);
        }

        let mut base: Spanned<AstTypeBase> =
            Spanned::new(AstTypeBase::Named(TypeName(name_str)), name_span);

        for arr_span in array_pairs.into_iter().rev() {
            let wrapped_span = arr_span.merge(&base.span);
            base = Spanned::new(AstTypeBase::Array(Box::new(base)), wrapped_span);
        }

        let mut modifier = AstTypeModifier::None;
        let mut modifier_span = Span {
            start: base.span.start + base.span.length,
            length: 0,
        };

        if self.check(TokenKind::Not)
            || self.check(TokenKind::FallibleOptional)
            || self.check(TokenKind::Optional)
        {
            let tk = self.stream_mut().peek().unwrap().clone();
            self.stream_mut().bump();

            modifier_span = tk.span;

            modifier = match AstTypeModifier::from_token_kind(tk.kind) {
                Some(m) => m,
                None => {
                    self.report_error(ParseError::UnexpectedToken { found: tk.kind }, tk.span);
                    return None;
                }
            };
        }

        let mut full_span = base.span;
        if let Some(gs) = generics_span {
            full_span = full_span.merge(&gs);
        }
        full_span = full_span.merge(&modifier_span);

        Some(Spanned::new(
            AstTypeInfo {
                base,
                modifier: Spanned::new(modifier, modifier_span),
                generics,
            },
            full_span,
        ))
    }

    pub(super) fn parse_separated_elements<T>(
        &mut self,
        separator: TokenKind,
        mut rule: impl FnMut(&mut Self) -> ParseResult<T>,
    ) -> ParseResult<Vec<T>> {
        let mut elements = vec![];

        let checkpoint = self.stream().checkpoint();
        let _diag_checkpoint = self.diagnostics.len();

        let node = match rule(self) {
            Some(n) => n,
            None => {
                self.stream_mut().rewind(checkpoint);
                // self.diagnostics.truncate(diag_checkpoint);
                return Some(elements);
            }
        };
        elements.push(node);

        loop {
            if self.try_expect(&[separator]) {
                let inner_checkpoint = self.stream().checkpoint();
                let inner_diag_checkpoint = self.diagnostics.len();

                let node = match rule(self) {
                    Some(n) => n,
                    None => {
                        self.stream_mut().rewind(inner_checkpoint);
                        self.diagnostics.truncate(inner_diag_checkpoint);
                        break;
                    }
                };
                elements.push(node);
            } else {
                break;
            }
        }

        Some(elements)
    }

    fn parse_global_let(&mut self) -> ParseResult<Spanned<ItemKind>> {
        let let_span = self.expect_bump(TokenKind::Let)?.span;
        let name_span = self.expect_bump(TokenKind::Ident)?.span;
        let name = Spanned::new(name_span.slice(self.source_text).to_string(), name_span);

        let initializer: Option<Spanned<ExprKind>> = if self.check(TokenKind::Equal) {
            self.expect_bump(TokenKind::Equal)?;
            Some(self.parse_expr()?)
        } else {
            None
        };

        let end = self.expect_bump(TokenKind::Semicolon)?;
        let complete_span = let_span.merge(&end.span);

        let let_stmt = LetStmt { name, initializer };

        Some(Spanned::new(
            ItemKind::GlobalLet(Box::new(let_stmt)),
            complete_span,
        ))
    }

    fn parse_type(&mut self) -> ParseResult<Spanned<ItemKind>> {
        let keyword_span = self.expect_bump(TokenKind::Type)?.span;
        let name_span = self.expect_bump(TokenKind::Ident)?.span;
        let name = name_span.slice(self.source_text).to_string();

        let mut generics: Vec<Spanned<GenericParam>> = vec![];

        if self.try_expect(&[TokenKind::LBracket]) {
            generics = self.parse_separated_elements(TokenKind::Comma, |p| p.parse_generic())?;
            self.expect_bump(TokenKind::RBracket)?;
        }

        self.expect_bump(TokenKind::LBrace)?;

        let fields = self.parse_separated_elements(TokenKind::Comma, |p| p.parse_param())?;

        let end_span = self.expect_bump(TokenKind::RBrace)?.span;

        Some(Spanned::new(
            ItemKind::Type(Box::new(TypeDef {
                fields,
                name: Spanned::new(name, name_span),
                generics,
            })),
            keyword_span.merge(&end_span),
        ))
    }

    fn parse_param(&mut self) -> ParseResult<Spanned<Param>> {
        let name_span = self.expect_bump(TokenKind::Ident)?.span;
        let name = Spanned::new(name_span.slice(self.source_text).to_string(), name_span);

        let mut kind = None;

        if self.check(TokenKind::Colon) {
            self.expect_bump(TokenKind::Colon)?;
            kind = Some(self.parse_type_info()?);
        }

        let kind_span = if let Some(ref k) = kind {
            name_span.merge(&k.span)
        } else {
            name_span
        };

        Some(Spanned::new(Param { name, kind }, kind_span))
    }

    fn parse_generic(&mut self) -> ParseResult<Spanned<GenericParam>> {
        let name_span = self.expect_bump(TokenKind::Ident)?.span;
        let name = name_span.slice(self.source_text).to_string();

        Some(Spanned::new(GenericParam { name }, name_span))
    }

    fn parse_meta(&mut self) -> ParseResult<Spanned<ItemKind>> {
        let mut start_span = self.expect_bump(TokenKind::At)?.span;
        let name_span = self.expect_bump(TokenKind::Ident)?.span;
        let name = name_span.slice(self.source_text).to_string();

        start_span = start_span.merge(&name_span);

        let mut args: Vec<Spanned<MetaArgument>> = vec![];
        if self.try_expect(&[TokenKind::LParen]) {
            args = self.parse_separated_elements(TokenKind::Comma, |p| p.parse_meta_argument())?;
            let r_paren_span = self.expect_bump(TokenKind::RParen)?.span;
            start_span = start_span.merge(&r_paren_span);
        }

        Some(Spanned::new(
            ItemKind::Metadata(MetaDataUsage {
                args,
                name: Spanned::new(name, name_span),
            }),
            start_span,
        ))
    }

    fn parse_meta_argument(&mut self) -> ParseResult<Spanned<MetaArgument>> {
        let name_span = self.expect_bump(TokenKind::Ident)?.span;
        let name = Spanned::new(name_span.slice(self.source_text).to_string(), name_span);

        self.expect_bump(TokenKind::Equal)?;
        let value = self.parse_expr()?;
        let complete_span = name_span.merge(&value.span);

        Some(Spanned::new(MetaArgument { name, value }, complete_span))
    }

    fn parse_impl(&mut self) -> ParseResult<Item> {
        let keyword_span = self.expect_bump(TokenKind::Impl)?.span;
        let target = self.parse_type_info()?;

        self.expect_bump(TokenKind::LBrace)?;

        let mut methods = vec![];

        while !self.stream().is_eof() && !self.check(TokenKind::RBrace) {
            let method_node = self.parse_function()?;
            methods.push(method_node);
        }

        let end_span = self.expect_bump(TokenKind::RBrace)?.span;
        let complete_span = keyword_span.merge(&end_span);

        Some(Spanned::new(
            ItemKind::Impl(ImplDef { methods, target }),
            complete_span,
        ))
    }
}
