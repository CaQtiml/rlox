use crate::expr::{Expr, ExprVisitor};
use crate::token::{Token, LiteralValue};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        Self
    }

    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
        // Expr::accept(expr, self)
        // DON'T ADD ; at the end of return expression!
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        // TODO: Implement this helper method
        // Hint: Format as "(name expr1 expr2 ...)"
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
        // TODO: Use parenthesize to format binary expressions
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_unary_expr(&mut self, _expr: &Expr, operator: &Token, right: &Expr) -> String {
        // TODO: Use parenthesize to format unary expressions
        self.parenthesize(&operator.lexeme, &[right])
    }

    fn visit_literal_expr(&mut self, _expr: &Expr, value: &Option<LiteralValue>) -> String {
        // TODO: Convert literal values to strings
        match value {
            Some(LiteralValue::String(s)) => s.clone(),
            Some(LiteralValue::Number(n)) => n.to_string(),
            Some(LiteralValue::Boolean(b)) => b.to_string(),
            Some(LiteralValue::Nil) => "nil".to_string(),
            None => "nil".to_string(),
        }
    }

    fn visit_grouping_expr(&mut self, _expr: &Expr, expression: &Expr) -> String {
        // TODO: Use parenthesize with "group"
        self.parenthesize("group", &[expression])
    }
}