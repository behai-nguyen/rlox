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

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_lexeme(&self) -> String {
        self.lexeme.clone()
    }

    pub fn get_err_msg(&self) -> String {
        self.err_msg.clone()
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
        write!(f,"[line {}] Error at '{}': {}", self.line, self.lexeme, self.err_msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let err = LoxError::new(10, "idx", "this is a test error");
        assert_eq!("[line 10] Error at 'idx': this is a test error", err.to_string());
    }
}
