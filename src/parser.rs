use crate::expr::Expr;
use crate::token::{Token, TokenType, LiteralValue};
use crate::error::ErrorReporter;
use anyhow::{Result, anyhow};
use std::result::Result::{Ok, Err};

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

impl std::error::Error for ParseError {}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self, error_reporter: &mut ErrorReporter) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(err) => {
                if let Some(parse_err) = err.downcast_ref::<ParseError>() {
                    error_reporter.report(parse_err.line, "", &parse_err.message);
                } else {
                    error_reporter.report(0, "", &err.to_string());
                }
                None // Return of Option when there is no value.
            }
        }
    }

    // Grammar rules - each becomes a method
    fn expression(&mut self) -> Result<Expr> {
        // TODO: Call equality()
        self.equality()
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