/* Date Created: 10/07/2025. */

//! The **Evaluating Expressions** in  
//! [https://craftinginterpreters.com/evaluating-expressions.html](https://craftinginterpreters.com/evaluating-expressions.html). 
//! 

use crate::token_type::TokenType;

use super::lox_error::LoxError;
use super::token::{LiteralValue, Token};

use super::expr::*;

// Rust-specific.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub struct Interpreter;

impl Interpreter {
    fn error(&self, token: &Token, message: &str) -> LoxError {
        LoxError::new(token.get_line(), &format!("{}", message))
    }

    fn evaluate(&self, expr: &Expr) -> Result<Value, LoxError> {
        expr.accept(self)
    }

    // Ruby rule: false and nil are falsey, and everything else is truthy. 
    fn is_truthy(&self, object: &Value) -> bool {
        match object {
            Value::Number(_) => true,
            Value::String(_) => true,
            Value::Boolean(b) => *b,
            Value::Nil => false,
        }
    }

    fn is_equal(&self, a: &Value, b: &Value) -> bool {
        a == b
    }

    fn check_number_operand(&self, operator: &Token, 
        operand: &Value) -> Result<(), LoxError> {
        match operand {
            Value::Number(_) => Ok(()),
            _ => Err(self.error(operator, "Operand must be a number.")),
        }
    }

    // In original Java version. Not being used in Rust.
    /*fn check_number_operands(&self, operator: &Token, 
        left: &Value, right: &Value) -> Result<(), LoxError> {
        match (left, right) {
            (Value::Number(_), Value::Number(_)) => Ok(()),
            _ => Err(self.error(operator, "Operands must be numbers.")),
        }
    }*/

    fn expect_number(&self, operator: &Token, value: &Value) -> Result<f64, LoxError> {
        match value {
            Value::Number(n) => Ok(*n),
            _ => Err(self.error(operator, "Operand must be a number.")),
        }
    }

    fn binary_number_op<F>(&self, 
        operator: &Token, left: &Value, right: &Value, f: F) -> Result<Value, LoxError>
    where
        F: Fn(f64, f64) -> f64,
    {
        let l = self.expect_number(operator, left)?;
        let r = self.expect_number(operator, right)?;
        Ok(Value::Number(f(l, r)))
    }

    // Unlike the author original Java version, this version purposely 
    // keeps '.0' for f64.
    fn stringify(&self, object: &Value) -> String {
        match object {
            Value::Number(n) => format!("{:?}", n),
            Value::String(s) => s.to_string(),
            Value::Boolean(b) => b.to_string(),            
            Value::Nil => "nil".to_string(),
        }
    }

    pub fn interpret(&self, expression: &Expr) -> Result<String, LoxError> {
        let value: Value = self.evaluate(expression)?;

        Ok(self.stringify(&value))
    }
}

impl Visitor<Value> for Interpreter {
    fn visit_assign_expr(&self, _: &Assign) -> Result<Value, LoxError> {
        unimplemented!()
    }

    /*
    // My first cut, seems okay.
    fn visit_binary_expr(&self, expr: &Binary) -> Result<Value, LoxError> {
        let left: Value = self.evaluate(&expr.get_left())?;
        let right: Value = self.evaluate(&expr.get_right())?;

        let operator: &Token = expr.get_operator();

        match operator.get_type() {
            TokenType::Greater => {
                self.check_number_operands(operator, &left, &right)?;
                Ok(Value::Boolean(self.expect_number(operator, &left)? > 
                    self.expect_number(operator, &right)?))
            }
            TokenType::GreaterEqual => {
                self.check_number_operands(operator, &left, &right)?;
                Ok(Value::Boolean(self.expect_number(operator, &left)? >=
                    self.expect_number(operator, &right)?))
            }
            TokenType::Less => {
                self.check_number_operands(operator, &left, &right)?;
                Ok(Value::Boolean(self.expect_number(operator, &left)? <
                    self.expect_number(operator, &right)?))
            }
            TokenType::LessEqual => {
                self.check_number_operands(operator, &left, &right)?;
                Ok(Value::Boolean(self.expect_number(operator, &left)? <=
                    self.expect_number(operator, &right)?))
            }
            TokenType::BangEqual => Ok(Value::Boolean(!self.is_equal(&left, &right))),
            TokenType::EqualEqual => Ok(Value::Boolean(self.is_equal(&left, &right))),
            TokenType::Minus => {
                self.check_number_operands(operator, &left, &right)?;
                Ok(Value::Number(self.expect_number(operator, &left)? -
                    self.expect_number(operator, &right)?))
            }
            TokenType::Plus => {
                match (left, right) {
                    (Value::Number(ln), Value::Number(rn)) => {
                        Ok(Value::Number(ln + rn))
                    }
                    (Value::String(ls), Value::String(rs)) => {
                        Ok(Value::String(format!("{}{}", ls, rs)))
                    }                    
                    _ => Err(self.error(operator, "Operands must be two numbers or two strings.")),
                }
            }
            TokenType::Slash => {
                self.check_number_operands(operator, &left, &right)?;
                Ok(Value::Number(self.expect_number(operator, &left)? /
                    self.expect_number(operator, &right)?))
            }
            TokenType::Star => {
                self.check_number_operands(operator, &left, &right)?;
                Ok(Value::Number(self.expect_number(operator, &left)? *
                    self.expect_number(operator, &right)?))
            },             
            // Unreachable.
            _ => { Ok(Value::Nil) }
        }
    }
    */
    fn visit_binary_expr(&self, expr: &Binary) -> Result<Value, LoxError> {
        let left: Value = self.evaluate(&expr.get_left())?;
        let right: Value = self.evaluate(&expr.get_right())?;

        let operator: &Token = expr.get_operator();

        match operator.get_type() {
            TokenType::Greater => {
                let l = self.expect_number(operator, &left)?;
                let r = self.expect_number(operator, &right)?;
                Ok(Value::Boolean(l > r))
            }
            TokenType::GreaterEqual => {
                let l = self.expect_number(operator, &left)?;
                let r = self.expect_number(operator, &right)?;
                Ok(Value::Boolean(l >= r))                    
            }
            TokenType::Less => {
                let l = self.expect_number(operator, &left)?;
                let r = self.expect_number(operator, &right)?;
                Ok(Value::Boolean(l < r))                    
            }
            TokenType::LessEqual => {
                let l = self.expect_number(operator, &left)?;
                let r = self.expect_number(operator, &right)?;
                Ok(Value::Boolean(l <= r))                    
            }
            TokenType::BangEqual => Ok(Value::Boolean(!self.is_equal(&left, &right))),
            TokenType::EqualEqual => Ok(Value::Boolean(self.is_equal(&left, &right))),
            TokenType::Minus => self.binary_number_op(operator, &left, &right, |a, b| a - b),
            TokenType::Plus => {
                match (left, right) {
                    (Value::Number(ln), Value::Number(rn)) => {
                        Ok(Value::Number(ln + rn))
                    }
                    (Value::String(ls), Value::String(rs)) => {
                        Ok(Value::String(format!("{}{}", ls, rs)))
                    }                    
                    _ => Err(self.error(operator, "Operands must be two numbers or two strings.")),
                }
            }
            TokenType::Slash => self.binary_number_op(operator, &left, &right, |a, b| a / b),
            TokenType::Star => self.binary_number_op(operator, &left, &right, |a, b| a * b),
            // Unreachable.
            _ => { Ok(Value::Nil) }
        }
    }

    fn visit_call_expr(&self, _: &Call) -> Result<Value, LoxError> {
        unimplemented!()
    }

    fn visit_get_expr(&self, _: &Get) -> Result<Value, LoxError> {
        unimplemented!()
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> Result<Value, LoxError> {
        self.evaluate(expr.get_expression())
    }

    fn visit_literal_expr(&self, expr: &Literal) -> Result<Value, LoxError> {
        match expr.get_value() {
            LiteralValue::Number(n) => Ok(Value::Number(*n)),
            LiteralValue::String(s) => Ok(Value::String(s.clone())),
            LiteralValue::Boolean(b) => Ok(Value::Boolean(*b)),
            LiteralValue::Nil => Ok(Value::Nil),
        }
    }

    fn visit_logical_expr(&self, _: &Logical) -> Result<Value, LoxError> {
        unimplemented!()
    }

    fn visit_set_expr(&self, _: &Set) -> Result<Value, LoxError> {
        unimplemented!()
    }

    fn visit_super_expr(&self, _: &Super) -> Result<Value, LoxError> {
        unimplemented!()
    }

    fn visit_this_expr(&self, _: &This) -> Result<Value, LoxError> {
        unimplemented!()
    }

    fn visit_unary_expr(&self, expr: &Unary) -> Result<Value, LoxError> {
        let right: Value = self.evaluate(&expr.get_right())?;

        match expr.get_operator().get_type() {
            TokenType::Bang => Ok(Value::Boolean(!self.is_truthy(&right))),
            TokenType::Minus => {
                self.check_number_operand(expr.get_operator(), &right)?;
                Ok(Value::Number(-self.expect_number(expr.get_operator(), &right)?))
            }
            // Unreachable.
            _ => { Ok(Value::Nil) }
        }
    }

    fn visit_variable_expr(&self, _: &Variable) -> Result<Value, LoxError> {
        unimplemented!()
    }
}