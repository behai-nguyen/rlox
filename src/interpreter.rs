/* Date Created: 10/07/2025. */

//! The **Evaluating Expressions** section in  
//! [https://craftinginterpreters.com/evaluating-expressions.html](https://craftinginterpreters.com/evaluating-expressions.html). 
//! 

use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;

use crate::token_type::TokenType;
use super::lox_error::LoxError;
use super::lox_error_helper::error; 
use super::token::{LiteralValue, Token};
use super::data_type::Value;
use super::{
    expr,
    expr::{Expr, Assign, Binary, Call, Get, Grouping,
        Literal, Logical, Set, Super, This, Unary, Variable,}
};
use super::{
    stmt,
    stmt::{Stmt, Block, Class, Expression, Function, If, Print, 
        Return, Var, While,}
};
use super::environment::{Environment, EnvironmentRef};

pub struct Interpreter<W: Write> {
    output: W,
    // The variable global scope.
    environment: EnvironmentRef,
}

impl<W: Write> Interpreter<W> {
    pub fn new(output: W) -> Self {
        Interpreter { 
            output,
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn get_output(&self) -> &W {
        &self.output
    }

    fn write_output(&mut self, value: &str) {
        writeln!(self.output, "{}", value).expect("Failed to write output");
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, LoxError> {
        expr.accept_ref(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        stmt.accept_ref(self)
    }

    // See: https://craftinginterpreters.com/statements-and-state.html#scope
    pub fn execute_block(&mut self, statements: &[Stmt], 
        new_env: EnvironmentRef) -> Result<(), LoxError> {
        let previous = std::mem::replace(&mut self.environment, new_env);

        let result = (|| {
            for stmt in statements {
                self.execute(stmt)?;
            }
            Ok(())
        })();

        self.environment = previous;
        result
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
            _ => Err(error(operator, "Operand must be a number.")),
        }
    }

    fn expect_number(&self, operator: &Token, value: &Value) -> Result<f64, LoxError> {
        match value {
            Value::Number(n) => Ok(*n),
            _ => Err(error(operator, "Operand must be a number.")),
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

    // Chapter 07 version: needed for the tests.
    #[allow(dead_code)]
    pub fn interpret_single_expression(&mut self, expression: &Expr) -> Result<String, LoxError> {
        let value: Value = self.evaluate(expression)?;

        Ok(self.stringify(&value))
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(&statement)?;
        }

        Ok(())
    }
}

impl<W: Write> expr::Visitor<Value> for Interpreter<W> {
    fn visit_assign_expr(&mut self, expr: &Assign) -> Result<Value, LoxError> {
        let value: Value = self.evaluate(expr.get_value())?;
        self.environment.borrow_mut().assign(expr.get_name(), value.clone())?;

        Ok(value)
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<Value, LoxError> {
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
                    _ => Err(error(operator, "Operands must be two numbers or two strings.")),
                }
            }
            TokenType::Slash => self.binary_number_op(operator, &left, &right, |a, b| a / b),
            TokenType::Star => self.binary_number_op(operator, &left, &right, |a, b| a * b),
            // Unreachable.
            _ => { Ok(Value::Nil) }
        }
    }

    fn visit_call_expr(&mut self, _: &Call) -> Result<Value, LoxError> {
        unimplemented!()
    }

    fn visit_get_expr(&mut self, _: &Get) -> Result<Value, LoxError> {
        unimplemented!()
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<Value, LoxError> {
        self.evaluate(expr.get_expression())
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<Value, LoxError> {
        match expr.get_value() {
            LiteralValue::Number(n) => Ok(Value::Number(*n)),
            LiteralValue::String(s) => Ok(Value::String(s.clone())),
            LiteralValue::Boolean(b) => Ok(Value::Boolean(*b)),
            LiteralValue::Nil => Ok(Value::Nil),
        }
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> Result<Value, LoxError> {
        let left = self.evaluate(&expr.get_left())?;

        match expr.get_operator().get_type() {
            TokenType::Or if self.is_truthy(&left) => Ok(left),
            TokenType::And if !self.is_truthy(&left) => Ok(left),
            _ => self.evaluate(&expr.get_right()),
        }
    }

    fn visit_set_expr(&mut self, _: &Set) -> Result<Value, LoxError> {
        unimplemented!()
    }

    fn visit_super_expr(&mut self, _: &Super) -> Result<Value, LoxError> {
        unimplemented!()
    }

    fn visit_this_expr(&mut self, _: &This) -> Result<Value, LoxError> {
        unimplemented!()
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<Value, LoxError> {
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

    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<Value, LoxError> {
        self.environment.borrow().get(expr.get_name())
    }
}

impl<W: Write> stmt::Visitor<()> for Interpreter<W> {
    fn visit_block_stmt(&mut self, stmt: &Block) -> Result<(), LoxError> {
        
        let new_env = Rc::new(RefCell::new(
            Environment::new_local_scope(Rc::clone(&self.environment))
        ));
        self.execute_block(stmt.get_statements(), new_env)?;

        Ok(())
    }

    fn visit_class_stmt(&mut self, _: &Class) -> Result<(), LoxError> {
        unimplemented!()
    }

    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Result<(), LoxError> {
        self.evaluate(stmt.get_expression())?;
        Ok(())
    }

    fn visit_function_stmt(&mut self, _: &Function) -> Result<(), LoxError> {
        unimplemented!()
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> Result<(), LoxError> {
        let value: Value = self.evaluate(stmt.get_condition())?;
        if self.is_truthy(&value) {
            Ok(self.execute(stmt.get_then_branch())?)
        } else if let Some(else_branch) = stmt.get_else_branch() {
            Ok(self.execute(else_branch)?)
        } else {
            Ok(())
        }
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> Result<(), LoxError> {
        let value: Value = self.evaluate(stmt.get_expression())?;

        // Note from the author in the original Java version:
        //     Before discarding the expression’s value, we convert it to a 
        //     string using the stringify() method we introduced in the last 
        //     chapter and then dump it to stdout.
        self.write_output(&self.stringify(&value));
        Ok(())
    }

    fn visit_return_stmt(&mut self, _: &Return) -> Result<(), LoxError> {
        unimplemented!()
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> Result<(), LoxError> {
        let mut value: Value = Value::Nil;

        if let Some(expr) = stmt.get_initializer() {
            value = self.evaluate(expr)?;
        }        

        self.environment.borrow_mut().define(stmt.get_name().get_lexeme().to_string(), value);

        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> Result<(), LoxError> {
        let mut value: Value = self.evaluate(stmt.get_condition())?;
        while self.is_truthy(&value) {
            self.execute(stmt.get_body())?;
            value = self.evaluate(stmt.get_condition())?;
        }
        Ok(())
    }
}
