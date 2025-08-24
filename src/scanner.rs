/* Date Created: 29/05/2025. */

//! The **The Scanner Class** in  
//! [https://craftinginterpreters.com/scanning.html](https://craftinginterpreters.com/scanning.html). 
//! 
//! This Rust implementation supports UTF-8.
//! 
//! The class variables ``start``, ``current`` and ``line`` are represented by 
//! [ScannerIndex](`crate::scanner_index::ScannerIndex`), and passed through methods
//! as parameter ``indexes``.
//! 
//! Method names follow Rust convention. I.e.: ``scan_tokens``, ``peek_next``, etc.
//! 
//! Where an identifier is not a keyword in the Java language, but a keyword in Rust, 
//! it is suffixed with an underscore **-**. E.g. ``match`` in Java is ``match_char`` in
//! Rust.

use std::collections::HashMap;

use super::lox_error::LoxError;
use super::lox_error_helper::{scanner_error, sys_error}; 
use super::scanner_index::ScannerIndex;
use super::token::{LiteralValue, Token};
use super::token_type::TokenType;

type KeywordsMap = HashMap<&'static str, TokenType>;

pub struct Scanner<'a> {
    source: &'a str,
    indexes: ScannerIndex,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source: source.trim(),
            indexes: ScannerIndex::new(),
        }
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.source.chars().nth(self.indexes.get_current()) {
            self.indexes.inc_lexeme_indexes(c.len_utf8());
            Some(c)
        } else {
            None
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.indexes.get_current() >= self.source.chars().count()
    }    

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        match self.source.chars().nth(self.indexes.get_current()) {
            Some(c) => {
                if c != expected {
                    return false;
                }
            }
            // Not in original https://craftinginterpreters.com/scanning.html#the-scanner-class
            None => return false
        }

        self.indexes.inc_lexeme_indexes(expected.len_utf8());
        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        if let Some(c) = self.source.chars().nth(self.indexes.get_current()) {
            c
        } else {
            // Not in original https://craftinginterpreters.com/scanning.html#the-scanner-class
            '\0'
        }
    }

    fn peek_next(&mut self) -> char {
        if (self.indexes.get_current() + 1) >= self.source.len() {
            return '\0';
        }

        if let Some(c) = self.source.chars().nth(self.indexes.get_current() + 1) {
            c
        } else {
            // Not in https://craftinginterpreters.com/scanning.html#the-scanner-class
            '\0'
        }
    }

    fn string(&mut self, lst: &mut Vec<Token>) -> Result<(), LoxError> {

        while (self.peek() != '"') && !self.is_at_end() {
            if self.peek() == '\n' {
                self.indexes.inc_line();
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(scanner_error(self.indexes.get_line(), self.peek(), "Unterminated string."));
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self.source[self.indexes.get_start() + 1..self.indexes.get_byte_count() - 1].to_string();

        self.add_token_with_literal(lst, TokenType::String, 
            Some(LiteralValue::String(value)));

        Ok(())
    }

    fn is_digit(c: char) -> bool {
        (c >= '0') & (c <= '9')
    }

    fn number(&mut self, lst: &mut Vec<Token>) -> Result<(), LoxError> {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part.        
        if (self.peek() == '.') && Self::is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        let mut str = self.source[self.indexes.get_start()..self.indexes.get_byte_count()].to_string();
        if !str.contains('.') {
            str.push_str(".0");
        } else if str.ends_with('.') {
            str.push('0');
        }

        let value = str
            .parse::<f64>()
            .map_err(|e| scanner_error(self.indexes.get_line(), self.peek(), &format!("Failed to parse float: {}", e)))?;

        self.add_token_with_literal(lst, TokenType::Number, 
            Some(LiteralValue::Number(value)));

        Ok(())
    }    

    fn is_alpha(c: char) -> bool {
        ((c >= 'a') & (c <= 'z')) ||
        ((c >= 'A') & (c <= 'Z')) ||
        c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn identifier(&mut self, keywords: &KeywordsMap, lst: &mut Vec<Token>) {        
        while Self::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let keyword = self.source[self.indexes.get_start()..self.indexes.get_byte_count()].to_string();

        if let Some(token) = keywords.get(keyword.as_str()) {
            self.add_token(lst, token.clone());
        } else {
            self.add_token(lst, TokenType::Identifier);
        }
    }

    fn add_token(&self, 
        lst: &mut Vec<Token>, 
        type_: TokenType) {
        self.add_token_with_literal(lst, type_, None);
    }

    fn add_token_with_literal(&self, 
        lst: &mut Vec<Token>,
        type_: TokenType, 
        literal: Option<LiteralValue>) {
            let lex = self.source[self.indexes.get_start()..self.indexes.get_byte_count()].to_string();
            lst.push(Token::new(type_, lex, literal, self.indexes.get_line()));
    }

    fn scan_token(&mut self, 
        keywords: &KeywordsMap, 
        lst: &mut Vec<Token>) -> Result<(), LoxError> {
        let c: char;

        if let Some(val) = self.advance() {
            c = val;
        } else {
            // Not in https://craftinginterpreters.com/scanning.html#the-scanner-class
            c = '\0'
        }

        match c {
            '(' => self.add_token(lst, TokenType::LeftParen),
            ')' => self.add_token(lst, TokenType::RightParen),
            '{' => self.add_token(lst, TokenType::LeftBrace),
            '}' => self.add_token(lst, TokenType::RightBrace),
            ',' => self.add_token(lst, TokenType::Comma),
            '.' => self.add_token(lst, TokenType::Dot),
            '-' => self.add_token(lst, TokenType::Minus),
            '+' => self.add_token(lst, TokenType::Plus),
            ';' => self.add_token(lst, TokenType::Semicolon),
            '*' => self.add_token(lst, TokenType::Star),

            '!' => {
                let type_ = if self.match_char('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.add_token(lst, type_);
            }

            '=' => {
                let type_ = if self.match_char('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_token(lst, type_);
            }

            '<' => {
                let type_ = if self.match_char('=') { TokenType::LessEqual } else { TokenType::Less };
                self.add_token(lst, type_);
            }

            '>' => {
                let type_ = if self.match_char('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_token(lst, type_);
            }

            '/' => {
                if self.match_char('/') {
                    while (self.peek() != '\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(lst, TokenType::Slash);
                }
            }

            ' ' | '\r' | '\t' => {},

            '\n' => self.indexes.inc_line(),

            '"' => self.string(lst)?,

            _ => {
                if Self::is_digit(c) {
                    self.number(lst)?;
                } else if Self::is_alpha(c) {
                    self.identifier(keywords, lst);
                } else {
                    return Err(scanner_error(self.indexes.get_line(), c,  &format!("Unexpected character: {}.", c)));
                }
            }
        }

        Ok(())
    }

    // Function to create the keyword map
    pub fn create_keywords_map() -> KeywordsMap {
        let mut keywords = HashMap::new();
        
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);
        
        keywords
    }

    // Scans the full source-text, captures all errors.
    // When there are multiple errors, they are separated by a 
    // newline ( \n ) character.    
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxError> {
        if self.is_at_end() {
            return Err(sys_error("", "Source text is empty."));
        }

        let keywords: KeywordsMap = Self::create_keywords_map();

        let mut tokens = Vec::<Token>::new();

        let mut err_msgs: Vec<String> = vec![];

        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            let _ = &self.indexes.set_start(self.indexes.get_byte_count());
 
            match self.scan_token(&keywords, &mut tokens) {
                Ok(_) => {},
                Err(err) => err_msgs.push(format!("{}", err)),                
            }
        }

        if err_msgs.len() == 0 {
            tokens.push(Token::new(TokenType::Eof, "".to_string(), None, self.indexes.get_line()));
            Ok(tokens)
        } else {
            Err(sys_error("", &err_msgs.join("\n")))
        }

    }
}