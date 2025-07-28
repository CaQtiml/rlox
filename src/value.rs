use std::fmt;
use crate::function::LoxFunction;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Function(LoxFunction),
}

impl Value {
    // Helper methods you'll need
    pub fn is_truthy(&self) -> bool {
        // TODO: Implement Lox's truthiness rules
        // In Lox: nil and false are falsy, everything else is truthy
        match self {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
        }
    }
    
    pub fn is_equal(&self, other: &Value) -> bool {
        // TODO: Implement equality comparison
        // This is different from Rust's PartialEq - it's Lox's equality rules
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Number(a), Value::Number(b)) => a==b,
            (Value::String(a), Value::String(b)) => a==b,
            (Value::Boolean(a), Value::Boolean(b)) => a==b,
            (Value::Function(a), Value::Function(b)) => a == b,
            _ => false,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Implement display formatting for Lox values
        // Numbers should print without trailing .0 if they're whole numbers
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            },
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Function(func) => write!(f, "<fn {}>", func.name()),
            Value::Nil => write!(f, "nil"),
        }
    }
}