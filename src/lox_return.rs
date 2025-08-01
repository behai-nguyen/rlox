/* Date Created: 28/07/2025. */

//! Chapter 10: Functions, the 
//! [Return Statements](https://craftinginterpreters.com/functions.html#return-statements)
//! section.
//! 
//! In the book, it is the ``Return.java`` module. `Return.java` creates a custom 
//! unchecked exception to signal early exits from Lox functions. In Rust, we use 
//! **control flow via `Result` and early returns**. 
//! 

use std::fmt;

use super::data_type::Value;

#[derive(Debug)]
pub struct LoxReturn {
    pub value: Value,
}

impl LoxReturn {
    pub fn new(value: Value) -> Self {
        LoxReturn { value }
    }
}

impl fmt::Display for LoxReturn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
