use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression { // Ex. 1+2;
        expression: Box<Expr>,
    },
    Print {
        expression: Box<Expr>,
    },
    Var {
        name: Token,
        initializer: Option<Box<Expr>>,
    },
    Block {
        statements: Vec<Stmt>
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>
    },
    While {
        condition: Box<Expr>,
        body: Box<Stmt>
    }
}
/*
// This creates an expression statement
// 1+2;
let stmt = Stmt::Expression {
    expression: Box::new(
        Expr::binary(
            Expr::literal(Some(LiteralValue::Number(1.0))),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Expr::literal(Some(LiteralValue::Number(2.0))),
        )
    ),
};

// This creates a Print Statement
// print "hello";
let stmt = Stmt::Print {
    expression: Box::new(
        Expr::literal(Some(LiteralValue::String("hello".to_string())))
    ),
};

// This creates a Var
// var x = 42;
let stmt = Stmt::Var {
    name: Token::new(TokenType::Identifier, "x".to_string(), None, 1),
    initializer: Some(Box::new(
        Expr::literal(Some(LiteralValue::Number(42.0)))
    )),
};
// var y;
let stmt = Stmt::Var {
    name: Token::new(TokenType::Identifier, "y".to_string(), None, 1),
    initializer: None,  // This means the variable gets nil as default value
};

*/

// Visitor pattern for statements
pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &Stmt, expression: &Expr) -> T;
    fn visit_print_stmt(&mut self, stmt: &Stmt, expression: &Expr) -> T;
    fn visit_var_stmt(&mut self, stmt: &Stmt, name: &Token, initializer: &Option<Box<Expr>>) -> T;
    fn visit_block_stmt(&mut self, stmt: &Stmt, statements: Vec<Stmt>) -> T;
    fn visit_if_stmt(&mut self, stmt: &Stmt, condition: &Expr, then_branch: &Stmt, else_branch: &Option<Box<Stmt>>) -> T;
    fn visit_while_stmt(&mut self, stmt: &Stmt, condition: &Expr, body: &Stmt) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        // TODO: Match on self and call appropriate visitor method
        match self {
            Stmt::Expression { expression } => {
                visitor.visit_expression_stmt(self, expression)
            },
            Stmt::Print { expression } => {
                visitor.visit_print_stmt(self, expression)
            },
            Stmt::Var { name, initializer } => {
                visitor.visit_var_stmt(self, name, initializer)
            }
            Stmt::Block { statements } => {
                visitor.visit_block_stmt(self, statements.clone())
            }
            Stmt::If { condition, then_branch, else_branch } => {
                visitor.visit_if_stmt(self, condition, then_branch, else_branch)
            }
            Stmt::While { condition, body } => {
                visitor.visit_while_stmt(self, condition, body)
            }
        }
    }

    // Helper constructors
    pub fn expression(expr: Expr) -> Self {
        // TODO: Create Expression variant
        Stmt::Expression { expression: Box::new(expr) }
    }

    pub fn print(expr: Expr) -> Self {
        // TODO: Create Print variant
        Stmt::Print { expression: Box::new(expr) }
    }

    pub fn var(name: Token, initializer: Option<Expr>) -> Self {
        // TODO: Create Var variant
        Stmt::Var { name: name, initializer: initializer.map(Box::new) }
    }

    pub fn block(statements: Vec<Stmt>) -> Self {
        Stmt::Block { statements }
    }

    pub fn if_stmt(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Self {
        Stmt::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        }
    }

    pub fn while_stmt(condition: Expr, body: Stmt) -> Self {
        Stmt::While {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }
}