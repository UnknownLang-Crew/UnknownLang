use crate::unknown::ast::{Expr, ExprKind, UnaryOp, BinaryOp};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(exprs: &[Expr]) {
        for expr in exprs {
            Self::print_expr(expr, "".to_string(), true);
        }
    }

    fn print_expr(expr: &Expr, prefix: String, is_last: bool) {
        let (label, children) = Self::node_info(expr);

        println!(
            "{}{}{}",
            prefix,
            if is_last { "└── " } else { "├── " },
            label
        );

        let new_prefix = prefix + if is_last { "    " } else { "│   " };

        for (i, child) in children.iter().enumerate() {
            let last = i == children.len() - 1;
            Self::print_expr(child, new_prefix.clone(), last);
        }
    }

    fn node_info(expr: &Expr) -> (String, Vec<&Expr>) {
        match &expr.kind {
            ExprKind::Integer(v) => {
                (format!("Integer({})", v), vec![])
            }

            ExprKind::Identifier(name) => {
                (format!("Identifier({})", name), vec![])
            }

            ExprKind::Unary { operator, operand } => {
                (
                    format!("Unary({})", Self::unary_name(*operator)),
                    vec![operand],
                )
            }

            ExprKind::Binary { left, operator, right } => {
                (
                    format!("Binary({})", Self::binary_name(*operator)),
                    vec![left, right],
                )
            }

            ExprKind::Grouping(expr) => {
                ("Grouping".to_string(), vec![expr])
            }
        }
    }

    fn unary_name(op: UnaryOp) -> &'static str {
        match op {
            UnaryOp::Negate => "-",
            UnaryOp::LogicalNot => "!",
            UnaryOp::BitwiseNot => "~",
        }
    }

    fn binary_name(op: BinaryOp) -> &'static str {
        match op {
            BinaryOp::Plus => "+",
            BinaryOp::Subtract => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
            BinaryOp::Modulo => "%",

            BinaryOp::Equal => "==",
            BinaryOp::NotEqual => "!=",

            BinaryOp::Less => "<",
            BinaryOp::LessEqual => "<=",

            BinaryOp::Greater => ">",
            BinaryOp::GreaterEqual => ">=",

            BinaryOp::LogicalAnd => "&&",
            BinaryOp::LogicalOr => "||",

            BinaryOp::LeftShift => "<<",
            BinaryOp::RightShift => ">>",

            BinaryOp::BitwiseAnd => "&",
            BinaryOp::BitwiseOr => "|",
            BinaryOp::BitwiseXor => "^",

            BinaryOp::RangeExclusive => "..",
            BinaryOp::RangeInclusive => "..=",
        }
    }
}