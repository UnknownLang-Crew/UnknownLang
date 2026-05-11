use std::collections::HashMap;
use std::fmt;

use crate::unknown::ast::{BinaryOp, Expr, ExprKind, UnaryOp};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl Value {
    fn as_number(&self) -> Result<f64, String> {
        match self {
            Self::Integer(value) => Ok(*value as f64),
            Self::Float(value) => Ok(*value),
            Self::String(_) | Self::Boolean(_) => Err("Expected number".to_string()),
        }
    }

    fn as_integer(&self) -> Result<i64, String> {
        match self {
            Self::Integer(value) => Ok(*value),
            Self::String(_) | Self::Float(_) | Self::Boolean(_) => {
                Err("Expected integer".to_string())
            }
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Self::Integer(value) => *value != 0,
            Self::Float(value) => *value != 0.0,
            Self::String(value) => !value.is_empty(),
            Self::Boolean(value) => *value,
        }
    }

    fn equals(&self, other: &Self) -> Result<bool, String> {
        match (self, other) {
            (Self::Boolean(left), Self::Boolean(right)) => Ok(left == right),
            (Self::String(left), Self::String(right)) => Ok(left == right),
            (Self::Boolean(_), _) | (_, Self::Boolean(_)) => Ok(false),
            (Self::String(_), _) | (_, Self::String(_)) => Ok(false),
            _ => Ok(self.as_number()? == other.as_number()?),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, "{}", value),
            Self::Float(value) => write!(f, "{}", value),
            Self::Boolean(value) => write!(f, "{}", value),
        }
    }
}

/// ----------------------
/// Environment
/// ----------------------
pub struct Env {
    vars: HashMap<String, Value>,
    mutable: HashMap<String, bool>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            mutable: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.vars.get(name).cloned()
    }

    pub fn set_let(&mut self, name: String, value: Value) {
        self.vars.insert(name.clone(), value);
        self.mutable.insert(name, false);
    }

    #[allow(dead_code)]
    pub fn set_mut(&mut self, name: String, value: Value) {
        self.vars.insert(name.clone(), value);
        self.mutable.insert(name, true);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if !self.vars.contains_key(name) {
            return Err(format!("Undefined variable: {}", name));
        }

        if !self.mutable.get(name).copied().unwrap_or(false) {
            return Err(format!("Cannot assign to immutable variable: {}", name));
        }

        self.vars.insert(name.to_string(), value);
        Ok(())
    }
}

/// ----------------------
/// Interpreter
/// ----------------------
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self, program: &[Expr], env: &mut Env) {
        for expr in program {
            match self.eval(expr, env) {
                Ok(val) => {
                    println!("{}", val);
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
        }
    }

    pub fn eval(&mut self, expr: &Expr, env: &mut Env) -> Result<Value, String> {
        match &expr.kind {
            ExprKind::Integer(v) => Ok(Value::Integer(*v)),
            ExprKind::Float(v) => Ok(Value::Float(*v)),
            ExprKind::String(v) => Ok(Value::String(v.clone())),

            ExprKind::Identifier(name) => env
                .get(name)
                .ok_or_else(|| format!("Undefined variable: {}", name)),

            ExprKind::Grouping(expr) => self.eval(expr, env),

            // let x = ...
            ExprKind::Let { name, value } => {
                let val = self.eval(value, env)?;
                env.set_let(name.clone(), val.clone());
                Ok(val)
            }

            // x = ...
            ExprKind::Assign { name, value } => {
                let val = self.eval(value, env)?;
                env.assign(name, val.clone())?;
                Ok(val)
            }

            ExprKind::Unary { operator, operand } => {
                let v = self.eval(operand, env)?;

                let result = match operator {
                    UnaryOp::Negate => match v {
                        Value::Integer(value) => Value::Integer(-value),
                        Value::Float(value) => Value::Float(-value),
                        Value::String(_) => return Err("Expected String as Number".to_string()),
                        Value::Boolean(_) => return Err("Expected number".to_string()),
                    },
                    UnaryOp::LogicalNot => Value::Boolean(!v.is_truthy()),
                    UnaryOp::BitwiseNot => Value::Integer(!v.as_integer()?),
                };

                Ok(result)
            }

            ExprKind::Binary {
                left,
                operator,
                right,
            } => {
                let l = self.eval(left, env)?;
                let r = self.eval(right, env)?;

                let result = match operator {
                    BinaryOp::Plus => match (&l, &r) {
                        (Value::Integer(left), Value::Integer(right)) => {
                            Value::Integer(left + right)
                        }
                        (Value::String(left), Value::String(right)) => {
                            Value::String(format!("{}{}", left, right))
                        }
                        _ => Value::Float(l.as_number()? + r.as_number()?),
                    },
                    BinaryOp::Subtract => match (&l, &r) {
                        (Value::Integer(left), Value::Integer(right)) => {
                            Value::Integer(left - right)
                        }
                        _ => Value::Float(l.as_number()? - r.as_number()?),
                    },
                    BinaryOp::Multiply => match (&l, &r) {
                        (Value::Integer(left), Value::Integer(right)) => {
                            Value::Integer(left * right)
                        }
                        _ => Value::Float(l.as_number()? * r.as_number()?),
                    },
                    BinaryOp::Divide => Value::Float(l.as_number()? / r.as_number()?),
                    BinaryOp::Modulo => Value::Integer(l.as_integer()? % r.as_integer()?),

                    BinaryOp::Equal => Value::Boolean(l.equals(&r)?),
                    BinaryOp::NotEqual => Value::Boolean(!l.equals(&r)?),
                    BinaryOp::Less => Value::Boolean(l.as_number()? < r.as_number()?),
                    BinaryOp::LessEqual => Value::Boolean(l.as_number()? <= r.as_number()?),
                    BinaryOp::Greater => Value::Boolean(l.as_number()? > r.as_number()?),
                    BinaryOp::GreaterEqual => Value::Boolean(l.as_number()? >= r.as_number()?),

                    BinaryOp::LogicalAnd => Value::Boolean(l.is_truthy() && r.is_truthy()),
                    BinaryOp::LogicalOr => Value::Boolean(l.is_truthy() || r.is_truthy()),

                    BinaryOp::BitwiseAnd => Value::Integer(l.as_integer()? & r.as_integer()?),
                    BinaryOp::BitwiseOr => Value::Integer(l.as_integer()? | r.as_integer()?),
                    BinaryOp::BitwiseXor => Value::Integer(l.as_integer()? ^ r.as_integer()?),

                    BinaryOp::LeftShift => Value::Integer(l.as_integer()? << r.as_integer()?),
                    BinaryOp::RightShift => Value::Integer(l.as_integer()? >> r.as_integer()?),

                    _ => return Err("unsupported operator".to_string()),
                };

                Ok(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::unknown::ast::{BinaryOp, Expr, Span};

    use super::{Env, Interpreter, Value};

    #[test]
    fn division_returns_float() {
        let expr = Expr::binary(
            Expr::integer(10, Span::default()),
            BinaryOp::Divide,
            Expr::integer(3, Span::default()),
            Span::default(),
        );

        let mut interpreter = Interpreter::new();
        let mut env = Env::new();

        assert_eq!(
            interpreter.eval(&expr, &mut env),
            Ok(Value::Float(10.0 / 3.0))
        );
        assert_eq!(
            interpreter.eval(&expr, &mut env).unwrap().to_string(),
            (10.0_f64 / 3.0).to_string()
        );
    }

    #[test]
    fn float_display_keeps_precision_artifacts() {
        let expr = Expr::binary(
            Expr {
                kind: crate::unknown::ast::ExprKind::Float(0.1),
                span: Span::default(),
            },
            BinaryOp::Plus,
            Expr {
                kind: crate::unknown::ast::ExprKind::Float(0.2),
                span: Span::default(),
            },
            Span::default(),
        );

        let mut interpreter = Interpreter::new();
        let mut env = Env::new();

        assert_eq!(
            interpreter.eval(&expr, &mut env).unwrap().to_string(),
            "0.30000000000000004"
        );
    }

    #[test]
    fn string_literals_evaluate_to_strings() {
        let expr = Expr::string("hello".to_string(), Span::default());

        let mut interpreter = Interpreter::new();
        let mut env = Env::new();

        assert_eq!(
            interpreter.eval(&expr, &mut env),
            Ok(Value::String("hello".to_string()))
        );
    }

    #[test]
    fn plus_concatenates_strings() {
        let expr = Expr::binary(
            Expr::string("hello ".to_string(), Span::default()),
            BinaryOp::Plus,
            Expr::string("world".to_string(), Span::default()),
            Span::default(),
        );

        let mut interpreter = Interpreter::new();
        let mut env = Env::new();

        assert_eq!(
            interpreter.eval(&expr, &mut env),
            Ok(Value::String("hello world".to_string()))
        );
    }

    #[test]
    fn strings_compare_by_value() {
        let expr = Expr::binary(
            Expr::string("hello".to_string(), Span::default()),
            BinaryOp::Equal,
            Expr::string("hello".to_string(), Span::default()),
            Span::default(),
        );

        let mut interpreter = Interpreter::new();
        let mut env = Env::new();

        assert_eq!(interpreter.eval(&expr, &mut env), Ok(Value::Boolean(true)));
    }
}
