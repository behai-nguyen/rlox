/* Date Created: 02/07/2025. */

use super::lox_error::LoxError;
use super::token_type::TokenType;
use super::token::{LiteralValue, LiteralValue::*, Token};

use super::expr::*;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens
        }
    }

    fn peek(&self, current: usize) -> &Token {
        &self.tokens[current]
    }
    
    fn is_at_end(&self, current: usize) -> bool {
        self.peek(current).get_type() == TokenType::Eof
    }

    fn previous(&self, current: usize) -> &Token {
        &self.tokens[current - 1]
    }

    fn advance(&self, current: &mut usize) -> &Token {
        if !self.is_at_end(*current) { *current += 1; };

        self.previous(*current)
    }

    fn check(&self, current: usize, type_: &TokenType) -> bool {
        if self.is_at_end(current) { return false };

        self.peek(current).get_type() == *type_
    }

    fn match_token(&self, current: &mut usize, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(*current, t) {
                self.advance(current);
                return true;
            }
        }

        false
    }

    fn error(&self, token: &Token, message: &str) -> LoxError {
        LoxError::new(token.get_line(), &format!("{}", message))
    }

    fn consume(&self, 
        current: &mut usize, 
        type_: &TokenType, 
        message: &str) -> Result<&Token, LoxError> {
        if self.check(*current, type_) {
            Ok(self.advance(current))
        } else {
            Err(self.error(self.peek(*current), message))
        }
    }

    // Rust-specific. Not in the book.
    fn literal_expr(&self, value: LiteralValue) -> Expr {
        Expr::Literal(Literal::new(value))
    }    

    fn primary(&self, current: &mut usize) -> Result<Expr, LoxError> {
        if self.match_token(current, &[TokenType::False]) {
            return Ok(self.literal_expr(Boolean(false)))
        }
        if self.match_token(current, &[TokenType::True]) {
            return Ok(self.literal_expr(Boolean(true)))
        }
        if self.match_token(current, &[TokenType::Nil]) {
            return Ok(self.literal_expr(Nil))
        }

        if self.match_token(current, &[TokenType::String]) {
            if let Some(LiteralValue::String(s)) = self.previous(*current).get_literal() {
                return Ok(self.literal_expr(String(s.clone())));
            }
            return Err(self.error(self.previous(*current), "Expected a string value"));
        }

        if self.match_token(current, &[TokenType::Number]) {
            if let Some(LiteralValue::Number(n)) = self.previous(*current).get_literal() {
                return Ok(self.literal_expr(Number(*n)));
            }
            return Err(self.error(self.previous(*current), "Expected a number value"));
        }
        
        if self.match_token(current, &[TokenType::LeftParen]) {
            let expr = self.expression(current)?;
            self.consume(current, &TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Grouping::new(expr)));
        }

        Err(self.error(self.peek(*current), "Expect expression."))
    }

    fn unary(&self, current: &mut usize) -> Result<Expr, LoxError> {
        if self.match_token(current, &[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous(*current).clone();            
            let right = self.unary(current)?;
            Ok(Expr::Unary(Unary::new(operator, right)))
        } else {
            self.primary(current)
        }
    }    

    fn factor(&self, current: &mut usize) -> Result<Expr, LoxError> {
        let mut expr= self.unary(current)?;

        while self.match_token(current, &[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous(*current).clone();
            let right = self.unary(current)?;
            expr = Expr::Binary(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn term(&self, current: &mut usize) -> Result<Expr, LoxError> {
        let mut expr = self.factor(current)?;

        while self.match_token(current, &[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous(*current).clone();
            let right = self.factor(current)?;
            expr = Expr::Binary(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn comparison(&self, current: &mut usize) -> Result<Expr, LoxError> {
        let mut expr = self.term(current)?;

        while self.match_token(current, &[
                TokenType::Greater, TokenType::GreaterEqual, 
                TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous(*current).clone();
            let right = self.term(current)?;
            expr = Expr::Binary(Binary::new(expr, operator, right));
        }

        Ok(expr)        
    }

    fn equality(&self, current: &mut usize) -> Result<Expr, LoxError> {
        let mut expr = self.comparison(current)?;

        while self.match_token(current, &[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous(*current).clone();
            let right = self.comparison(current)?;
            expr = Expr::Binary(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn expression(&self, current: &mut usize) -> Result<Expr, LoxError> {
        self.equality(current)
    }

    fn synchronize(&self, current: &mut usize) {
        self.advance(current);

        while !self.is_at_end(*current) {
            if self.previous(*current).get_type() == TokenType::Semicolon {
                return;
            }

            match self.peek(*current).get_type() {
                TokenType::Class | TokenType::Fun | TokenType::Var |
                TokenType::For | TokenType::If | TokenType::While |
                TokenType::Print | TokenType::Return => { return; }
                _ => (),
            }

            self.advance(current);
        }
    }

    pub fn parse(&self) -> Result<Expr, LoxError> {
        let mut current: usize = 0;

        self.expression(&mut current)
    }
}