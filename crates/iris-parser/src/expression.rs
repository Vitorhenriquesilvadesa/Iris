use iris_ast::{
    Expression, Spanned,
    expression::{
        AssignmentExpr, AssignmentOp, BinaryExpr, BinaryOp, CallExpr, ExprKind, IfExpr, Literal,
        MemberExpr, UnaryExpr, UnaryOp,
    },
};
use iris_span::Span;
use iris_syntax::TokenKind;

use crate::{ParseResult, Parser, error::ParseError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OperandParseStrategy {
    Single,
    Multiple,
}

impl<'a> Parser<'a> {
    pub(super) fn parse_expr(&mut self) -> ParseResult<Expression> {
        self.parse_assign()
    }

    fn parse_assign(&mut self) -> ParseResult<Expression> {
        let lhs = self.parse_ternary()?;

        let is_assign = self
            .stream
            .peek()
            .map(|t| {
                matches!(
                    t.kind,
                    TokenKind::Equal
                        | TokenKind::PlusEqual
                        | TokenKind::MinusEqual
                        | TokenKind::StarEqual
                        | TokenKind::SlashEqual
                        | TokenKind::PercentEqual
                        | TokenKind::BitwiseAndEqual
                        | TokenKind::LShiftEqual
                        | TokenKind::RShiftEqual
                        | TokenKind::BitwiseOrEqual
                )
            })
            .unwrap_or(false);

        if is_assign {
            let op_token = self.stream.bump().unwrap().clone();

            let op = AssignmentOp::from_token(op_token.kind)?;
            let rhs = self.parse_assign()?;

            let span = lhs.span.merge(&rhs.span);

            return Some(Spanned::new(
                ExprKind::Assign(Box::new(AssignmentExpr {
                    assignee: lhs,
                    op,
                    value: rhs,
                })),
                span,
            ));
        }

        Some(lhs)
    }

    fn parse_ternary(&mut self) -> ParseResult<Expression> {
        let expr = self.parse_catch()?;

        if self.check(TokenKind::If) {
            let mut start_span = self.expect_bump(TokenKind::If)?.span;
            let then_branch = self.parse_catch()?;
            start_span = start_span.merge(&self.expect_bump(TokenKind::Else)?.span);
            let else_branch = self.parse_catch()?;
            let end_span = else_branch.span;

            return Some(Expression::new(
                ExprKind::If(Box::new(IfExpr {
                    condition: expr,
                    else_branch,
                    then_branch,
                })),
                start_span.merge(&end_span),
            ));
        }

        Some(expr)
    }

    fn parse_catch(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |p| p.parse_try(),
            OperandParseStrategy::Multiple,
            &[TokenKind::Catch],
        )
    }

    fn parse_try(&mut self) -> ParseResult<Expression> {
        if !self.check(TokenKind::Try) {
            return self.parse_equality();
        }

        let tok = self.stream_mut().bump().unwrap().clone();

        let expr = self.parse_equality()?;

        let span = tok.span.merge(&expr.span);
        Some(Spanned::new(
            ExprKind::Unary(Box::new(UnaryExpr {
                op: UnaryOp::Try,
                expr,
            })),
            span,
        ))
    }

    fn parse_equality(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |parser| parser.parse_or(),
            OperandParseStrategy::Single,
            &[TokenKind::EqualEqual, TokenKind::BangEqual],
        )
    }

    fn parse_or(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |parser| parser.parse_and(),
            OperandParseStrategy::Multiple,
            &[TokenKind::Or],
        )
    }

    fn parse_and(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |parser| parser.parse_comparison(),
            OperandParseStrategy::Multiple,
            &[TokenKind::And],
        )
    }

    fn parse_comparison(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |parser| parser.parse_pipe(),
            OperandParseStrategy::Single,
            &[
                TokenKind::Greater,
                TokenKind::GreaterEq,
                TokenKind::Less,
                TokenKind::LessEq,
            ],
        )
    }

    fn parse_pipe(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |parser| parser.parse_range(),
            OperandParseStrategy::Multiple,
            &[TokenKind::PipeGt],
        )
    }

    fn parse_range(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |parser| parser.parse_bitwise_or(),
            OperandParseStrategy::Single,
            &[TokenKind::Range],
        )
    }

    fn parse_bitwise_or(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |parser| parser.parse_bitwise_and(),
            OperandParseStrategy::Multiple,
            &[TokenKind::Pipe],
        )
    }

    fn parse_bitwise_and(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |parser| parser.parse_bitwise_shift(),
            OperandParseStrategy::Multiple,
            &[TokenKind::BitwiseAnd],
        )
    }

    fn parse_bitwise_shift(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |parser| parser.parse_term(),
            OperandParseStrategy::Multiple,
            &[TokenKind::LShift, TokenKind::RShift],
        )
    }

    fn parse_term(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |parser| parser.parse_factor(),
            OperandParseStrategy::Multiple,
            &[TokenKind::Plus, TokenKind::Minus],
        )
    }

    fn parse_factor(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |parser| parser.parse_unary(),
            OperandParseStrategy::Multiple,
            &[TokenKind::Star, TokenKind::Slash, TokenKind::Percent],
        )
    }

    fn parse_unary(&mut self) -> ParseResult<Expression> {
        self.parse_prefix(
            |p| p.parse_postfix(),
            OperandParseStrategy::Multiple,
            &[TokenKind::Minus, TokenKind::Plus, TokenKind::Not],
        )
    }

    fn parse_postfix(&mut self) -> ParseResult<Expression> {
        let mut expr = self.parse_literal()?;

        loop {
            if self.check(TokenKind::Dot) {
                let dot = self.stream_mut().bump().unwrap().clone();

                let (kind, span) = match self.stream().peek() {
                    Some(t) => (t.kind, t.span),
                    None => {
                        self.report_error(
                            ParseError::UnexpectedToken {
                                found: TokenKind::Eof,
                            },
                            dot.span,
                        );
                        return Some(expr);
                    }
                };

                if kind != TokenKind::Ident {
                    self.report_error(ParseError::UnexpectedToken { found: kind }, span);
                    self.stream_mut().bump();
                    continue;
                }

                self.stream_mut().bump();
                let name = span.slice(self.source_text).to_string();

                let full_span = expr.span.merge(&span);
                expr = Spanned::new(
                    ExprKind::Member(Box::new(MemberExpr {
                        object: expr,
                        member: Spanned::new(name, span),
                    })),
                    full_span,
                );
                continue;
            }

            if self.check(TokenKind::LParen) {
                self.stream_mut().bump();

                let args = if !self.check(TokenKind::RParen) {
                    self.parse_separated_elements(TokenKind::Comma, |p| p.parse_expr())?
                } else {
                    vec![]
                };

                let end_span = match self.stream_mut().expect(TokenKind::RParen) {
                    Ok(tok) => tok.span,
                    Err(e) => {
                        self.report(*e);
                        expr.span
                    }
                };

                let call_span = expr.span.merge(&end_span);
                expr = Spanned::new(
                    ExprKind::Call(Box::new(CallExpr { callee: expr, args })),
                    call_span,
                );
                continue;
            }

            break;
        }

        Some(expr)
    }

    fn parse_literal(&mut self) -> ParseResult<Expression> {
        let (kind, span) = match self.stream().peek() {
            Some(token) => (token.kind, token.span),
            None => return None,
        };

        match kind {
            TokenKind::Int => {
                self.stream_mut().bump();
                let text = span.slice(self.source_text);
                let value = text.parse::<i64>().unwrap_or(0);
                Some(Spanned::new(ExprKind::Literal(Literal::Int(value)), span))
            }
            TokenKind::Float => {
                self.stream_mut().bump();
                let text = span.slice(self.source_text);
                let value = text.parse::<f64>().unwrap_or(0.0);
                Some(Spanned::new(ExprKind::Literal(Literal::Float(value)), span))
            }
            TokenKind::Ident => {
                self.stream_mut().bump();
                let name = span.slice(self.source_text);
                Some(Spanned::new(
                    ExprKind::Ident(Spanned::new(name.to_string(), span)),
                    span,
                ))
            }
            TokenKind::String => {
                self.stream_mut().bump();
                let text = span.slice(self.source_text);

                let content = if text.len() >= 2 && text.starts_with('"') && text.ends_with('"') {
                    &text[1..text.len() - 1]
                } else {
                    text
                };

                let content = self.unescape(content);

                Some(Spanned::new(
                    ExprKind::Literal(Literal::String(content.to_string())),
                    span,
                ))
            }
            TokenKind::Boolean => {
                self.stream_mut().bump();
                let text = span.slice(self.source_text);
                Some(Spanned::new(
                    ExprKind::Literal(Literal::Bool(text == "true")),
                    span,
                ))
            }
            TokenKind::LBracket => {
                self.stream_mut().bump();

                if self.check(TokenKind::RBracket) {
                    let end_token = self.stream_mut().bump().unwrap();
                    let full_span = span.merge(&end_token.span);
                    return Some(Spanned::new(ExprKind::List(Vec::new()), full_span));
                }

                let mut elements = vec![];

                loop {
                    elements.push(self.parse_assign()?);

                    if !self.check(TokenKind::Comma) {
                        break;
                    }

                    self.stream_mut().bump();
                }

                let end_span = match self.stream_mut().expect(TokenKind::RBracket) {
                    Ok(tok) => tok.span,
                    Err(e) => {
                        self.report(*e);
                        self.stream().peek().map(|t| t.span).unwrap_or(span)
                    }
                };

                let full_span = span.merge(&end_span);
                Some(Spanned::new(ExprKind::List(elements), full_span))
            }
            TokenKind::LParen => {
                self.stream_mut().bump();

                if self.check(TokenKind::RParen) {
                    let end_token = self.stream_mut().bump().unwrap();
                    let full_span = span.merge(&end_token.span);
                    return Some(Spanned::new(ExprKind::List(Vec::new()), full_span));
                }

                let expr = self.parse_expr()?;

                let end_span = match self.stream_mut().expect(TokenKind::RParen) {
                    Ok(s) => s,
                    Err(e) => {
                        self.report(*e);
                        return Some(Spanned::new(ExprKind::Grouping(Box::new(expr)), span));
                    }
                }
                .span;

                let full_span = span.merge(&end_span);
                Some(Spanned::new(ExprKind::Grouping(Box::new(expr)), full_span))
            }
            _ => {
                self.report_error(ParseError::UnexpectedToken { found: kind }, span);
                self.stream_mut().bump();
                None
            }
        }
    }

    fn unescape(&self, input: &str) -> String {
        let mut res = String::with_capacity(input.len());
        let mut chars = input.chars();

        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('n') => res.push('\n'),
                    Some('t') => res.push('\t'),
                    Some('r') => res.push('\r'),
                    Some('\\') => res.push('\\'),
                    Some('"') => res.push('"'),
                    Some(other) => {
                        res.push('\\');
                        res.push(other);
                    }
                    None => res.push('\\'),
                }
            } else {
                res.push(c);
            }
        }
        res
    }

    fn parse_binary<F>(
        &mut self,
        mut parse_operand: F,
        strategy: OperandParseStrategy,
        valid_tokens: &[TokenKind],
    ) -> ParseResult<Expression>
    where
        F: FnMut(&mut Self) -> ParseResult<Expression>,
    {
        let mut lhs = parse_operand(self)?;

        loop {
            let is_op = self
                .stream
                .peek()
                .map(|t| valid_tokens.contains(&t.kind))
                .unwrap_or(false);

            if !is_op {
                break;
            }

            let op_token = self.stream.bump().unwrap().clone();

            let op = match BinaryOp::from_token_kind(op_token.kind) {
                Some(op) => op,
                None => {
                    self.report_error(
                        ParseError::UnexpectedToken {
                            found: op_token.kind,
                        },
                        op_token.span,
                    );
                    return None;
                }
            };

            let rhs = parse_operand(self)?;

            let span = lhs.span.merge(&rhs.span);
            lhs = Spanned::new(
                ExprKind::Binary(Box::new(BinaryExpr {
                    left: lhs,
                    op,
                    right: rhs,
                })),
                span,
            );

            if strategy == OperandParseStrategy::Single {
                break;
            }
        }

        Some(lhs)
    }

    fn parse_prefix<F>(
        &mut self,
        mut parse_operand: F,
        strategy: OperandParseStrategy,
        valid_tokens: &[TokenKind],
    ) -> ParseResult<Expression>
    where
        F: FnMut(&mut Self) -> ParseResult<Expression>,
    {
        let mut ops: Vec<(TokenKind, Span)> = Vec::new();

        loop {
            let is_op = self
                .stream
                .peek()
                .map(|t| valid_tokens.contains(&t.kind))
                .unwrap_or(false);

            if !is_op {
                break;
            }

            let tok = self.stream.bump().unwrap().clone();
            ops.push((tok.kind, tok.span));

            if strategy == OperandParseStrategy::Single {
                break;
            }
        }

        let mut expr = parse_operand(self)?;

        for (kind, op_span) in ops.into_iter().rev() {
            let span = op_span.merge(&expr.span);
            expr = Spanned::new(
                ExprKind::Unary(Box::new(UnaryExpr {
                    op: UnaryOp::from_token_kind(kind)?,
                    expr,
                })),
                span,
            );
        }

        Some(expr)
    }
}
