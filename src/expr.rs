use crate::token::{LiteralValue, Token};

// This will be your main expression enum
#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Literal {
        value: Option<LiteralValue>,
    },
    Grouping {
        expression: Box<Expr>,
    },
}

// You'll need this trait for the Visitor pattern
pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, expr: &Expr, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_unary_expr(&mut self, expr: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_literal_expr(&mut self, expr: &Expr, value: &Option<LiteralValue>) -> T;
    fn visit_grouping_expr(&mut self, expr: &Expr, expression: &Expr) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        // TODO: Implement this method
        // Hint: Match on self and call the appropriate visitor method
        match self {
            Expr::Binary { left, operator, right } => {
                visitor.visit_binary_expr(self, left, operator, right)
            }
            Expr::Grouping { expression } => {
                visitor.visit_grouping_expr(self, expression)
            }
            Expr::Literal { value } =>  {
                visitor.visit_literal_expr(self, value)
            }
            Expr::Unary { operator, right } => {
                visitor.visit_unary_expr(self, operator, right)
            }
        }
    }

    // Constructor helper methods
    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn unary(operator: Token, right: Expr) -> Self {
        Expr::Unary {
            operator,
            right: Box::new(right),
        }
    }

    pub fn literal(value: Option<LiteralValue>) -> Self {
        Expr::Literal { value }
    }

    pub fn grouping(expression: Expr) -> Self {
        Expr::Grouping {
            expression: Box::new(expression),
        }
    }
}