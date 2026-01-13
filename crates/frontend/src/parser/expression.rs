use common::{
    ast::{
        Expression, Spanned,
        expression::{BinaryExpr, BinaryOp, ExprKind, Literal, UnaryExpr, UnaryOp},
    },
    token::TokenKind,
};

use crate::parser::{ParseResult, Parser, error::ParseError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinaryParseStrategy {
    Single,
    Multiple,
}

impl<'a> Parser<'a> {
    pub(super) fn parse_expr(&mut self) -> ParseResult<Expression> {
        let expr = self.parse_equality()?;
        Some(expr)
    }

    fn parse_equality(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |parser| parser.parse_comparison(),
            BinaryParseStrategy::Single,
            &[TokenKind::EqualEqual, TokenKind::BangEqual],
        )
    }

    fn parse_comparison(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |parser| parser.parse_range(),
            BinaryParseStrategy::Single,
            &[
                TokenKind::Greater,
                TokenKind::GreaterEq,
                TokenKind::Less,
                TokenKind::LessEq,
            ],
        )
    }

    fn parse_range(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |parser| parser.parse_term(),
            BinaryParseStrategy::Single,
            &[TokenKind::Range],
        )
    }

    fn parse_term(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |parser| parser.parse_factor(),
            BinaryParseStrategy::Multiple,
            &[TokenKind::Plus, TokenKind::Minus],
        )
    }

    fn parse_factor(&mut self) -> ParseResult<Expression> {
        self.parse_binary(
            |parser| parser.parse_unary(),
            BinaryParseStrategy::Multiple,
            &[TokenKind::Star, TokenKind::Slash],
        )
    }

    fn parse_unary(&mut self) -> ParseResult<Expression> {
        if !self.check_unary_op() {
            return self.parse_literal();
        }

        let (op_kind, op_span) = match self.stream_mut().bump() {
            Some(tok) => (tok.kind, tok.span),
            None => return None,
        };

        let expr = self.parse_unary()?;

        let span = op_span.merge(&expr.span);

        Some(Spanned::new(
            ExprKind::Unary(Box::new(UnaryExpr {
                op: UnaryOp::from_token_kind(op_kind)?,
                expr,
            })),
            span,
        ))
    }

    fn parse_literal(&mut self) -> ParseResult<Expression> {
        let (kind, span) = match self.stream().peek() {
            Some(token) => (token.kind, token.span),
            None => return None,
        };

        match kind {
            TokenKind::IntLiteral => {
                self.stream_mut().bump();
                let text = span.slice(self.source_text);
                let value = text.parse::<i64>().unwrap_or(0);
                Some(Spanned::new(ExprKind::Literal(Literal::Int(value)), span))
            }
            TokenKind::FloatLiteral => {
                self.stream_mut().bump();
                let text = span.slice(self.source_text);
                let value = text.parse::<f64>().unwrap_or(0.0);
                Some(Spanned::new(ExprKind::Literal(Literal::Float(value)), span))
            }
            TokenKind::StringLiteral => {
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
            TokenKind::BoolLiteral(b) => {
                self.stream_mut().bump();
                Some(Spanned::new(ExprKind::Literal(Literal::Bool(b)), span))
            }
            _ => {
                self.report_error(ParseError::UnexpectedToken { found: kind }, span);
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
        strategy: BinaryParseStrategy,
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

            let op_token = self.stream.bump().unwrap();
            let op = BinaryOp::from_token_kind(op_token.kind)?;

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

            if strategy == BinaryParseStrategy::Single {
                break;
            }
        }

        Some(lhs)
    }

    fn check_unary_op(&self) -> bool {
        self.try_expect(&[TokenKind::Minus, TokenKind::Plus, TokenKind::Not])
    }
}
