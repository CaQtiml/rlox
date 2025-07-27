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
    /*
    Expr::Variable {
        name: Token {
            token_type: TokenType::Identifier,
            lexeme: "x".to_string(),
            literal: None,
            line: 1,
        }
    }
    */
    Variable {
        name: Token,
    },
    Assign {
        name: Token,
        value: Box<Expr>
    },
    Logical { // Don't use Binary because we want to shortcut the case (True or ...) and (False and ...)
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}

// You'll need this trait for the Visitor pattern
pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, expr: &Expr, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_unary_expr(&mut self, expr: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_literal_expr(&mut self, expr: &Expr, value: &Option<LiteralValue>) -> T;
    fn visit_grouping_expr(&mut self, expr: &Expr, expression: &Expr) -> T;
    fn visit_variable_expr(&mut self, expr: &Expr, name: &Token) -> T;
    fn visit_assign_expr(&mut self, expr: &Expr, name: &Token, value: &Expr) -> T;
    fn visit_logical_expr(&mut self, expr: &Expr, left: &Expr, operator: &Token, right: &Expr) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        // TODO: Implement this method
        // Hint: Match on self and call the appropriate visitor method
        match self {
            Expr::Binary { left, operator, right } => {
                visitor.visit_binary_expr(self, left, operator, right)
            },
            Expr::Grouping { expression } => {
                visitor.visit_grouping_expr(self, expression)
            },
            Expr::Literal { value } =>  {
                visitor.visit_literal_expr(self, value)
            },
            Expr::Unary { operator, right } => {
                visitor.visit_unary_expr(self, operator, right)
            },
            Expr::Variable { name } => {
                visitor.visit_variable_expr(self, name)
            },
            Expr::Assign { name, value } => {
                visitor.visit_assign_expr(self, name, value)
            },
            Expr::Logical { left, operator, right } => {
                visitor.visit_logical_expr(self, left, operator, right)
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
        // In fact, it is 
        // Expr::Variable { name: name }
        Expr::Literal { value }
    }

    pub fn grouping(expression: Expr) -> Self {
        Expr::Grouping {
            expression: Box::new(expression),
        }
    }

    pub fn variable(name: Token) -> Self {
        Expr::Variable { name }
    }

    pub fn assign(name: Token, value: Expr) -> Self {
        Expr::Assign { name, value: Box::new(value) }
    }

    pub fn logical(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Logical {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}