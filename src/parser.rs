/* Date Created: 02/07/2025. */

//! The **Parsing Expressions** in  
//! [https://craftinginterpreters.com/parsing-expressions.html](https://craftinginterpreters.com/parsing-expressions.html)
//! 

// To run test for this module only: 
// 
//     * cargo test parser::tests

use std::rc::Rc;

use crate::stmt;

use super::lox_error::LoxError;
use super::token_type::TokenType;
use super::token::{LiteralValue, LiteralValue::*, Token};
use super::lox_error_helper::error; 
use super::expr::*;
use super::stmt::*;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn is_at_end(&self) -> bool {
        self.peek().token_type() == TokenType::Eof
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.current += 1; };

        self.previous()
    }

    fn check(&self, type_: &TokenType) -> bool {
        if self.is_at_end() { return false };

        self.peek().token_type() == *type_
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, 
        type_: &TokenType, 
        message: &str) -> Result<&Token, LoxError> {
        if self.check(type_) {
            Ok(self.advance())
        } else {
            Err(error(self.peek(), message))
        }
    }

    // Rust-specific. Not in the book.
    fn literal_expr(&self, value: LiteralValue) -> Expr {
        Expr::Literal(Literal::new(value))
    }    

    fn primary(&mut self) -> Result<Rc<Expr>, LoxError> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Rc::new(self.literal_expr(Boolean(false))))
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(Rc::new(self.literal_expr(Boolean(true))))
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Rc::new(self.literal_expr(Nil)))
        }

        if self.match_token(&[TokenType::String]) {
            if let Some(LiteralValue::String(s)) = self.previous().literal() {
                return Ok(Rc::new(self.literal_expr(String(s.clone()))));
            }
            return Err(error(self.previous(), "Expected a string value"));
        }

        if self.match_token(&[TokenType::Number]) {
            if let Some(LiteralValue::Number(n)) = self.previous().literal() {
                return Ok(Rc::new(self.literal_expr(Number(*n))));
            }
            return Err(error(self.previous(), "Expected a number value"));
        }

        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Rc::new(Expr::Variable(Variable::new(self.previous().clone()))))
        }
        
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Rc::new(Expr::Grouping(Grouping::new(expr))));
        }

        Err(error(self.peek(), "Expect expression."))
    }

    fn unary(&mut self) -> Result<Rc<Expr>, LoxError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();            
            let right = self.unary()?;
            Ok(Rc::new(Expr::Unary(Unary::new(operator, right))))
        } else {
            self.call()
        }
    }

    fn finish_call(&mut self, callee: Rc<Expr>) -> Result<Rc<Expr>, LoxError> {
        let mut arguments: Vec<Rc<Expr>> = vec![];

        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(error(self.peek(), "Can't have more than 255 arguments."));
                }
                arguments.push(self.expression()?);

                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren: Token = self.consume(&TokenType::RightParen, "Expect ')' after arguments.")?.clone();

        Ok(Rc::new(Expr::Call(Call::new(callee, paren, arguments))))
    }

    fn call(&mut self) -> Result<Rc<Expr>, LoxError> {
        let mut expr: Rc<Expr> = self.primary()?;

        while self.match_token(&[TokenType::LeftParen]) {
            expr = self.finish_call(expr)?;
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Rc<Expr>, LoxError> {
        let mut expr= self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Rc::new(Expr::Binary(Binary::new(expr, operator, right)));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Rc<Expr>, LoxError> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Rc::new(Expr::Binary(Binary::new(expr, operator, right)));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Rc<Expr>, LoxError> {
        let mut expr = self.term()?;

        while self.match_token(&[
                TokenType::Greater, TokenType::GreaterEqual, 
                TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Rc::new(Expr::Binary(Binary::new(expr, operator, right)));
        }

        Ok(expr)        
    }

    fn equality(&mut self) -> Result<Rc<Expr>, LoxError> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Rc::new(Expr::Binary(Binary::new(expr, operator, right)));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Rc<Expr>, LoxError> {
        let mut expr = self.equality()?;

        while self.match_token(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Rc::new(Expr::Logical(Logical::new(expr, operator, right)));
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Rc<Expr>, LoxError> {
        let mut expr = self.and()?;

        while self.match_token(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Rc::new(Expr::Logical(Logical::new(expr, operator, right)));
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Rc<Expr>, LoxError> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Rc<Stmt>, LoxError> {
        let result = if self.match_token(&[TokenType::Fun]) {
            self.function("function")
        } else if self.match_token(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        match result {
            Ok(stmt) => Ok(stmt),
            Err(err) => {
                self.synchronize();
                Err(err)
            }
        }
    }    
    
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type() == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type() {
                TokenType::Class | TokenType::Fun | TokenType::Var |
                TokenType::For | TokenType::If | TokenType::While |
                TokenType::Print | TokenType::Return => { return; }
                _ => (),
            }

            self.advance();
        }
    }

    fn if_statement(&mut self) -> Result<Rc<Stmt>, LoxError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition: Rc<Expr> = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch: Rc<Stmt> = self.statement()?;

        let else_branch: Option<Rc<Stmt>> = if self.match_token(&[TokenType::Else]) {
            Some(self.statement()?)
        } else {
            None
        };

        Ok(Rc::new(stmt::Stmt::If(stmt::If::new(condition, then_branch, else_branch))))
    }

    fn print_statement(&mut self) -> Result<Rc<Stmt>, LoxError> {
        let value: Rc<Expr> = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Rc::new(stmt::Stmt::Print(stmt::Print::new(value))))
    }

    fn return_statement(&mut self) -> Result<Rc<Stmt>, LoxError> {
        let keyword: Token = self.previous().clone();

        let value = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?) 
        } else { None };

        self.consume(&TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Rc::new(stmt::Stmt::Return(stmt::Return::new(keyword, value))))
    }

    fn var_declaration(&mut self) -> Result<Rc<Stmt>, LoxError> {
        // Clone the token immediately to break the borrow
        let name = self.consume(&TokenType::Identifier, "Expect variable name.")?.clone();

        let mut initializer = None;
        if self.match_token(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(&TokenType::Semicolon, "Expect ';' after variable declaration.")?;

        Ok(Rc::new(stmt::Stmt::Var(stmt::Var::new(name, initializer))))
    }

    fn while_statement(&mut self) -> Result<Rc<Stmt>, LoxError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition: Rc<Expr> = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect ')' after condition.")?;
        let body: Rc<Stmt> = self.statement()?;

        Ok(Rc::new(stmt::Stmt::While(stmt::While::new(condition, body))))
    }

    fn for_statement(&mut self) -> Result<Rc<Stmt>, LoxError> {
        // The opening parenthesis before the clauses.
        self.consume(&TokenType::LeftParen, "Expect '(' after 'for'.")?;

        // The initialiser clause.
        let initializer: Option<Rc<Stmt>>;
        if self.match_token(&[TokenType::Semicolon]) {
            initializer = None;
        } else if self.match_token(&[TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        };

        // Loop condition clause.
        let condition: Option<Rc<Expr>> = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?) } else { None };
        self.consume(&TokenType::Semicolon, "Expect ';' after loop condition.")?;

        // Increment clause.    
        let increment: Option<Rc<Expr>> = if !self.check(&TokenType::RightParen) {
            Some(self.expression()?) } else { None };
        self.consume(&TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(inc) = increment {
            let new_body: Vec<Rc<Stmt>> = vec![
                body,
                Rc::new(stmt::Stmt::Expression(stmt::Expression::new(inc)))
            ];
            body = Rc::new(stmt::Stmt::Block(stmt::Block::new(new_body)));
        };

        body = Rc::new(stmt::Stmt::While(stmt::While::new(
            condition.unwrap_or(
                Rc::new(Expr::Literal(Literal::new(LiteralValue::Boolean(true))))
            ), 
            body)
        ));

        if let Some(ini) = initializer {
            body = Rc::new(stmt::Stmt::Block(stmt::Block::new(vec![ini, body])));
        };

        Ok(body)
    }

    fn expression_statement(&mut self) -> Result<Rc<Stmt>, LoxError> {
        let expr: Rc<Expr> = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after expression.")?;

        Ok(Rc::new(stmt::Stmt::Expression(stmt::Expression::new(expr))))
    }

    fn function(&mut self, kind: &str) -> Result<Rc<Stmt>, LoxError> {
        let name: Token = self.consume(&TokenType::Identifier, &format!("Expect {} name.", kind))?.clone();

        // Parse the parameter list and the pair of parentheses wrapped around it.
        self.consume(&TokenType::LeftParen, &format!("Expect '(' after {} name.", kind))?;
        let mut parameters: Vec<Token> = vec![];
        if !self.check(&TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err(error(self.peek(), "Can't have more than 255 parameters."));
                }
                
                parameters.push(
                    self.consume(&TokenType::Identifier, "Expect parameter name.")?.clone()
                );

                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(&TokenType::RightParen, "Expect ')' after parameters.")?;

        // Parse the body and wrap it all up in a function node.
        self.consume(&TokenType::LeftBrace, &format!("Expect '{{' before {} body.", kind))?;
        let body: Vec<Rc<Stmt>> = self.block()?;
        Ok(Rc::new(Stmt::Function(Function::new(name, parameters, body))))
    }

    // The author's note:
    //     Having block() return the raw list of statements and leaving it to 
    //     statement() to wrap the list in a Stmt.Block looks a little odd. I did 
    //     it that way because we’ll reuse block() later for parsing function bodies 
    //     and we don’t want that body wrapped in a Stmt.Block.
    // See: https://craftinginterpreters.com/statements-and-state.html#scope
    //
    // declaration() handles both statements and declarations (like var), which is 
    // exactly what we want inside blocks.
    fn block(&mut self) -> Result<Vec<Rc<Stmt>>, LoxError> {
        let mut statements: Vec<Rc<Stmt>> = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(&TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn assignment(&mut self) -> Result<Rc<Expr>, LoxError> {
        let expr: Rc<Expr> = self.or()?;

        if self.match_token(&[TokenType::Equal]) {
            let equals: Token = self.previous().clone();
            let value: Rc<Expr> = self.assignment()?;

            match expr.as_ref() {
                Expr::Variable(var) => return Ok(Rc::new(Expr::Assign(Assign::new(var.name().clone(), value)))),
                _ => return Err(error(&equals, "Invalid assignment target."))
            }
        }

        Ok(expr)
    }

    fn statement(&mut self) -> Result<Rc<Stmt>, LoxError> {
        if self.match_token(&[TokenType::For]) {
            self.for_statement()
        } else if self.match_token(&[TokenType::If]) {
            self.if_statement()
        } else if self.match_token(&[TokenType::Print]) {
            self.print_statement()
        } else if self.match_token(&[TokenType::Return]) {
            self.return_statement()
        } else if self.match_token(&[TokenType::While]) {
            self.while_statement()
        } else if self.match_token(&[TokenType::LeftBrace]) {
            Ok(Rc::new(stmt::Stmt::Block(stmt::Block::new(self.block()?))))
        } else {
            self.expression_statement()
        }
    }

    // Chapter 06 version: needed for the tests.
    #[allow(dead_code)]
    pub fn parse_single_expression(&mut self) -> Result<Rc<Expr>, LoxError> {
        self.expression()
    }

    pub fn parse(&mut self) -> Result<Vec<Rc<Stmt>>, LoxError> {
        let mut statements: Vec<Rc<Stmt>> = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_type::TokenType;
    use crate::token::Token;

    #[test]
    fn test_rc_identity_for_variable_expr() {
        let token = Token::new(TokenType::Identifier, "x".to_string(), None, 1);
        let expr1 = Rc::new(Expr::Variable(Variable::new(token.clone())));
        let expr2 = Rc::clone(&expr1);

        assert!(Rc::ptr_eq(&expr1, &expr2));
    }
}