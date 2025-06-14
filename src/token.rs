/* Date Created: 29/05/2025. */

//! The **class Token** in  
//! [https://craftinginterpreters.com/scanning.html](https://craftinginterpreters.com/scanning.html).

use std::fmt;

use super::token_type::TokenType;

pub struct Token {
    type_: TokenType,
    lexeme: String,
    /// I don't Rust implementation needs this ``literal`` field. 
    /// Possibly will be removed in the future.
    literal: Option<String>,
    line: usize,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Token { type_, lexeme, literal, line }
    }

    #[allow(dead_code)]
    pub fn get_type(&self) -> TokenType {
        self.type_.clone()
    }

    #[allow(dead_code)]
    pub fn get_lexeme(&self) -> &str {
        &self.lexeme
    }

    #[allow(dead_code)]
    pub fn get_line(&self) -> usize {
        self.line
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type_: {}, lexeme: {}, literal: {}, line: {}", 
            self.type_, 
            self.lexeme, 
            if let Some(val) = &self.literal { val } else { "None" },
            self.line
        )
    }
}