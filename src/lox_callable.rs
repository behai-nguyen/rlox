/* Date Created: 26/07/2025. */

//! Chapter 10: Functions, the 
//! [Function Calls](https://craftinginterpreters.com/functions.html#function-calls)
//! section, [Interpreting function calls](https://craftinginterpreters.com/functions.html#interpreting-function-calls) 
//! subsection.

use std::fmt;
use std::any::Any;

use super::interpreter::Interpreter;
use super::lox_runtime_error::LoxRuntimeError;
use super::value::Value;

pub trait LoxCallable: fmt::Debug + fmt::Display {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, LoxRuntimeError>;
    #[allow(dead_code)]
    // No downcast_ref::<T>() anywhere in the code yet.
    // Might remove later, but not now.
    fn as_any(&self) -> &dyn Any;
}
