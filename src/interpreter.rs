use crate::expr::{Expr, ExprVisitor};
use crate::token::{Token, TokenType, LiteralValue};
use crate::value::Value;
use anyhow::{anyhow, Ok, Result};

pub struct Interpreter;

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Runtime Error: {}", self.token.line, self.message)
    }
}

impl std::error::Error for RuntimeError {}

impl Interpreter {
    pub fn new() -> Self {
        Self
    }
    
    pub fn interpret(&mut self, expr: &Expr) -> Result<Value> {
        // TODO: Use the visitor pattern to evaluate the expression
        // Call expr.accept(self) and handle any runtime errors
        expr.accept(self) 
        // Basically peeling one layer out 
        // and take the peeled expression to the accept function for further evaluation
    }
    
    fn runtime_error(&self, token: &Token, message: &str) -> anyhow::Error {
        RuntimeError {
            token: token.clone(),
            message: message.to_string(),
        }.into()
    }

    fn check_number_operand(&self, operator: &Token, operand: &Value) -> Result<f64> {
        match operand {
            Value::Number(n) => Ok(*n),
            _ => Err(self.runtime_error(operator, "Operand must be a number.")),
        }
    }

    fn check_number_operands(&self, operator: &Token, left: &Value, right: &Value) -> Result<(f64, f64)> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok((*l, *r)),
            _ => Err(self.runtime_error(operator, "Operands must be numbers.")),
        }
    }
}

impl ExprVisitor<Result<Value>> for Interpreter {
    fn visit_literal_expr(&mut self, _expr: &Expr, value: &Option<LiteralValue>) -> Result<Value> {
        // TODO: Convert LiteralValue to Value
        // This should be straightforward mapping
        match value {
            Some(LiteralValue::Boolean(b)) => Ok(Value::Boolean(*b)),
            Some(LiteralValue::Nil) | None => Ok(Value::Nil),
            Some(LiteralValue::Number(n)) => Ok(Value::Number(*n)),
            Some(LiteralValue::String(s)) => Ok(Value::String(s.clone())),
        }
    }

    fn visit_grouping_expr(&mut self, _expr: &Expr, expression: &Expr) -> Result<Value> {
        // TODO: Just evaluate the inner expression
        expression.accept(self)
    }

    fn visit_unary_expr(&mut self, _expr: &Expr, operator: &Token, right: &Expr) -> Result<Value> {
        // TODO: Evaluate the right operand first, then apply the operator
        // Handle TokenType::Bang and TokenType::Minus
        // Remember to check types and throw runtime errors for invalid operations
        let mut right_value = right.accept(self)?;
        match operator.token_type {
            TokenType::Bang => {
                Ok(Value::Boolean(!right_value.is_truthy()))
            },
            TokenType::Minus => {
                let num = self.check_number_operand(operator, &right_value)?;
                Ok(Value::Number(-num))
            },
            _ => Err(anyhow!("Unknown unary operator: {:?}", operator.token_type)),
        }
    }

    fn visit_binary_expr(&mut self, _expr: &Expr, left: &Expr, operator: &Token, right: &Expr) -> Result<Value> {
        // TODO: This is the big one! Handle all binary operators:
        // - Arithmetic: +, -, *, /
        // - Comparison: >, >=, <, <=
        // - Equality: ==, !=
        // 
        // Special cases to handle:
        // - Division by zero
        // - String concatenation with +
        // - Type checking for arithmetic operations
        let left_value = left.accept(self)?;
        let right_value = right.accept(self)?;

        match operator.token_type {
            // Arithmetic operators
            TokenType::Plus => {
                // Special case: + can be arithmetic OR string concatenation
                match (&left_value, &right_value) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
                    // In Lox, if either operand is a string, both are converted to strings
                    (Value::String(l), r) => Ok(Value::String(format!("{}{}", l, r))),
                    (l, Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
                    _ => Err(self.runtime_error(operator, "Operands must be two numbers or two strings.")),
                }
            }
            TokenType::Minus => {
                let (l, r) = self.check_number_operands(operator, &left_value, &right_value)?;
                Ok(Value::Number(l - r))
            }
            TokenType::Star => {
                let (l, r) = self.check_number_operands(operator, &left_value, &right_value)?;
                Ok(Value::Number(l * r))
            }
            TokenType::Slash => {
                let (l, r) = self.check_number_operands(operator, &left_value, &right_value)?;
                if r == 0.0 {
                    return Err(self.runtime_error(operator, "Division by zero."));
                }
                Ok(Value::Number(l / r))
            }

            // Comparison operators (only for numbers)
            TokenType::Greater => {
                let (l, r) = self.check_number_operands(operator, &left_value, &right_value)?;
                Ok(Value::Boolean(l > r))
            }
            TokenType::GreaterEqual => {
                let (l, r) = self.check_number_operands(operator, &left_value, &right_value)?;
                Ok(Value::Boolean(l >= r))
            }
            TokenType::Less => {
                let (l, r) = self.check_number_operands(operator, &left_value, &right_value)?;
                Ok(Value::Boolean(l < r))
            }
            TokenType::LessEqual => {
                let (l, r) = self.check_number_operands(operator, &left_value, &right_value)?;
                Ok(Value::Boolean(l <= r))
            }

            // Equality operators (work on any types)
            TokenType::EqualEqual => {
                Ok(Value::Boolean(left_value.is_equal(&right_value)))
            }
            TokenType::BangEqual => {
                Ok(Value::Boolean(!left_value.is_equal(&right_value)))
            }

            _ => Err(anyhow!("Unknown binary operator: {:?}", operator.token_type)),
        }
    }
}