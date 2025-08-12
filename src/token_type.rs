/* Date Created: 29/05/2025. */

//! The **Token type** in  
//! [https://craftinginterpreters.com/scanning.html](https://craftinginterpreters.com/scanning.html).

use std::fmt;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, 
    RightParen, 
    LeftBrace, 
    RightBrace,
    Comma, 
    Dot, 
    Minus, 
    Plus, 
    Semicolon, 
    Slash, 
    Star,

    // One or two character tokens.
    Bang, 
    BangEqual,
    Equal, 
    EqualEqual,
    Greater, 
    GreaterEqual,
    Less, 
    LessEqual,

    // Literals.
    Identifier, 
    String, 
    Number,

    // Keywords.
    And, 
    Class, 
    Else, 
    False, 
    Fun, 
    For, 
    If, 
    Nil, 
    Or,
    Print, 
    Return, 
    Super, 
    This, 
    True, 
    Var, 
    While,
    Eof
}

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{:?}", self)
  }
}