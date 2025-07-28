use crate::stmt::Stmt;
use crate::token::Token;
use crate::environment::Environment;

#[derive(Debug, Clone)]
pub struct LoxFunction {
    declaration: FunctionDeclaration,
    closure: Environment,
}

#[derive(Debug, Clone)]  
pub struct FunctionDeclaration {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

impl LoxFunction {
    pub fn new(declaration: FunctionDeclaration, closure: Environment) -> Self {
        Self { declaration, closure }
    }
    
    pub fn arity(&self) -> usize {
        self.declaration.params.len()
    }
    
    pub fn name(&self) -> &str {
        &self.declaration.name.lexeme
    }
    
    pub fn declaration(&self) -> &FunctionDeclaration {
        &self.declaration
    }
    
    pub fn closure(&self) -> &Environment {
        &self.closure
    }
}

impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        self.declaration.name.lexeme == other.declaration.name.lexeme
    }
}