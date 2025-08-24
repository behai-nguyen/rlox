/* Date Created: 28/07/2025. */

//! Chapter 10: Functions, the 
//! [Return Statements](https://craftinginterpreters.com/functions.html#return-statements)
//! section.
//! 

// Rust-specific.

use std::fmt;

use super::lox_error::LoxError;
use super::lox_return::LoxReturn;

// `Return.java` creates a custom unchecked exception to signal early exits 
// from Lox functions. In Rust, we use **control flow via `Result` and early 
// returns**.
// 
// It is used in ./src/lox_function.rs where interpreter.execute_block() 
// returns Result<(), LoxRuntimeError>:
//
//    return match interpreter.execute_block(&self.declaration.get_body(), environment) {
//        Err(LoxRuntimeError::Return(ret)) => {
//            Ok(ret.value)
//        }
//        Err(LoxRuntimeError::Error(err)) => Err(LoxRuntimeError::Error(err)),
//        Ok(_) => {
//            Ok(Value::Nil)
//        }
//    };
//
#[derive(Debug)]
pub enum LoxRuntimeError {
    Error(LoxError),
    Return(LoxReturn),
}

impl From<LoxError> for LoxRuntimeError {
    fn from(error: LoxError) -> Self {
        LoxRuntimeError::Error(error)
    }
}

impl From<LoxRuntimeError> for LoxError {
    fn from(error: LoxRuntimeError) -> Self {
        let inner = match error {
            LoxRuntimeError::Error(e) => e,
            _ => unreachable!("Expected RuntimeError::Error"),
        };
        inner
    }
}

impl fmt::Display for LoxRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxRuntimeError::Error(err) => write!(f, "{}", err),
            LoxRuntimeError::Return(ret) => write!(f, "Return: {}", ret),
        }
    }
}
