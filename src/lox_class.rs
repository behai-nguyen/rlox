/* Date Created: 12/08/2025. */

//! The **Class Declarations** section in  
//! [https://craftinginterpreters.com/classes.html](https://craftinginterpreters.com/classes.html). 
//! 
//! The runtime representation of a class.
//! 

use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use super::lox_runtime_error::LoxRuntimeError;
use super::interpreter::Interpreter;
use super::value::Value;
use super::lox_callable::LoxCallable;
use super::lox_instance::LoxInstance;
use super::lox_function::LoxFunction;

pub type LoxFunctionsMap = HashMap<String, LoxFunction>;

#[derive(Debug, Clone, PartialEq)]
pub struct LoxClass {
    name: String,
    superclass: Option<Rc<LoxClass>>,
    methods: LoxFunctionsMap,
}

impl LoxClass {
    pub fn new(name: String, 
        superclass: Option<Rc<LoxClass>>, 
        methods: LoxFunctionsMap) -> Self {
        LoxClass { 
            name, 
            superclass,
            methods,
        }
    }

    pub fn find_method(&self, name: &str) -> Option<LoxFunction> {
        if let Some(method) = self.methods.get(name) {
            return Some(method.clone());
        } else if let Some(sc) = &self.superclass {
            return sc.find_method(name);
        }
        None
    }
}

impl std::fmt::Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        if let Some(initializer) = self.find_method("init") {
            initializer.arity()
        } else {
            0
        }
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, LoxRuntimeError> {
        let instance = LoxInstance::new(self.clone());
        let instance_ref = Rc::new(RefCell::new(instance));

        if let Some(initializer) = self.find_method("init") {
            initializer.bind(
                Rc::clone(&instance_ref))
                .call(interpreter, arguments)?;
        }
        Ok(Value::LoxInstance(instance_ref))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}