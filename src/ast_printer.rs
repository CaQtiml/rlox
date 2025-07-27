use crate::expr::{Expr, ExprVisitor};
use crate::token::{Token, LiteralValue};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        Self
    }

    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        let mut result = format!("({}", name);
        
        for expr in exprs {
            result.push(' ');
            result.push_str(&expr.accept(self));
        }
        
        result.push(')');
        result
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, _expr: &Expr, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_unary_expr(&mut self, _expr: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[right])
    }

    fn visit_literal_expr(&mut self, _expr: &Expr, value: &Option<LiteralValue>) -> String {
        match value {
            Some(LiteralValue::String(s)) => s.clone(),
            Some(LiteralValue::Number(n)) => n.to_string(),
            Some(LiteralValue::Boolean(b)) => b.to_string(),
            Some(LiteralValue::Nil) => "nil".to_string(),
            None => "nil".to_string(),
        }
    }

    fn visit_grouping_expr(&mut self, _expr: &Expr, expression: &Expr) -> String {
        self.parenthesize("group", &[expression])
    }

    fn visit_variable_expr(&mut self, _expr: &Expr, name: &Token) -> String {
        name.lexeme.clone()
    }

    fn visit_assign_expr(&mut self, _expr: &Expr, name: &Token, value: &Expr) -> String {
        format!("(= {} {})", name.lexeme, value.accept(self))
    }

    fn visit_logical_expr(&mut self, _expr: &Expr, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
    }
}