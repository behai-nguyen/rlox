/* Date Created: 27/07/2025. */

// Rust-specific. Not in a separate module in the original Java version.

//! Chapter 10: Functions, the 
//! [Native Functions](https://craftinginterpreters.com/functions.html#native-functions)
//! section.

use super::data_type::Value;
use super::lox_runtime_error::LoxRuntimeError;
use super::lox_callable::LoxCallable;
use super::interpreter::Interpreter;

#[derive(Debug, Clone, PartialEq)]
pub struct LoxClock;

impl LoxCallable for LoxClock {
    fn arity(&self) -> usize {
        0 // Takes no arguments
    }

    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Value>) -> Result<Value, LoxRuntimeError> {
        use std::time::{SystemTime, UNIX_EPOCH};

        let since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let seconds = since_epoch.as_secs() as f64 + since_epoch.subsec_micros() as f64 / 1_000_000.0;

        Ok(Value::Number(seconds))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_string(&self) -> String {
        "<native fn>".to_string()
    }
}