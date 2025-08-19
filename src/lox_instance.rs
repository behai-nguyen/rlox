/* Date Created: 12/08/2025. */

//! The **Creating Instances** section in  
//! [https://craftinginterpreters.com/classes.html](https://craftinginterpreters.com/classes.html). 
//! 
//! The runtime representation of an instance of a Lox class.
//! 

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use super::lox_class::LoxClass;
use super::token::Token;
use super::value::{Value, ValueMap};
use super::lox_error::LoxError;
use super::lox_error_helper::error;

#[derive(Debug, Clone, PartialEq)]
pub struct LoxInstance {
    klass: LoxClass,
    fields: ValueMap,
}

impl LoxInstance {
    pub fn new(klass: LoxClass) -> Self {
        LoxInstance { 
            klass,
            fields: HashMap::new(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} instance", self.klass)
    }

    // lox_instance comes from the `Interpreter::visit_get_expr()` method.
    pub fn get(lox_instance: Rc<RefCell<LoxInstance>>,
        name: &Token,
    ) -> Result<Value, LoxError> {
        if let Some(v) = lox_instance.borrow().fields.get(name.lexeme()) {
            return Ok(v.clone());
        }

        if let Some(method) = lox_instance.borrow().klass.find_method(name.lexeme()) {
            // Bind to the *same* instance
            let bound = method.bind(Rc::clone(&lox_instance));
            return Ok(Value::LoxCallable(Rc::new(bound)));
        }

        Err(error(name, &format!("Undefined property '{}'.", name.lexeme())))
    }

    pub fn set(&mut self, name: &Token, value: Value) {
        self.fields.insert(name.lexeme().to_string(), value);
    }
}

impl std::fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
