/* Date Created: 16/07/2025. */

//! Rust-specific. Helper function to create instances of 
//! [`super::lox_error::LoxError`].

use super::lox_error::LoxError;
use super::lox_runtime_error::LoxRuntimeError;
use super::token::Token;

/// Report an error at a line and a character. Used by the scanner.
pub fn scanner_error(line: usize, c: char, message: &str) -> LoxError {
    LoxError::new(line, &c.to_string(), message)
}

/// Global error report function. All code using Token should 
/// call this to create return error for `Result<T, LoxError>`.
pub fn error(token: &Token, message: &str) -> LoxError {
    LoxError::new(token.line(), token.lexeme(), message)
}

/// Global error report function. All code which do not use Token 
/// should call this to create return error for `Result<T, LoxError>`.
pub fn sys_error(lexeme: &str, message: &str) -> LoxError {
    LoxError::new(0, lexeme, message)
}

pub fn runtime_error(token: &Token, msg: &str) -> LoxRuntimeError {
    LoxRuntimeError::Error(error(token, msg))
}
