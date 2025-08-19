/* Date Created: 27/07/2025. */

//! Chapter 10: Functions, the 
//! [Function Objects](https://craftinginterpreters.com/functions.html#function-objects)
//! section.
//! 

use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;

use crate::stmt;
use super::lox_runtime_error::LoxRuntimeError;
use super::interpreter::Interpreter;
use super::value::Value;
use super::environment::{Environment, EnvironmentRef};
use super::lox_callable::LoxCallable;
use super::lox_instance::LoxInstance;

pub struct LoxFunction {
    declaration: stmt::Function,
    closure: EnvironmentRef,
    is_initializer: bool,
}

impl LoxFunction {
    pub fn new(declaration: stmt::Function, closure: EnvironmentRef, is_initializer: bool) -> Self {
        Self { declaration, closure, is_initializer }
    }

    pub fn bind(&self, instance: Rc<RefCell<LoxInstance>>) -> LoxFunction {
        let mut env = Environment::new_local_scope(self.closure.clone());
        env.define("this".to_string(), Value::LoxInstance(instance));
        LoxFunction::new(self.declaration.clone(), 
            Rc::new(RefCell::new(env)), self.is_initializer)
    }
}

impl std::fmt::Debug for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LoxFunction {{ name: {} }}", self.declaration.name().lexeme())
    }
}

// We need to take charge of implementing `PartialEq` and `Clone` for 
// `LoxFunction`, due to:
//
// `closure: EnvironmentRef` is an `Rc<RefCell<Environment>>`, and `Environment` 
// contains a `HashMap<String, Value>`, which may include functions, which 
// contain environments...
//
// So `PartialEq` on `LoxFunction` can recurse infinitely or deeply through cyclic 
// structures: this would eventually result in stack overflow.
impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.closure, &other.closure)
            && self.declaration == other.declaration
    }
}

impl Clone for LoxFunction {
    fn clone(&self) -> Self {
        LoxFunction {
            declaration: self.declaration.clone(),
            closure: Rc::clone(&self.closure), // Shallow clone.
            is_initializer: self.is_initializer,
        }
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.declaration.params().len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, LoxRuntimeError> {
        let environment = Rc::new(RefCell::new(
            Environment::new_local_scope(Rc::clone(&self.closure))
        ));

        for (i, param) in self.declaration.params().iter().enumerate() {
            let arg = arguments.get(i).unwrap_or(&Value::Nil).clone();
            environment.borrow_mut().define(param.lexeme().to_string(), arg);
        }

        return match interpreter.execute_block(&self.declaration.body(), environment) {
            Err(LoxRuntimeError::Return(ret)) => {
                if self.is_initializer {
                    Ok(Environment::get_at(&self.closure, 0, "this"))
                } else {
                    Ok(ret.value)
                }
            }
            Err(LoxRuntimeError::Error(err)) => Err(LoxRuntimeError::Error(err)),
            Ok(_) => {
                if self.is_initializer {
                    Ok(Environment::get_at(&self.closure, 0, "this"))
                } else {
                    Ok(Value::Nil)
                }
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl std::fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.declaration.name().lexeme())
    }
}
