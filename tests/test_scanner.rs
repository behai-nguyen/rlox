// Date Created: 05/06/2025.

//! Uses data from `./data/scanning/`.
//! 
//! To run test for this module only: 
//! 
//!     * cargo test --test test_scanner
//! 
//! To run a specific test method: 
//! 
//!     * cargo test identifiers -- --exact [--nocapture]
//!     * cargo test keywords -- --exact [--nocapture]
//!     * cargo test numbers -- --exact [--nocapture]
//!     * cargo test punctuators -- --exact [--nocapture]
//!     * cargo test strings -- --exact [--nocapture]
//!     * cargo test whitespace -- --exact [--nocapture]
//!     * cargo test sample -- --exact [--nocapture]
//!     * cargo test utf8_text -- --exact [--nocapture]

mod test_common;

use crate::test_common::{
    get_script_contents,
    assert_literal_number,
    assert_literal_string,
    assert_literal_none,
};

use rlox::token_type::TokenType;
use rlox::scanner::Scanner;

#[test]
fn identifiers() {
    let res = get_script_contents("./tests/data/scanning/identifiers.lox");
    // Read script file was successful.
    assert_eq!(res.is_err(), false);

    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Scanning was successful.
    assert_eq!(res.is_err(), false);

    let token_list = res.unwrap();

    assert_eq!(token_list.len(), 9);

    let token = &token_list[0];
    assert_eq!(token.get_type(), TokenType::Identifier);
    assert_eq!(token.get_lexeme(), "andy");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[1];
    assert_eq!(token.get_type(), TokenType::Identifier);
    assert_eq!(token.get_lexeme(), "formless");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[2];
    assert_eq!(token.get_type(), TokenType::Identifier);
    assert_eq!(token.get_lexeme(), "fo");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[3];
    assert_eq!(token.get_type(), TokenType::Identifier);
    assert_eq!(token.get_lexeme(), "_");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[4];
    assert_eq!(token.get_type(), TokenType::Identifier);
    assert_eq!(token.get_lexeme(), "_123");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[5];
    assert_eq!(token.get_type(), TokenType::Identifier);
    assert_eq!(token.get_lexeme(), "_abc");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[6];
    assert_eq!(token.get_type(), TokenType::Identifier);
    assert_eq!(token.get_lexeme(), "ab123");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[7];
    assert_eq!(token.get_type(), TokenType::Identifier);
    assert_eq!(token.get_lexeme(), "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 2);

    let token = &token_list[8];
    assert_eq!(token.get_type(), TokenType::Eof);
    assert_eq!(token.get_lexeme(), "");
    assert_literal_none(token.get_literal());
    // 13 is correct. There are 13 lines.
    assert_eq!(token.get_line(), 13);
}

#[test]
fn keywords() {
    let res = get_script_contents("./tests/data/scanning/keywords.lox");
    // Read script file was successful.
    assert_eq!(res.is_err(), false);

    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Scanning was successful.
    assert_eq!(res.is_err(), false);

    let token_list = res.unwrap();

    assert_eq!(token_list.len(), 16);

    let token = &token_list[0];
    assert_eq!(token.get_type(), TokenType::And);
    assert_eq!(token.get_lexeme(), "and");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);
	
    let token = &token_list[1];
    assert_eq!(token.get_type(), TokenType::Class);
    assert_eq!(token.get_lexeme(), "class");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);
	
    let token = &token_list[2];
    assert_eq!(token.get_type(), TokenType::Else);
    assert_eq!(token.get_lexeme(), "else");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);
	
    let token = &token_list[3];
    assert_eq!(token.get_type(), TokenType::False);
    assert_eq!(token.get_lexeme(), "false");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);
	
    let token = &token_list[4];
    assert_eq!(token.get_type(), TokenType::For);
    assert_eq!(token.get_lexeme(), "for");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);
	
    let token = &token_list[5];
    assert_eq!(token.get_type(), TokenType::Fun);
    assert_eq!(token.get_lexeme(), "fun");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);
	
    let token = &token_list[6];
    assert_eq!(token.get_type(), TokenType::If);
    assert_eq!(token.get_lexeme(), "if");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);
	
    let token = &token_list[7];
    assert_eq!(token.get_type(), TokenType::Nil);
    assert_eq!(token.get_lexeme(), "nil");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);
	
    let token = &token_list[8];
    assert_eq!(token.get_type(), TokenType::Or);
    assert_eq!(token.get_lexeme(), "or");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);
	
    let token = &token_list[9];
    assert_eq!(token.get_type(), TokenType::Return);
    assert_eq!(token.get_lexeme(), "return");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);
	
    let token = &token_list[10];
    assert_eq!(token.get_type(), TokenType::Super);
    assert_eq!(token.get_lexeme(), "super");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);
	
    let token = &token_list[11];
    assert_eq!(token.get_type(), TokenType::This);
    assert_eq!(token.get_lexeme(), "this");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);
	
    let token = &token_list[12];
    assert_eq!(token.get_type(), TokenType::True);
    assert_eq!(token.get_lexeme(), "true");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);
	
    let token = &token_list[13];
    assert_eq!(token.get_type(), TokenType::Var);
    assert_eq!(token.get_lexeme(), "var");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);
	
    let token = &token_list[14];
    assert_eq!(token.get_type(), TokenType::While);
    assert_eq!(token.get_lexeme(), "while");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);
	
    let token = &token_list[15];
    assert_eq!(token.get_type(), TokenType::Eof);
    assert_eq!(token.get_lexeme(), "");
    assert_literal_none(token.get_literal());
    // There are 19 lines in the script file.
    assert_eq!(token.get_line(), 19);
}

#[test]
// To fully make sense of test conditions in this method, one should read this 
// section https://craftinginterpreters.com/scanning.html#number-literals of 
// the book fairly thoroughly: particularly the leading and trailing decimal
// point.
fn numbers() {
    let res = get_script_contents("./tests/data/scanning/numbers.lox");
    // Read script file was successful.
    assert_eq!(res.is_err(), false);

    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Scanning was successful.
    assert_eq!(res.is_err(), false);

    let token_list = res.unwrap();

    assert_eq!(token_list.len(), 7);

    let token = &token_list[0];
    assert_eq!(token.get_type(), TokenType::Number);
    assert_eq!(token.get_lexeme(), "123");
    assert_literal_number(token.get_literal(), 123.0);
    assert_eq!(token.get_line(), 1);

    let token = &token_list[1];
    assert_eq!(token.get_type(), TokenType::Number);
    assert_eq!(token.get_lexeme(), "123.456");
    assert_literal_number(token.get_literal(), 123.456);
    assert_eq!(token.get_line(), 2);
    
    let token = &token_list[2];
    assert_eq!(token.get_type(), TokenType::Dot);
    assert_eq!(token.get_lexeme(), ".");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 3);

    let token = &token_list[3];
    assert_eq!(token.get_type(), TokenType::Number);
    assert_eq!(token.get_lexeme(), "456");
    assert_literal_number(token.get_literal(), 456.0);
    assert_eq!(token.get_line(), 3);

    let token = &token_list[4];
    assert_eq!(token.get_type(), TokenType::Number);
    assert_eq!(token.get_lexeme(), "123");
    assert_literal_number(token.get_literal(), 123.0);
    assert_eq!(token.get_line(), 4);

    let token = &token_list[5];
    assert_eq!(token.get_type(), TokenType::Dot);
    assert_eq!(token.get_lexeme(), ".");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 4);    

    let token = &token_list[6];
    assert_eq!(token.get_type(), TokenType::Eof);
    assert_eq!(token.get_lexeme(), "");
    assert_literal_none(token.get_literal());
    // There are 13 lines in the script file.
    assert_eq!(token.get_line(), 13);
}

#[test]
fn punctuators() {
    let res = get_script_contents("./tests/data/scanning/punctuators.lox");
    // Read script file was successful.
    assert_eq!(res.is_err(), false);

    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Scanning was successful.
    assert_eq!(res.is_err(), false);

    let token_list = res.unwrap();

    assert_eq!(token_list.len(), 19);

    let token = &token_list[0];
    assert_eq!(token.get_type(), TokenType::LeftParen);
    assert_eq!(token.get_lexeme(), "(");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[1];
    assert_eq!(token.get_type(), TokenType::RightParen);
    assert_eq!(token.get_lexeme(), ")");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[2];
    assert_eq!(token.get_type(), TokenType::LeftBrace);
    assert_eq!(token.get_lexeme(), "{");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[3];
    assert_eq!(token.get_type(), TokenType::RightBrace);
    assert_eq!(token.get_lexeme(), "}");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[4];
    assert_eq!(token.get_type(), TokenType::Semicolon);
    assert_eq!(token.get_lexeme(), ";");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[5];
    assert_eq!(token.get_type(), TokenType::Comma);
    assert_eq!(token.get_lexeme(), ",");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[6];
    assert_eq!(token.get_type(), TokenType::Plus);
    assert_eq!(token.get_lexeme(), "+");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[7];
    assert_eq!(token.get_type(), TokenType::Minus);
    assert_eq!(token.get_lexeme(), "-");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[8];
    assert_eq!(token.get_type(), TokenType::Star);
    assert_eq!(token.get_lexeme(), "*");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[9];
    assert_eq!(token.get_type(), TokenType::BangEqual);
    assert_eq!(token.get_lexeme(), "!=");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[10];
    assert_eq!(token.get_type(), TokenType::EqualEqual);
    assert_eq!(token.get_lexeme(), "==");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[11];
    assert_eq!(token.get_type(), TokenType::LessEqual);
    assert_eq!(token.get_lexeme(), "<=");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[12];
    assert_eq!(token.get_type(), TokenType::GreaterEqual);
    assert_eq!(token.get_lexeme(), ">=");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[13];
    assert_eq!(token.get_type(), TokenType::BangEqual);
    assert_eq!(token.get_lexeme(), "!=");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[14];
    assert_eq!(token.get_type(), TokenType::Less);
    assert_eq!(token.get_lexeme(), "<");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[15];
    assert_eq!(token.get_type(), TokenType::Greater);
    assert_eq!(token.get_lexeme(), ">");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[16];
    assert_eq!(token.get_type(), TokenType::Slash);
    assert_eq!(token.get_lexeme(), "/");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[17];
    assert_eq!(token.get_type(), TokenType::Dot);
    assert_eq!(token.get_lexeme(), ".");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[18];
    assert_eq!(token.get_type(), TokenType::Eof);
    assert_eq!(token.get_lexeme(), "");
    assert_literal_none(token.get_literal());
    // The script file has 22 lines.
    assert_eq!(token.get_line(), 22);
}

#[test]
fn strings() {
    let res = get_script_contents("./tests/data/scanning/strings.lox");
    // Read script file was successful.
    assert_eq!(res.is_err(), false);

    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Scanning was successful.
    assert_eq!(res.is_err(), false);

    let token_list = res.unwrap();

    assert_eq!(token_list.len(), 3);

    let token = &token_list[0];
    // If these two following test conditions don't make sense immediately, it's okay...
    assert_eq!(token.get_type(), TokenType::String);
    assert_eq!(token.get_lexeme(), "\"\"");
    assert_literal_string(token.get_literal(), "");
    assert_eq!(token.get_line(), 1);

    let token = &token_list[1];
    // Similar to the above...
    assert_eq!(token.get_type(), TokenType::String);
    assert_eq!(token.get_lexeme(), "\"string\"");
    assert_literal_string(token.get_literal(), "string");
    assert_eq!(token.get_line(), 2);

    let token = &token_list[2];
    assert_eq!(token.get_type(), TokenType::Eof);
    assert_eq!(token.get_lexeme(), "");
    assert_literal_none(token.get_literal());
    // Test script file has 6 lines.
    assert_eq!(token.get_line(), 6);
}

#[test]
fn whitespace() {
    let res = get_script_contents("./tests/data/scanning/whitespace.lox");
    // Read script file was successful.
    assert_eq!(res.is_err(), false);

    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Scanning was successful.
    assert_eq!(res.is_err(), false);

    let token_list = res.unwrap();

    assert_eq!(token_list.len(), 5);

    let token = &token_list[0];
    assert_eq!(token.get_type(), TokenType::Identifier);
    assert_eq!(token.get_lexeme(), "space");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[1];
    assert_eq!(token.get_type(), TokenType::Identifier);
    assert_eq!(token.get_lexeme(), "tabs");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[2];
    assert_eq!(token.get_type(), TokenType::Identifier);
    assert_eq!(token.get_lexeme(), "newlines");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[3];
    assert_eq!(token.get_type(), TokenType::Identifier);
    assert_eq!(token.get_lexeme(), "end");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 6);

    let token = &token_list[4];
    assert_eq!(token.get_type(), TokenType::Eof);
    assert_eq!(token.get_lexeme(), "");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 13);
}

#[test]
fn sample() {
    let res = get_script_contents("./tests/data/scanning/sample.lox");
    // Read script file was successful.
    assert_eq!(res.is_err(), false);

    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Scanning was successful.
    assert_eq!(res.is_err(), false);

    let token_list = res.unwrap();

    assert_eq!(token_list.len(), 80);

    let token = &token_list[0];
    assert_eq!(token.get_type(), TokenType::Class);
    assert_eq!(token.get_lexeme(), "class");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[8];
    assert_eq!(token.get_type(), TokenType::String);
    assert_eq!(token.get_lexeme(), "\"Fry until golden brown.\"");
    assert_literal_string(token.get_literal(), "Fry until golden brown.");
    assert_eq!(token.get_line(), 3);

    let token = &token_list[13];
    assert_eq!(token.get_type(), TokenType::Identifier);
    assert_eq!(token.get_lexeme(), "BostonCream");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 7);

    let token = &token_list[27];
    assert_eq!(token.get_type(), TokenType::Print);
    assert_eq!(token.get_lexeme(), "print");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 10);

    let token = &token_list[55];
    assert_eq!(token.get_type(), TokenType::For);
    assert_eq!(token.get_lexeme(), "for");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 20);

    let token = &token_list[64];
    assert_eq!(token.get_type(), TokenType::Number);
    assert_eq!(token.get_lexeme(), "5");
    assert_literal_number(token.get_literal(), 5.00);
    assert_eq!(token.get_line(), 20);

    let token = &token_list[79];
    assert_eq!(token.get_type(), TokenType::Eof);
    assert_eq!(token.get_lexeme(), "");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 22);
}

#[test]
fn utf8_text() {
    let res = get_script_contents("./tests/data/scanning/utf8_text.lox");
    // Read script file was successful.
    assert_eq!(res.is_err(), false);

    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Scanning was successful.
    assert_eq!(res.is_err(), false);

    let token_list = res.unwrap();

    assert_eq!(token_list.len(), 16);

    let token = &token_list[0];
    assert_eq!(token.get_type(), TokenType::Var);
    assert_eq!(token.get_lexeme(), "var");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[1];
    assert_eq!(token.get_type(), TokenType::Identifier);
    assert_eq!(token.get_lexeme(), "str");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[2];
    assert_eq!(token.get_type(), TokenType::Equal);
    assert_eq!(token.get_lexeme(), "=");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[3];
    assert_eq!(token.get_type(), TokenType::String);
    assert_eq!(token.get_lexeme(), "\"運去英雄飲恨多\"");
    assert_literal_string(token.get_literal(), "運去英雄飲恨多");
    assert_eq!(token.get_line(), 1);

    let token = &token_list[4];
    assert_eq!(token.get_type(), TokenType::Semicolon);
    assert_eq!(token.get_lexeme(), ";");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 1);

    let token = &token_list[5];
    assert_eq!(token.get_type(), TokenType::Var);
    assert_eq!(token.get_lexeme(), "var");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 3);

    let token = &token_list[6];
    assert_eq!(token.get_type(), TokenType::Identifier);
    assert_eq!(token.get_lexeme(), "str1");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 3);

    let token = &token_list[7];
    assert_eq!(token.get_type(), TokenType::Equal);
    assert_eq!(token.get_lexeme(), "=");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 3);

    let token = &token_list[8];
    assert_eq!(token.get_type(), TokenType::String);
    assert_eq!(token.get_lexeme(), "\"Đông thê thê như gió thổi u hồn 😢\"");
    assert_literal_string(token.get_literal(), "Đông thê thê như gió thổi u hồn 😢");
    assert_eq!(token.get_line(), 3);

    let token = &token_list[9];
    assert_eq!(token.get_type(), TokenType::Semicolon);
    assert_eq!(token.get_lexeme(), ";");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 3);

    let token = &token_list[10];
    assert_eq!(token.get_type(), TokenType::Var);
    assert_eq!(token.get_lexeme(), "var");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 6);

    let token = &token_list[11];
    assert_eq!(token.get_type(), TokenType::Identifier);
    assert_eq!(token.get_lexeme(), "str2");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 6);

    let token = &token_list[12];
    assert_eq!(token.get_type(), TokenType::Equal);
    assert_eq!(token.get_lexeme(), "=");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 6);

    let token = &token_list[13];
    assert_eq!(token.get_type(), TokenType::String);
    assert_eq!(token.get_lexeme(), "\"秋の終わり\"");
    assert_literal_string(token.get_literal(), "秋の終わり");
    assert_eq!(token.get_line(), 6);

    let token = &token_list[14];
    assert_eq!(token.get_type(), TokenType::Semicolon);
    assert_eq!(token.get_lexeme(), ";");
    assert_literal_none(token.get_literal());
    assert_eq!(token.get_line(), 6);

    let token = &token_list[15];
    assert_eq!(token.get_type(), TokenType::Eof);
    assert_eq!(token.get_lexeme(), "");
    assert_literal_none(token.get_literal());
    // 24 lines in the script file.
    assert_eq!(token.get_line(), 24);
}
