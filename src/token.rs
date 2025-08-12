/* Date Created: 29/05/2025. */

//! The **class Token** in  
//! [https://craftinginterpreters.com/scanning.html](https://craftinginterpreters.com/scanning.html).

use std::fmt;
use std::hash::{Hash, Hasher};

use super::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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

    pub fn token_type(&self) -> TokenType {
        self.type_.clone()
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn literal(&self) -> &Option<LiteralValue> {
        &self.literal
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "type_: {}, lexeme: {}, literal: {}, line: {}", 
            self.type_, 
            self.lexeme, 
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

impl Hash for LiteralValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            LiteralValue::Number(n) => {
                state.write_u8(0); // tag for Number
                state.write_u64(n.to_bits()); // safe hashable representation
            }
            LiteralValue::String(s) => {
                state.write_u8(1);
                s.hash(state);
            }
            LiteralValue::Boolean(b) => {
                state.write_u8(2);
                b.hash(state);
            }
            LiteralValue::Nil => {
                state.write_u8(3);
            }
        }
    }
}

impl Eq for LiteralValue {}