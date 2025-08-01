/* Date Created: 27/07/2025. */

//! Chapter 10: Functions, the 
//! [Function Objects](https://craftinginterpreters.com/functions.html#function-objects)
//! section.
//! 

use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;

use crate::stmt;
use super::interpreter::Interpreter;
use super::data_type::Value;
use super::environment::{Environment, EnvironmentRef};
use super::lox_callable::LoxCallable;

use super::lox_runtime_error::LoxRuntimeError;

#[derive(Debug, Clone, PartialEq)]
pub struct LoxFunction {
    declaration: stmt::Function,
    closure: EnvironmentRef,
}

impl LoxFunction {
    pub fn new(declaration: stmt::Function, closure: EnvironmentRef) -> Self {
        Self { declaration, closure }
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.declaration.get_params().len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, LoxRuntimeError> {
        let environment = Rc::new(RefCell::new(
            Environment::new_local_scope(Rc::clone(&self.closure))
        ));

        for (i, param) in self.declaration.get_params().iter().enumerate() {
            let arg = arguments.get(i).unwrap_or(&Value::Nil).clone();
            environment.borrow_mut().define(param.get_lexeme().to_string(), arg);
        }

        return match interpreter.execute_block(&self.declaration.get_body(), environment) {
            Err(LoxRuntimeError::Return(ret)) => {
                Ok(ret.value)
            }
            Err(LoxRuntimeError::Error(err)) => Err(LoxRuntimeError::Error(err)),
            Ok(_) => {
                Ok(Value::Nil)
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.get_name().get_lexeme())
    }
}