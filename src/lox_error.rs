/* Date Created: 30/05/2025.
   References: 
      https://doc.rust-lang.org/std/error/trait.Error.html
      https://doc.rust-lang.org/std/fmt/trait.Display.html
      https://doc.rust-lang.org/std/fmt/trait.Debug.html
*/

//! Error reporting, includes the line number and the actual error message.

// To run test for this module only: 
// 
//     * cargo test lox_error::tests
//
//     * cargo test lox_error::tests::test_valid_error_message -- --exact [--nocapture]
//     * cargo test lox_error::tests::test_valid_no_line -- --exact [--nocapture]
//     * cargo test lox_error::tests::test_valid_no_line_no_lexeme -- --exact [--nocapture]
//

use std::fmt;

pub struct LoxError {
    line: usize,
    lexeme: String,
    err_msg: String
}

impl LoxError {
    pub fn new(line: usize, lexeme: &str, msg: &str) -> LoxError {
        LoxError{line, lexeme: lexeme.to_string(), err_msg: msg.to_string()}
    }

    #[allow(dead_code)]
    pub fn line(&self) -> usize {
        self.line
    }

    #[allow(dead_code)]
    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    #[allow(dead_code)]
    pub fn err_msg(&self) -> &str {
        &self.err_msg
    }
}

impl fmt::Debug for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LoxError")
            .field("line", &self.line) 
            .field("lexeme", &self.lexeme)
            .field("err_msg", &self.err_msg)
            .finish()
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.line > 0 {
            write!(f,"[line {}] Error at '{}': {}", self.line, self.lexeme, self.err_msg)
        } else {
            if self.lexeme.len() > 0 {
                write!(f, "Error at '{}': {}", self.lexeme, self.err_msg)
            } else {
                write!(f, "{}", self.err_msg)
            }
        }
    }
}

// This allows `LoxError` to be used in standard error propagation chains 
// (e.g., `Result<T, Box<dyn std::error::Error>>`).
impl std::error::Error for LoxError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_error_message() {
        let err = LoxError::new(10, "idx", "this is a test error");
        assert_eq!("[line 10] Error at 'idx': this is a test error", err.to_string());
    }

    #[test]
    fn test_valid_no_line() {
        let err = LoxError::new(0, "super", "this is a test error");
        assert_eq!("Error at 'super': this is a test error", err.to_string());
    }

    #[test]
    fn test_valid_no_line_no_lexeme() {
        let err = LoxError::new(0, "", "this is a test error");
        assert_eq!("this is a test error", err.to_string());
    }
}
