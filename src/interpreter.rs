/*
Interpreter.rs: Evaluation/Execution

Input: AST nodes
Output: Runtime values and side effects (printing, variable storage, etc.)
Walks the tree using visitor pattern and executes
*/

use crate::expr::{Expr, ExprVisitor};
use crate::stmt::{Stmt, StmtVisitor};
use crate::environment::Environment;
use crate::token::{Token, TokenType, LiteralValue};
use crate::value::Value;
use anyhow::{anyhow, Result};
use crate::function::{LoxFunction, FunctionDeclaration};
use crate::native::NativeFunction;

pub struct Interpreter {
    globals: Environment,
    environment: Environment,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

#[derive(Debug)]
pub struct ReturnValue {
    pub value: Value,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Runtime Error: {}", self.token.line, self.message)
    }
}

impl std::fmt::Display for ReturnValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Return: {}", self.value)
    }
}

impl std::error::Error for RuntimeError {}
// We implement error typeclass to ReturnValue because we want "?" to immediately exit the execution.
// Since "return" should stop executing the remaining statements in the function.
impl std::error::Error for ReturnValue {}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new();
        
        // Define native functions
        globals.define("clock".to_string(), Value::NativeFunction(NativeFunction::Clock));
        
        Self {
            globals: globals.clone(),
            environment: globals,
        }
    }
    
    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<()> {
        // TODO: Execute each statement
        // Handle runtime errors gracefully
        for statement in statements {
            statement.accept(self)?;
        }
        Ok(())
    }

    pub fn execute_block(&mut self, statements: Vec<Stmt>) -> Result<()> {
        // Create new environment with current one as parent
        let current_env = std::mem::replace(&mut self.environment, Environment::new()); // take a value out of dest -> put src into dest -> return the old value that was in dest
        let block_env = Environment::new_with_enclosing(current_env);
        self.environment = block_env;
        
        // Use a closure to ensure cleanup happens even if there's an error
        // Executes each statement in the block using the new environment
        // If any statement fails, the ? operator returns the error immediately
        let result = (|| {
            for statement in statements {
                statement.accept(self)?;
            }
            Ok(())
        })();

        // Restore the parent environment
        let block_env = std::mem::replace(&mut self.environment, Environment::new());
        if let Some(parent) = block_env.into_enclosing() {
            self.environment = parent;
        }
        
        result
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

    pub fn call_lox_function(&mut self, function: &LoxFunction, arguments: Vec<Value>) -> Result<Value> {
        // TODO: Execute function call
        // 1. Create new environment with function's closure as parent
        // 2. Bind parameters to arguments in new environment
        // 3. Execute function body in new environment  
        // 4. Handle return values (catch ReturnValue errors)
        // 5. Restore previous environment
        // 6. Return function result (or nil if no return)

        let current_env = self.environment.clone();
        let mut call_env = Environment::new_with_enclosing(function.closure().clone());
        
        // When a function is declared, it captures the current environment as its "closure". 
        // But at the time of declaration, the function hasn't been added to the environment yet, 
        // so the function's closure doesn't contain itself.
        // This line manually adds the function to its own call environment, 
        // so when it looks up its own name for recursion, it can find itself.
        call_env.define(function.name().to_string(), Value::Function(function.clone()));
        
        // replaces the interpreter's current environment with the function's call environment
        self.environment = call_env;

        /*
        fun add(a, b) {  // params = ["a", "b"]
            return a + b;
        }

        add(5, 10);      // arguments = [5, 10]
         */
        for (param, arg) in function.declaration().params.iter().zip(arguments.iter()) {
            self.environment.define(param.lexeme.clone(), arg.clone());
        }
        
        // Use Closure because "?" has "return" behind the scene
        // If we dont use closure and one statement cant be executed, "call_function" will return
        // error value immediately. The result is that the old enviroment restoration done later in this 
        // "call_function" will not be done because "?" has already returned something on a behalf of
        // this function.
        // Closure will make "?" returning its error to the variable "result" instead. The "call_function"
        // can still run until the end.
        let result: anyhow::Result<Value> = (|| {
            for statement in &function.declaration().body {
                statement.accept(self)?;
            }
            Ok(Value::Nil)
        })();

        // restores the original environment, 
        // so the interpreter continues executing in the correct context after the function returns.
        let _call_env = std::mem::replace(&mut self.environment, current_env);

        // If the error is ReturnValue error, it is in fact not the error. It works properly and returns the value.
        // Otherwise, it is the actual error.
        match result {
            Err(err) => {
                if let Some(return_val) = err.downcast_ref::<ReturnValue>() {
                    Ok(return_val.value.clone())
                } else {
                    Err(err)
                }
            }
            Ok(_) => Ok(Value::Nil),
        }
    }
}

impl StmtVisitor<Result<()>> for Interpreter {
    fn visit_expression_stmt(&mut self, _stmt: &Stmt, expression: &Expr) -> Result<()> {
        // TODO: Evaluate expression and discard result
        expression.accept(self)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, _stmt: &Stmt, expression: &Expr) -> Result<()> {
        // TODO: Evaluate expression and print result
        let value = expression.accept(self)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var_stmt(&mut self, _stmt: &Stmt, name: &Token, initializer: &Option<Box<Expr>>) -> Result<()> {
        // TODO: 
        // If initializer exists, evaluate it, otherwise use nil
        // Define variable in environment
        let value = if let Some(init) = initializer {
            init.accept(self)?
        } else {
            Value::Nil
        };

        self.environment.define(name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, _stmt: &Stmt, statements: Vec<Stmt>) -> Result<()> {
        self.execute_block(statements)
    }

    fn visit_if_stmt(&mut self, _stmt: &Stmt, 
                                condition: &Expr, 
                                then_branch: &Stmt, 
                                else_branch: &Option<Box<Stmt>>) -> Result<()> {
        // TODO: 
        // 1. Evaluate the condition
        // 2. Check if it's truthy using Value::is_truthy()
        // 3. Execute then_branch if true, else_branch if false and it exists
        let condition = condition.accept(self)?; 
        // This "self" implements both ExprVisitor and StmtVisitor, so it can automatically
        // coerce itself to the right trait obj type to "condition"
        if condition.is_truthy() {
            then_branch.accept(self)?;
        }
        else if let Some(else_stmt) = else_branch {
            else_stmt.accept(self)?;
        }
        
        Ok(())
    }

    fn visit_while_stmt(&mut self, _stmt: &Stmt, condition: &Expr, body: &Stmt) -> Result<()> {
        // TODO:
        // 1. Loop while condition is truthy
        // 2. Execute body in each iteration
        // Be careful with Rust's ownership - you might need to use references
        while condition.accept(self)?.is_truthy() {
            body.accept(self)?;
        }
        Ok(())
    }

    fn visit_function_stmt(&mut self, _stmt: &Stmt, name: &Token, params: &[Token], body: &[Stmt]) -> Result<()> {
        // TODO: Create function object and store in environment
        // 1. Create FunctionDeclaration
        // 2. Capture current environment as closure
        // 3. Create LoxFunction
        // 4. Store in environment with function name
        let declaration = FunctionDeclaration {
            name: name.clone(),
            params: params.to_vec(),
            body: body.to_vec(),
        };
        
        // Define the function in the environment first with a placeholder
        // This makes the name available for recursive calls
        self.environment.define(name.lexeme.clone(), Value::Nil);
        
        // Now create the function with the environment that includes the function name
        let function = LoxFunction::new(declaration, self.environment.clone());
        
        // Replace the placeholder with the actual function
        self.environment.define(name.lexeme.clone(), Value::Function(function));
        
        Ok(())
    }
    
    fn visit_return_stmt(&mut self, _stmt: &Stmt, _keyword: &Token, value: &Option<Box<Expr>>) -> Result<()> {
        // TODO: Evaluate return value and "throw" it as a special error
        // 1. Evaluate value (or use nil if None)
        // 2. Create ReturnValue error
        // 3. Return the error (this will unwind the stack)
        let val = if let Some(v) = value {
            v.accept(self)?
        } else {
            Value::Nil
        };
        
        // Not an actual error. We only need to bypass the remaining statements
        // "?" after "accept(self)" immediately exits the loop
        Err(ReturnValue { value: val }.into())
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

    fn visit_variable_expr(&mut self, _expr: &Expr, name: &Token) -> Result<Value> {
        // TODO: Look up variable in environment
        // Convert environment errors to runtime errors
        self.environment.get(&name.lexeme)
            .map_err(|_| self.runtime_error(name, &format!("Undefined variable '{}'.", name.lexeme)))
    }

    fn visit_assign_expr(&mut self, _expr: &Expr, name: &Token, value: &Expr) -> Result<Value> {
        let val = value.accept(self)?;
        self.environment.assign(&name.lexeme, val.clone())
            .map_err(|_| self.runtime_error(name, &format!("Undefined variable '{}'.", name.lexeme)))?;
        Ok(val)
    }

    fn visit_logical_expr(&mut self, _expr: &Expr, left: &Expr, operator: &Token, right: &Expr) -> Result<Value> {
        // TODO: Implement short-circuiting logic
        // For "or": if left is truthy, return left, otherwise return right
        // For "and": if left is falsy, return left, otherwise return right
        
        let left_value = left.accept(self)?;
        
        match operator.token_type {
            TokenType::Or => {
                if left_value.is_truthy() {
                    // TODO: Return left_value (short-circuit)
                    Ok(left_value)
                } else {
                    // TODO: Evaluate and return right
                    right.accept(self)
                }
            }
            TokenType::And => {
                if !left_value.is_truthy() {
                    // TODO: Return left_value (short-circuit)  
                    Ok(left_value)
                } else {
                    // TODO: Evaluate and return right
                    right.accept(self)
                }
            }
            _ => Err(anyhow!("Unknown logical operator: {:?}", operator.token_type)),
        }
    }

    fn visit_call_expr(&mut self, _expr: &Expr, callee: &Expr, paren: &Token, arguments: &[Expr]) -> Result<Value> {
        // TODO: This is the big one! Function calls
        // 1. Evaluate callee (should be a function)
        // 2. Evaluate all arguments
        // 3. Check arity (argument count)
        // 4. Call the function

        // Simple case: add is callee_value
        // More complex case: add(1,2) is callee_value
        // Error case: f(1,2) when f is defined by var f = 'a';
        let callee_value = callee.accept(self)?;
        let mut args = Vec::new();
        for argument in arguments {
            args.push(argument.accept(self)?);
        }

        match callee_value {
            Value::Function(function) => {
                if arguments.len() != function.arity() {
                    return Err(self.runtime_error(paren, 
                        &format!("Expected {} arguments but got {}.", function.arity(), arguments.len())));
                }
                self.call_lox_function(&function, args)
            }
            Value::NativeFunction(function) => {
                if arguments.len() != function.arity() {
                    return Err(self.runtime_error(paren, 
                        &format!("Expected {} arguments but got {}.", function.arity(), arguments.len())));
                }
                function.call(self, args)
            }
            _ => Err(self.runtime_error(paren, "Can only call functions and classes."))
        }
    }
}