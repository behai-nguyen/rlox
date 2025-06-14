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
//! it is suffixed with an underscore **-**. E.g. ``match`` in Java is ``match_`` in
//! Rust.

use std::collections::HashMap;

use super::lox_error::LoxError;
use super::scanner_index::ScannerIndex;
use super::token::Token;
use super::token_type::TokenType;

type KeywordsMap = HashMap<&'static str, TokenType>;

pub struct Scanner<'a> {
    source: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source
        }
    }

    fn advance(&self, indexes: &mut ScannerIndex) -> Option<char> {
        if let Some(c) = self.source.chars().nth(indexes.get_current()) {
            indexes.inc_lexeme_indexes(c.len_utf8());
            return Some(c);
        }

        None
    }

    fn is_at_end(&self, indexes: &mut ScannerIndex) -> bool {
        indexes.get_current() >= self.source.chars().count()
    }    

    fn match_(&self, indexes: &mut ScannerIndex, expected: char) -> bool {
        if self.is_at_end(indexes) {
            return false;
        }

        match self.source.chars().nth(indexes.get_current()) {
            Some(c) => {
                if c != expected {
                    return false;
                }
            }
            // Not in original https://craftinginterpreters.com/scanning.html#the-scanner-class
            None => return false
        }

        indexes.inc_lexeme_indexes(expected.len_utf8());
        true
    }

    fn peek(&self, indexes: &mut ScannerIndex) -> char {
        if self.is_at_end(indexes) {
            return '\0';
        }

        if let Some(c) = self.source.chars().nth(indexes.get_current()) {
            return c;
        }

        // Not in original https://craftinginterpreters.com/scanning.html#the-scanner-class
        '\0'
    }

    fn peek_next(&self, indexes: &mut ScannerIndex) -> char {
        if (indexes.get_current() + 1) >= self.source.len() {
            return '\0';
        }

        if let Some(c) = self.source.chars().nth(indexes.get_current() + 1) {
            return c;
        }

        // Not in https://craftinginterpreters.com/scanning.html#the-scanner-class
        '\0'
    }

    fn string(&self, 
        indexes: &mut ScannerIndex, 
        lst: &mut Vec<Token>) -> Result<(), LoxError> {

        while (self.peek(indexes) != '"') & !self.is_at_end(indexes) {
            if self.peek(indexes) == '\n' {
                indexes.inc_line();
            }

            self.advance(indexes);
        }

        if self.is_at_end(indexes) {
            return Err(LoxError::new(indexes.get_line(), "Unterminated string."));
        }

        // The closing ".
        self.advance(indexes);

        // Trim the surrounding quotes.
        let value = self.source[indexes.get_start() + 1..indexes.get_byte_count() - 1].to_string();

        #[cfg(debug_assertions)]
        {
            println!("string():\n{0}org. value: {1}\n{0}type: {2}\n", 
                      "    ", value.clone(), TokenType::String(value.clone()));
        }

        self.add_token(indexes, lst, TokenType::String(value));

        Ok(())
    }

    fn is_digit(c: char) -> bool {
        (c >= '0') & (c <= '9')
    }

    fn number(&self, indexes: &mut ScannerIndex, lst: &mut Vec<Token>) -> Result<(), LoxError> {
        while Self::is_digit(self.peek(indexes)) {
            self.advance(indexes);
        }

        // Look for a fractional part.        
        if (self.peek(indexes) == '.') & Self::is_digit(self.peek_next(indexes)) {
            // Consume the "."
            self.advance(indexes);

            while Self::is_digit(self.peek(indexes)) {
                self.advance(indexes);
            }
        }

        let str = self.source[indexes.get_start()..indexes.get_byte_count()].to_string();
        let value = str.parse::<f32>().map_err(|e| LoxError::new(indexes.get_line(), &format!("Failed to parse float: {}", e)))?;

        #[cfg(debug_assertions)]
        {
            println!("number():\n{0}org. str: {1}\n{0}type: {2}\n", 
                      "    ", str, TokenType::Number(value));
        }

        self.add_token(indexes, lst, TokenType::Number(value));

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

    fn identifier(&self, indexes: &mut ScannerIndex, keywords: &KeywordsMap, lst: &mut Vec<Token>) {        
        while Self::is_alpha_numeric(self.peek(indexes)) {
            self.advance(indexes);
        }

        #[cfg(debug_assertions)] { println!("identifier():"); }

        let keyword = self.source[indexes.get_start()..indexes.get_byte_count()].to_string();

        #[cfg(debug_assertions)] { println!("    keyword: {keyword}"); }

        if let Some(token) = keywords.get(keyword.as_str()) {
            #[cfg(debug_assertions)] { println!("    type: {token}"); }

            self.add_token(indexes, lst, token.clone());
        } else {
            #[cfg(debug_assertions)] { println!("    type: {}", TokenType::Identifier); }

            self.add_token(indexes, lst, TokenType::Identifier);
        }
    }

    fn add_token(&self, 
        indexes: &mut ScannerIndex,
        lst: &mut Vec<Token>, 
        type_: TokenType) {
        self.add_token_with_literal(indexes, lst, type_, None);
    }

    fn add_token_with_literal(&self, 
        indexes: &mut ScannerIndex,
        lst: &mut Vec<Token>,
        type_: TokenType, 
        literal: Option<String>) {
            let lex = self.source[indexes.get_start()..indexes.get_byte_count()].to_string();
            #[cfg(debug_assertions)] 
            {
                println!("add_token_with_literal():\n{0}type: {1}\n{0}lex: \
                          [{2}]\n{0}literal: {3:?}\n{0}start: {4} - byte count: {5}",
                          "    ", type_, lex, literal, indexes.get_start(), indexes.get_byte_count());
            }

            lst.push(Token::new(type_, lex, literal, indexes.get_line()));
    }

    fn scan_token(&self, 
        indexes: &mut ScannerIndex, 
        keywords: &KeywordsMap, 
        lst: &mut Vec<Token>) -> Result<(), LoxError> {
        let c: char;

        if let Some(val) = self.advance(indexes) {
            c = val;
        } else {
            // Not in https://craftinginterpreters.com/scanning.html#the-scanner-class
            c = '\0'
        }

        match c {
            '(' => self.add_token(indexes, lst, TokenType::LeftParen),
            ')' => self.add_token(indexes, lst, TokenType::RightParen),
            '{' => self.add_token(indexes, lst, TokenType::LeftBrace),
            '}' => self.add_token(indexes, lst, TokenType::RightBrace),
            ',' => self.add_token(indexes, lst, TokenType::Comma),
            '.' => self.add_token(indexes, lst, TokenType::Dot),
            '-' => self.add_token(indexes, lst, TokenType::Minus),
            '+' => self.add_token(indexes, lst, TokenType::Plus),
            ';' => self.add_token(indexes, lst, TokenType::Semicolon),
            '*' => self.add_token(indexes, lst, TokenType::Star),

            '!' => {
                let type_ = if self.match_(indexes, '=') { TokenType::BangEqual } else { TokenType::Bang };
                self.add_token(indexes, lst, type_);
            }

            '=' => {
                let type_ = if self.match_(indexes, '=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_token(indexes, lst, type_);
            }

            '<' => {
                let type_ = if self.match_(indexes, '=') { TokenType::LessEqual } else { TokenType::Less };
                self.add_token(indexes, lst, type_);
            }

            '>' => {
                let type_ = if self.match_(indexes, '=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_token(indexes, lst, type_);
            }

            '/' => {
                if self.match_(indexes, '/') {
                    while (self.peek(indexes) != '\n') & !self.is_at_end(indexes) {
                        self.advance(indexes);
                    }
                } else {
                    self.add_token(indexes, lst, TokenType::Slash);
                }
            }

            ' ' | '\r' | '\t' => {},

            '\n' => indexes.inc_line(),

            '"' => self.string(indexes, lst)?,

            _ => {
                if Self::is_digit(c) {
                    self.number(indexes, lst)?;
                } else if Self::is_alpha(c) {
                    self.identifier(indexes, keywords, lst);
                } else {
                    return Err(LoxError::new(indexes.get_line(), 
                        &format!("Unexpected character: {}.", c)));
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

    pub fn scan_tokens(&self) -> Result<Vec<Token>, LoxError> {
        let mut indexes = ScannerIndex::new();

        let keywords: KeywordsMap = Self::create_keywords_map();

        let mut tokens = Vec::<Token>::new();

        while !self.is_at_end(&mut indexes) {
            // We are at the beginning of the next lexeme.
            let _ = &indexes.set_start(indexes.get_byte_count());
 
            self.scan_token(&mut indexes, &keywords, &mut tokens)?;
        }

        tokens.push(Token::new(TokenType::Eof, "".to_string(), None, indexes.get_line()));
        Ok(tokens)
    }
}