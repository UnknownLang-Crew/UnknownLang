#![allow(dead_code)]
use crate::unknown::ast::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
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

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.position + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += 1;
        Some(ch)
    }

    fn current_span(&self, start: usize) -> Span {
        Span::new(start, self.position)
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.advance();
        }
    }

    pub fn lex_all(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();
            let is_eof = token.kind == TokenKind::EOF;

            tokens.push(token);

            if is_eof {
                break;
            }
        }

        tokens
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let start = self.position;

        let ch = match self.peek() {
            Some(c) => c,
            None => {
                return Token::new(TokenKind::EOF, Span::new(self.position, self.position));
            }
        };

        // Numbers
        if ch.is_ascii_digit() {
            return self.lex_number();
        }

        // Identifiers
        if ch.is_ascii_alphabetic() || ch == '_' {
            return self.lex_identifier();
        }

        self.advance();

        let kind = match ch {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,

            '&' => {
                if self.peek() == Some('&') {
                    self.advance();
                    TokenKind::AndAnd
                } else {
                    TokenKind::Ampersand
                }
            }

            '|' => {
                if self.peek() == Some('|') {
                    self.advance();
                    TokenKind::OrOr
                } else {
                    TokenKind::Pipe
                }
            }

            '^' => TokenKind::Caret,
            '~' => TokenKind::Tilde,
            '!' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }

            '=' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::EqualEqual
                } else {
                    // Phase 0: no assignment yet → treat as error-ish fallback
                    TokenKind::EqualEqual
                }
            }

            '<' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::LessEqual
                } else if self.peek() == Some('<') {
                    self.advance();
                    TokenKind::LeftShift
                } else {
                    TokenKind::Less
                }
            }

            '>' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::GreaterEqual
                } else if self.peek() == Some('>') {
                    self.advance();
                    TokenKind::RightShift
                } else {
                    TokenKind::Greater
                }
            }

            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,

            '.' => {
                if self.peek() == Some('.') {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        TokenKind::DotDotEqual
                    } else {
                        TokenKind::DotDot
                    }
                } else {
                    // unknown single dot (not used yet)
                    TokenKind::DotDot
                }
            }

            _ => TokenKind::EOF, // placeholder fallback
        };

        Token::new(kind, Span::new(start, self.position))
    }

    fn lex_number(&mut self) -> Token {
        let start = self.position;

        let mut value = 0i64;

        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            let digit = self.advance().unwrap().to_digit(10).unwrap() as i64;
            value = value * 10 + digit;
        }

        Token::new(TokenKind::Integer(value), Span::new(start, self.position))
    }

    fn lex_identifier(&mut self) -> Token {
        let start = self.position;

        while matches!(self.peek(), Some(c) if c.is_ascii_alphanumeric() || c == '_') {
            self.advance();
        }

        let text: String = self.chars[start..self.position].iter().collect();

        Token::new(TokenKind::Identifier(text), Span::new(start, self.position))
    }
}