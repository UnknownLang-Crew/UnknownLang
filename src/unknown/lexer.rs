#![allow(dead_code)]

use crate::unknown::ast::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Integer(i64),
    Identifier(String),

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    Ampersand,
    Pipe,
    Caret,

    LeftShift,
    RightShift,

    Bang,
    Tilde,

    EqualEqual,
    BangEqual,

    Less,
    LessEqual,

    Greater,
    GreaterEqual,

    AndAnd,
    OrOr,

    DotDot,
    DotDotEqual,

    LeftParen,
    RightParen,

    EOF,
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: Vec<char>,
    position: usize,
}

