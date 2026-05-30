#![allow(dead_code)]

//
// Program
//

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub items: Vec<Expr>,
}

impl Program {
    pub fn new(items: Vec<Expr>) -> Self {
        Self { items }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

//
// Expressions
//

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn integer(value: i64, span: Span) -> Self {
        Self {
            kind: ExprKind::Integer(value),
            span,
        }
    }

    pub fn float(value: f64, span: Span) -> Self {
        Self {
            kind: ExprKind::Float(value),
            span,
        }
    }

    pub fn string(value: String, span: Span) -> Self {
        Self {
            kind: ExprKind::String(value),
            span,
        }
    }

    pub fn boolean(value: bool, span: Span) -> Self {
        Self {
            kind: ExprKind::Boolean(value),
            span,
        }
    }

    pub fn identifier(name: impl Into<String>, span: Span) -> Self {
        Self {
            kind: ExprKind::Identifier(name.into()),
            span,
        }
    }

    pub fn grouping(expr: Expr, span: Span) -> Self {
        Self {
            kind: ExprKind::Grouping(Box::new(expr)),
            span,
        }
    }

    pub fn unary(operator: UnaryOp, operand: Expr, span: Span) -> Self {
        Self {
            kind: ExprKind::Unary {
                operator,
                operand: Box::new(operand),
            },
            span,
        }
    }

    pub fn binary(left: Expr, operator: BinaryOp, right: Expr, span: Span) -> Self {
        Self {
            kind: ExprKind::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            },
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Identifier(String),

    Grouping(Box<Expr>),

    Let {
        name: String,
        value: Box<Expr>,
    },

    Assign {
        name: String,
        value: Box<Expr>,
    },

    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },

    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
    },

    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
}

//
// Operators
//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Negate,     // -
    LogicalNot, // !
    BitwiseNot, // ~
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BinaryOp {
    // Arithmetic
    Plus,     // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %

    // Bitwise
    BitwiseAnd, // &
    BitwiseOr,  // |
    BitwiseXor, // ^

    LeftShift,  // <<
    RightShift, // >>

    // Comparison
    Equal,    // ==
    NotEqual, // !=

    Less,      // <
    LessEqual, // <=

    Greater,      // >
    GreaterEqual, // >=

    // Logical
    LogicalAnd, // &&
    LogicalOr,  // ||

    // Range
    RangeExclusive, // ..
    RangeInclusive, // ..=
}

impl BinaryOp {
    pub fn precedence(self) -> u8 {
        match self {
            Self::Multiply | Self::Divide | Self::Modulo => 80,

            Self::Plus | Self::Subtract => 70,

            Self::LeftShift | Self::RightShift => 60,

            Self::BitwiseAnd => 50,
            Self::BitwiseXor => 40,
            Self::BitwiseOr => 30,

            Self::Equal
            | Self::NotEqual
            | Self::Less
            | Self::LessEqual
            | Self::Greater
            | Self::GreaterEqual => 20,

            Self::LogicalAnd => 10,
            Self::LogicalOr => 5,

            Self::RangeExclusive | Self::RangeInclusive => 1,
        }
    }
}

//
// Source Spans
//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(self) -> bool {
        self.start == self.end
    }

    pub fn merge(a: Span, b: Span) -> Self {
        Self {
            start: a.start.min(b.start),
            end: a.end.max(b.end),
        }
    }
}
