/* Date Created: 16/07/2025. */

//! Rust-specific. Helper function to create instances of 
//! [`super::lox_error::LoxError`].

use super::lox_error::LoxError;
use super::token::Token;

/// Report an error at a line and a character. Used by the scanner.
pub fn scanner_error(line: usize, c: char, message: &str) -> LoxError {
    LoxError::new(line, &c.to_string(), &format!("{}", message))
}

/// Global error report function. All code should call this 
/// to create return error for `Result<T, LoxError>`.
pub fn error(token: &Token, message: &str) -> LoxError {
    LoxError::new(token.line(), token.lexeme(), &format!("{}", message))
}
