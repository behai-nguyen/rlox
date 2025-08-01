/* Date Created: 26/07/2025. */

//! Chapter 10: Functions, the 
//! [Function Calls](https://craftinginterpreters.com/functions.html#function-calls)
//! section, [Interpreting function calls](https://craftinginterpreters.com/functions.html#interpreting-function-calls) 
//! subsection.

use std::fmt::Debug;
use std::any::Any;

use super::interpreter::Interpreter;
use super::lox_runtime_error::LoxRuntimeError;
use super::data_type::Value;

// Box<dyn LoxCallable> require manual implementations because trait objects don’t 
// support Clone or PartialEq by default.
pub trait CloneLoxCallable {
    fn clone_box(&self) -> Box<dyn LoxCallable>;
}

impl<T> CloneLoxCallable for T
where
    T: LoxCallable + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn LoxCallable> {
        Box::new(self.clone())
    }
}

// Box<dyn LoxCallable> require manual implementations because trait objects don’t 
// support Clone or PartialEq by default.
pub trait PartialEqLoxCallable {
    fn equals_callable(&self, other: &dyn LoxCallable) -> bool;
}

impl<T> PartialEqLoxCallable for T
where
    T: LoxCallable + PartialEq + Any,
{
    fn equals_callable(&self, other: &dyn LoxCallable) -> bool {
        other
            .as_any()
            .downcast_ref::<T>()
            .map_or(false, |other_t| self == other_t)
    }
}

pub trait LoxCallable: Debug + CloneLoxCallable + PartialEqLoxCallable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, LoxRuntimeError>;
    fn as_any(&self) -> &dyn Any;
    fn to_string(&self) -> String;
}

impl PartialEq for Box<dyn LoxCallable> {
    fn eq(&self, other: &Self) -> bool {
        (**self).equals_callable(&**other)
    }
}
