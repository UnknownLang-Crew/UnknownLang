use crate::unknown::ast::{Expr, ExprKind, Span};
use crate::unknown::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens[self.position].clone();
        self.position += 1;
        tok
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::EOF)
    }

    // -------------------------
    // Entry point
    // -------------------------

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut exprs = Vec::new();

        while !self.is_at_end() {
            exprs.push(self.parse_expression(0));
        }

        exprs
    }

    // -------------------------
    // Pratt parser core
    // -------------------------

    fn parse_expression(&mut self, min_prec: u8) -> Expr {
        let mut left = self.parse_prefix();

        while !self.is_at_end() {
            let prec = match self.infix_precedence() {
                Some(p) => p,
                None => break,
            };

            if prec < min_prec {
                break;
            }

            left = self.parse_infix(left, prec);
        }

        left
    }

    // -------------------------
    // Prefix (nud)
    // -------------------------

    fn parse_prefix(&mut self) -> Expr {
        let token = self.advance().clone();

        match token.kind {
            TokenKind::Integer(value) => Expr::integer(value, token.span),
            TokenKind::Float(value) => Expr::float(value, token.span),

            TokenKind::Identifier(name) => Expr::identifier(name, token.span),

            TokenKind::Let => {
                let name = self.expect_identifier();
                self.expect(TokenKind::Equal);
                let value = self.parse_expression(0);
                let span = Span::merge(token.span, value.span);

                Expr {
                    kind: ExprKind::Let {
                        name,
                        value: Box::new(value),
                    },
                    span,
                }
            }

            TokenKind::Minus => {
                let rhs = self.parse_expression(100);
                Expr::unary(crate::unknown::ast::UnaryOp::Negate, rhs, token.span)
            }

            TokenKind::Bang => {
                let rhs = self.parse_expression(100);
                Expr::unary(crate::unknown::ast::UnaryOp::LogicalNot, rhs, token.span)
            }

            TokenKind::Tilde => {
                let rhs = self.parse_expression(100);
                Expr::unary(crate::unknown::ast::UnaryOp::BitwiseNot, rhs, token.span)
            }

            TokenKind::LeftParen => {
                let expr = self.parse_expression(0);
                self.expect(TokenKind::RightParen);
                expr
            }

            _ => panic!("Unexpected token in prefix position: {:?}", token.kind),
        }
    }

    // -------------------------
    // Infix (led)
    // -------------------------

    fn parse_infix(&mut self, left: Expr, prec: u8) -> Expr {
        let token = self.advance().clone();

        if matches!(token.kind, TokenKind::Equal) {
            let right = self.parse_expression(prec);
            let span = Span::merge(left.span, right.span);

            return match left.kind {
                ExprKind::Identifier(name) => Expr {
                    kind: ExprKind::Assign {
                        name,
                        value: Box::new(right),
                    },
                    span,
                },
                _ => panic!("Invalid assignment target"),
            };
        }

        let right = self.parse_expression(prec + 1);

        let op = match token.kind {
            TokenKind::Plus => crate::unknown::ast::BinaryOp::Plus,
            TokenKind::Minus => crate::unknown::ast::BinaryOp::Subtract,
            TokenKind::Star => crate::unknown::ast::BinaryOp::Multiply,
            TokenKind::Slash => crate::unknown::ast::BinaryOp::Divide,
            TokenKind::Percent => crate::unknown::ast::BinaryOp::Modulo,

            TokenKind::EqualEqual => crate::unknown::ast::BinaryOp::Equal,
            TokenKind::BangEqual => crate::unknown::ast::BinaryOp::NotEqual,

            TokenKind::Less => crate::unknown::ast::BinaryOp::Less,
            TokenKind::LessEqual => crate::unknown::ast::BinaryOp::LessEqual,

            TokenKind::Greater => crate::unknown::ast::BinaryOp::Greater,
            TokenKind::GreaterEqual => crate::unknown::ast::BinaryOp::GreaterEqual,

            TokenKind::AndAnd => crate::unknown::ast::BinaryOp::LogicalAnd,
            TokenKind::OrOr => crate::unknown::ast::BinaryOp::LogicalOr,

            TokenKind::LeftShift => crate::unknown::ast::BinaryOp::LeftShift,
            TokenKind::RightShift => crate::unknown::ast::BinaryOp::RightShift,

            TokenKind::Ampersand => crate::unknown::ast::BinaryOp::BitwiseAnd,
            TokenKind::Pipe => crate::unknown::ast::BinaryOp::BitwiseOr,
            TokenKind::Caret => crate::unknown::ast::BinaryOp::BitwiseXor,

            TokenKind::DotDot => crate::unknown::ast::BinaryOp::RangeExclusive,
            TokenKind::DotDotEqual => crate::unknown::ast::BinaryOp::RangeInclusive,

            _ => panic!("Unexpected infix operator: {:?}", token.kind),
        };

        Expr::binary(
            left.clone(),
            op,
            right.clone(),
            Span::merge(left.span, right.span),
        )
    }

    // -------------------------
    // Precedence table
    // -------------------------

    fn infix_precedence(&self) -> Option<u8> {
        match self.peek().kind {
            TokenKind::Equal => Some(0),

            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Some(80),

            TokenKind::Plus | TokenKind::Minus => Some(70),

            TokenKind::LeftShift | TokenKind::RightShift => Some(60),

            TokenKind::Ampersand => Some(50),

            TokenKind::Caret => Some(40),

            TokenKind::Pipe => Some(30),

            TokenKind::EqualEqual
            | TokenKind::BangEqual
            | TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual => Some(20),

            TokenKind::AndAnd => Some(10),

            TokenKind::OrOr => Some(5),

            TokenKind::DotDot | TokenKind::DotDotEqual => Some(1),

            _ => None,
        }
    }

    // -------------------------
    // Helpers
    // -------------------------

    fn expect(&mut self, expected: TokenKind) {
        let tok = self.advance();
        if std::mem::discriminant(&tok.kind) != std::mem::discriminant(&expected) {
            panic!("Expected {:?}, got {:?}", expected, tok.kind);
        }
    }

    fn expect_identifier(&mut self) -> String {
        let tok = self.advance();
        match tok.kind {
            TokenKind::Identifier(name) => name,
            _ => panic!("Expected identifier, got {:?}", tok.kind),
        }
    }
}
