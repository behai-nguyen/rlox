/* Date Created: 29/05/2025. */

//! The **class Token** in  
//! [https://craftinginterpreters.com/scanning.html](https://craftinginterpreters.com/scanning.html).

use std::fmt;

use super::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    type_: TokenType,
    lexeme: String,
    literal: Option<LiteralValue>,
    line: usize,
}

impl Token {
    pub fn new(type_: TokenType, 
        lexeme: String, 
        literal: Option<LiteralValue>, 
        line: usize) -> Self {
            Token { type_, lexeme, literal, line }
    }

    pub fn get_type(&self) -> TokenType {
        self.type_.clone()
    }

    pub fn get_lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn get_literal(&self) -> &Option<LiteralValue> {
        &self.literal
    }

    pub fn get_line(&self) -> usize {
        self.line
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "type_: {}, lexeme: {}, literal: {}, line: {}", 
            self.type_, 
            self.lexeme, 
            /*match &self.literal {
                None => "None".to_string(),
                Some(val) => {
                    match val {
                        LiteralValue::Number(n) => n.to_string(),
                        LiteralValue::String(s) => s.to_string(),
                        LiteralValue::Boolean(b) => b.to_string(),
                        LiteralValue::Nil => "nil".to_string()
                    }
                }
            },*/
            self.literal
                .as_ref()
                .map(|val| match val {
                    LiteralValue::Number(n) => n.to_string(),
                    LiteralValue::String(s) => s.to_string(),
                    LiteralValue::Boolean(b) => b.to_string(),
                    LiteralValue::Nil => "nil".to_string(),
                })
                .unwrap_or_else(|| "None".to_string()),
            self.line
        )
    }
}