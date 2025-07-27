use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType, LiteralValue};
use crate::error::ErrorReporter;
use anyhow::{anyhow, Result};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize, // point to the next token waiting to be parsed
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Parse error: {}", self.line, self.message)
    }
}

// Expression: a piece of code that evaluates to a value. It represents a computation that produces a result.
// Expression Statement: an expression followed by a semicolon that turns it into a statement. The value is evaluated but then discarded.

/*
When to use expression()
if (x > 5) { ... }       // Condition needs the boolean value
var y = x + 1;           // Initializer needs the computed value
while (count < 10) { ... } // Condition needs the boolean value

When to use expression_statement()
x = 5;                   // Standalone assignment
calculate();             // Standalone function call  
3 + 4;                   // Standalone calculation (unusual but valid)
*/

impl std::error::Error for ParseError {}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self, error_reporter: &mut ErrorReporter) -> Option<Vec<Stmt>> {
        // TODO: Parse multiple statements instead of single expression
        // Return Vec<Stmt> instead of Expr
        let mut statements = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    if let Some(parse_err) = err.downcast_ref::<ParseError>() {
                        error_reporter.report(parse_err.line, "", &parse_err.message);
                    }
                    else {
                        error_reporter.report(0, "", &err.to_string());
                    }
                    self.synchronize();
                }
            }
        }
        if statements.is_empty() {None}
        else{ Some(statements) }
    }

    fn declaration(&mut self) -> Result<Stmt> {
        if self.match_tokens(&[TokenType::Var]) { // Reminder:match_tokens already moves away from "var"
            self.var_declaration()
        } 
        else if self.match_tokens(&[TokenType::LeftBrace]){
            Ok(Stmt::block(self.block()?))
        }
        else {
            self.statement()
        }
    }

    fn block(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        // TODO: Implement this
        // Hint: consume "(", parse condition, consume ")", parse then branch
        // Check for "else" and parse else branch if present
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?; 
        // Don't forget that a block statement is ONE statement. containing several statements inside
        let else_branch = if self.match_tokens(&[TokenType::Else]) {
            Some(self.statement()?)
        } else {
            None
        };

        Ok(Stmt::if_stmt(condition, then_branch, else_branch))
    }

    fn while_statement(&mut self) -> Result<Stmt> {
        // TODO: Implement this
        // Similar to if, but simpler - just condition and body
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after while condition.")?;
        let body = self.statement()?;

        Ok(Stmt::while_stmt(condition, body))
    }

    fn for_statement(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        // check if the for loop has an initializer: for(var i=0;...)
        let initializer = if self.match_tokens(&[TokenType::Semicolon]){ 
            // for (; i < 10; i = i + 1) { ... }
            // If the token following the ( is a semicolon then the initializer has been omitted.
            None
        }
        else if self.match_tokens(&[TokenType::Var]){
            // for (var i = 0; i < 10; i = i + 1) { ... }
            Some(self.var_declaration()?)
        } 
        else{
            // for (i = 0; i < 10; i = i + 1) { ... } // i is declared elsewhere
            Some(self.expression_statement()?)
        };

        // check the loop condition: for(...;i<10;...)
        let condition = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        // check the loop increment
        let increment = if !self.check(&TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        // Do the transformation from for to while loop
        // Start from creating a loop block -> pack a loop block with the condition 
        // -> pack the while loop with initialization

        // start from creating a "block" containing loop body following by increment
        if let Some(increment_expr) = increment {
            body = Stmt::block(vec![
                body,
                Stmt::expression(increment_expr),
            ]);
        }
        
        // Create the while loop by packing a condition and body together
        let condition_expr = condition.unwrap_or_else(|| {
            Expr::literal(Some(LiteralValue::Boolean(true))) // No condition means "while true {...}"
        });
        body = Stmt::while_stmt(condition_expr, body);

        // If there's an initializer, wrap everything in a block
        if let Some(init) = initializer {
            body = Stmt::block(vec![init, body]);
        }

        Ok(body)
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.match_tokens(&[TokenType::Print]) {
            self.print_statement()
        } 
        else if self.match_tokens(&[TokenType::If]) {
            // TODO: Call self.if_statement()
            self.if_statement()
        } 
        else if self.match_tokens(&[TokenType::While]) {
            // TODO: Call self.while_statement()
            self.while_statement()
        }
        else if self.match_tokens(&[TokenType::LeftBrace]) {
            Ok(Stmt::block(self.block()?))
        }
        else if self.match_tokens(&[TokenType::For]){
            self.for_statement()
        }
        else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        // TODO: 
        // Parse expression after "print"
        // Consume semicolon
        // Return Stmt::print()
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::print(value))
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        // TODO:
        // Expect identifier for variable name
        // If "=" found, parse initializer expression
        // Consume semicolon
        // Return Stmt::var()
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?.clone();
        
        let initializer = if self.match_tokens(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::var(name.clone(), initializer))
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        // TODO:
        // Parse expression
        // Consume semicolon  
        // Return Stmt::expression()
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::expression(expr))
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.or()?; // check the left side first

        if self.match_tokens(&[TokenType::Equal]) { // if the right side is "=", do this
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable { name } = expr {
                return Ok(Expr::assign(name, value));
            }

            return Err(self.error(&equals, "Invalid assignment target."));
        }

        Ok(expr) // If not, do this
    }

    fn or(&mut self) -> Result<Expr> {
        // TODO: Implement logical OR with short-circuiting
        // Pattern: similar to equality() but for "or" operators
        let mut expr = self.and()?;
        while self.match_tokens(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::logical(expr, operator, right);
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr> {
        // TODO: Implement logical AND
        // Call equality() for the operands
        let mut expr = self.equality()?;
        while self.match_tokens(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::logical(expr, operator, right);
        }
        Ok(expr)
    }

    // Grammar rules - each becomes a method
    fn expression(&mut self) -> Result<Expr> {
        // TODO: Call equality()
        self.assignment()
    }

    fn equality(&mut self) -> Result<Expr> {
        // TODO: Implement equality rule
        // Pattern: left-associative binary operators
        // Start with comparison(), then loop while we see != or ==
        let mut expr = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let mut operator = self.previous().clone();
            let mut right_expr = self.comparison()?;
            expr = Expr::binary(expr, operator, right_expr);
        }
        Ok(expr) 
    }

    fn comparison(&mut self) -> Result<Expr> {
        // TODO: Similar to equality, but for >, >=, <, <=
        let mut expr = self.term()?;
        while self.match_tokens(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let mut operator = self.previous().clone();
            let right_expr = self.term()?;
            expr = Expr::binary(expr, operator, right_expr);
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr> {
        // TODO: Handle + and -
        let mut expr = self.factor()?;
        while self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let mut operator = self.previous().clone();
            let right_expr = self.factor()?;
            expr = Expr::binary(expr, operator, right_expr);
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr> {
        // TODO: Handle * and /
        let mut expr = self.unary()?;
        while self.match_tokens(&[TokenType::Star, TokenType::Slash]) {
            let mut operator = self.previous().clone();
            let right_expr = self.unary()?;
            expr = Expr::binary(expr, operator, right_expr);
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr> {
        // TODO: Handle ! and - prefix operators
        // If we see ! or -, consume it and recursively call unary()
        // Otherwise, call primary()
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]){
            let mut operator = self.previous().clone();
            let mut right_expr = self.unary()?;
            return Ok(Expr::unary(operator, right_expr));
        }
        else { return self.primary();}
    }

    fn primary(&mut self) -> Result<Expr> {
        // TODO: Handle literals, identifiers, and grouping
        // This is where you handle the "leaves" of the expression tree

        if self.match_tokens(&[TokenType::Identifier]) {
            return Ok(Expr::variable(self.previous().clone()));
        }

        if self.match_tokens(&[TokenType::False]) {
            return Ok(Expr::literal(Some(LiteralValue::Boolean(false))));
        }

        if self.match_tokens(&[TokenType::True]) {
            return Ok(Expr::literal(Some(LiteralValue::Boolean(true))));
        }

        if self.match_tokens(&[TokenType::Nil]) {
            return Ok(Expr::literal(Some(LiteralValue::Nil)));
        }

        if self.match_tokens(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::literal(self.previous().literal.clone()));
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::grouping(expr));
        }

        Err(self.error(self.peek(), "Expect expression."))
    }

    // Helper methods for token manipulation
    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        // TODO: Check if current token matches any of the given types
        // If so, advance and return true
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        // TODO: Return true if current token is of given type
        // Don't advance
        if self.is_at_end() {return false;}
        else {
            &self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> &Token {
        // TODO: Move to next token and return the previous one
        if !self.is_at_end() { 
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        // TODO: Check if we're at EOF token
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        // TODO: Return current token without advancing
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        // TODO: Return the previous token
        return &self.tokens[self.current-1];
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(self.error(self.peek(), message))  // NEW: Use error() method
        }
    }

    fn error(&self, token: &Token, message: &str) -> anyhow::Error {
        let error_msg = if token.token_type == TokenType::Eof {
            format!("{} at end", message)
        } else {
            format!("{} at '{}'", message, token.lexeme)
        };

        ParseError {
            message: error_msg,
            line: token.line,
        }.into()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}