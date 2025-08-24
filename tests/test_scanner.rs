// Date Created: 05/06/2025.

//! Uses data from `./data/`.
//! 
//! To run test for this module only: 
//! 
//!     * cargo test --test test_scanner
//! 
//! To run a specific test method: 
//! 
//!     * cargo test test_scanner_identifiers -- --exact [--nocapture]
//!     * cargo test test_scanner_keywords -- --exact [--nocapture]
//!     * cargo test test_scanner_numbers -- --exact [--nocapture]
//!     * cargo test test_scanner_punctuators -- --exact [--nocapture]
//!     * cargo test test_scanner_strings -- --exact [--nocapture]
//!     * cargo test test_scanner_whitespace -- --exact [--nocapture]
//!     * cargo test test_scanner_sample -- --exact [--nocapture]
//!     * cargo test test_scanner_utf8_text -- --exact [--nocapture]
//! 
//!     * cargo test test_scanner_generics -- --exact [--nocapture]
//! 

mod test_common;

use crate::test_common::{
    get_script_contents,
    assert_literal_number,
    assert_literal_string,
    assert_literal_none,
    TestScriptAndResult,
    TestScriptAndResults,
    assert_scanner_result,
};

use rlox::token_type::TokenType;
use rlox::scanner::Scanner;

#[test]
fn test_scanner_identifiers() {
    let res = get_script_contents("./tests/data/scanning/identifiers.lox");
    // Read script file was successful.
    assert_eq!(res.is_err(), false);

    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Scanning was successful.
    assert_eq!(res.is_err(), false);

    let token_list = res.unwrap();

    assert_eq!(token_list.len(), 9);

    let token = &token_list[0];
    assert_eq!(token.token_type(), TokenType::Identifier);
    assert_eq!(token.lexeme(), "andy");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[1];
    assert_eq!(token.token_type(), TokenType::Identifier);
    assert_eq!(token.lexeme(), "formless");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[2];
    assert_eq!(token.token_type(), TokenType::Identifier);
    assert_eq!(token.lexeme(), "fo");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[3];
    assert_eq!(token.token_type(), TokenType::Identifier);
    assert_eq!(token.lexeme(), "_");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[4];
    assert_eq!(token.token_type(), TokenType::Identifier);
    assert_eq!(token.lexeme(), "_123");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[5];
    assert_eq!(token.token_type(), TokenType::Identifier);
    assert_eq!(token.lexeme(), "_abc");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[6];
    assert_eq!(token.token_type(), TokenType::Identifier);
    assert_eq!(token.lexeme(), "ab123");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[7];
    assert_eq!(token.token_type(), TokenType::Identifier);
    assert_eq!(token.lexeme(), "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 2);

    let token = &token_list[8];
    assert_eq!(token.token_type(), TokenType::Eof);
    assert_eq!(token.lexeme(), "");
    assert_literal_none(token.literal());
    // 13 is correct. There are 13 lines: but blank line is trimmed off.
    assert_eq!(token.line(), 12);
}

#[test]
fn test_scanner_keywords() {
    let res = get_script_contents("./tests/data/scanning/keywords.lox");
    // Read script file was successful.
    assert_eq!(res.is_err(), false);

    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Scanning was successful.
    assert_eq!(res.is_err(), false);

    let token_list = res.unwrap();

    assert_eq!(token_list.len(), 16);

    let token = &token_list[0];
    assert_eq!(token.token_type(), TokenType::And);
    assert_eq!(token.lexeme(), "and");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);
	
    let token = &token_list[1];
    assert_eq!(token.token_type(), TokenType::Class);
    assert_eq!(token.lexeme(), "class");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);
	
    let token = &token_list[2];
    assert_eq!(token.token_type(), TokenType::Else);
    assert_eq!(token.lexeme(), "else");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);
	
    let token = &token_list[3];
    assert_eq!(token.token_type(), TokenType::False);
    assert_eq!(token.lexeme(), "false");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);
	
    let token = &token_list[4];
    assert_eq!(token.token_type(), TokenType::For);
    assert_eq!(token.lexeme(), "for");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);
	
    let token = &token_list[5];
    assert_eq!(token.token_type(), TokenType::Fun);
    assert_eq!(token.lexeme(), "fun");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);
	
    let token = &token_list[6];
    assert_eq!(token.token_type(), TokenType::If);
    assert_eq!(token.lexeme(), "if");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);
	
    let token = &token_list[7];
    assert_eq!(token.token_type(), TokenType::Nil);
    assert_eq!(token.lexeme(), "nil");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);
	
    let token = &token_list[8];
    assert_eq!(token.token_type(), TokenType::Or);
    assert_eq!(token.lexeme(), "or");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);
	
    let token = &token_list[9];
    assert_eq!(token.token_type(), TokenType::Return);
    assert_eq!(token.lexeme(), "return");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);
	
    let token = &token_list[10];
    assert_eq!(token.token_type(), TokenType::Super);
    assert_eq!(token.lexeme(), "super");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);
	
    let token = &token_list[11];
    assert_eq!(token.token_type(), TokenType::This);
    assert_eq!(token.lexeme(), "this");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);
	
    let token = &token_list[12];
    assert_eq!(token.token_type(), TokenType::True);
    assert_eq!(token.lexeme(), "true");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);
	
    let token = &token_list[13];
    assert_eq!(token.token_type(), TokenType::Var);
    assert_eq!(token.lexeme(), "var");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);
	
    let token = &token_list[14];
    assert_eq!(token.token_type(), TokenType::While);
    assert_eq!(token.lexeme(), "while");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);
	
    let token = &token_list[15];
    assert_eq!(token.token_type(), TokenType::Eof);
    assert_eq!(token.lexeme(), "");
    assert_literal_none(token.literal());
    // There are 19 lines in the script file: but blank line is trimmed off.
    assert_eq!(token.line(), 18);
}

#[test]
// To fully make sense of test conditions in this method, one should read this 
// section https://craftinginterpreters.com/scanning.html#number-literals of 
// the book fairly thoroughly: particularly the leading and trailing decimal
// point.
fn test_scanner_numbers() {
    let res = get_script_contents("./tests/data/scanning/numbers.lox");
    // Read script file was successful.
    assert_eq!(res.is_err(), false);

    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Scanning was successful.
    assert_eq!(res.is_err(), false);

    let token_list = res.unwrap();

    assert_eq!(token_list.len(), 7);

    let token = &token_list[0];
    assert_eq!(token.token_type(), TokenType::Number);
    assert_eq!(token.lexeme(), "123");
    assert_literal_number(token.literal(), 123.0);
    assert_eq!(token.line(), 1);

    let token = &token_list[1];
    assert_eq!(token.token_type(), TokenType::Number);
    assert_eq!(token.lexeme(), "123.456");
    assert_literal_number(token.literal(), 123.456);
    assert_eq!(token.line(), 2);
    
    let token = &token_list[2];
    assert_eq!(token.token_type(), TokenType::Dot);
    assert_eq!(token.lexeme(), ".");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 3);

    let token = &token_list[3];
    assert_eq!(token.token_type(), TokenType::Number);
    assert_eq!(token.lexeme(), "456");
    assert_literal_number(token.literal(), 456.0);
    assert_eq!(token.line(), 3);

    let token = &token_list[4];
    assert_eq!(token.token_type(), TokenType::Number);
    assert_eq!(token.lexeme(), "123");
    assert_literal_number(token.literal(), 123.0);
    assert_eq!(token.line(), 4);

    let token = &token_list[5];
    assert_eq!(token.token_type(), TokenType::Dot);
    assert_eq!(token.lexeme(), ".");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 4);    

    let token = &token_list[6];
    assert_eq!(token.token_type(), TokenType::Eof);
    assert_eq!(token.lexeme(), "");
    assert_literal_none(token.literal());
    // There are 13 lines in the script file: but blank line is trimmed off.
    assert_eq!(token.line(), 12);
}

#[test]
fn test_scanner_punctuators() {
    let res = get_script_contents("./tests/data/scanning/punctuators.lox");
    // Read script file was successful.
    assert_eq!(res.is_err(), false);

    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Scanning was successful.
    assert_eq!(res.is_err(), false);

    let token_list = res.unwrap();

    assert_eq!(token_list.len(), 19);

    let token = &token_list[0];
    assert_eq!(token.token_type(), TokenType::LeftParen);
    assert_eq!(token.lexeme(), "(");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[1];
    assert_eq!(token.token_type(), TokenType::RightParen);
    assert_eq!(token.lexeme(), ")");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[2];
    assert_eq!(token.token_type(), TokenType::LeftBrace);
    assert_eq!(token.lexeme(), "{");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[3];
    assert_eq!(token.token_type(), TokenType::RightBrace);
    assert_eq!(token.lexeme(), "}");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[4];
    assert_eq!(token.token_type(), TokenType::Semicolon);
    assert_eq!(token.lexeme(), ";");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[5];
    assert_eq!(token.token_type(), TokenType::Comma);
    assert_eq!(token.lexeme(), ",");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[6];
    assert_eq!(token.token_type(), TokenType::Plus);
    assert_eq!(token.lexeme(), "+");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[7];
    assert_eq!(token.token_type(), TokenType::Minus);
    assert_eq!(token.lexeme(), "-");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[8];
    assert_eq!(token.token_type(), TokenType::Star);
    assert_eq!(token.lexeme(), "*");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[9];
    assert_eq!(token.token_type(), TokenType::BangEqual);
    assert_eq!(token.lexeme(), "!=");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[10];
    assert_eq!(token.token_type(), TokenType::EqualEqual);
    assert_eq!(token.lexeme(), "==");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[11];
    assert_eq!(token.token_type(), TokenType::LessEqual);
    assert_eq!(token.lexeme(), "<=");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[12];
    assert_eq!(token.token_type(), TokenType::GreaterEqual);
    assert_eq!(token.lexeme(), ">=");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[13];
    assert_eq!(token.token_type(), TokenType::BangEqual);
    assert_eq!(token.lexeme(), "!=");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[14];
    assert_eq!(token.token_type(), TokenType::Less);
    assert_eq!(token.lexeme(), "<");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[15];
    assert_eq!(token.token_type(), TokenType::Greater);
    assert_eq!(token.lexeme(), ">");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[16];
    assert_eq!(token.token_type(), TokenType::Slash);
    assert_eq!(token.lexeme(), "/");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[17];
    assert_eq!(token.token_type(), TokenType::Dot);
    assert_eq!(token.lexeme(), ".");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[18];
    assert_eq!(token.token_type(), TokenType::Eof);
    assert_eq!(token.lexeme(), "");
    assert_literal_none(token.literal());
    // The script file has 22 lines: but blank line is trimmed off.
    assert_eq!(token.line(), 21);
}

#[test]
fn test_scanner_strings() {
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
    assert_eq!(token.token_type(), TokenType::String);
    assert_eq!(token.lexeme(), "\"\"");
    assert_literal_string(token.literal(), "");
    assert_eq!(token.line(), 1);

    let token = &token_list[1];
    // Similar to the above...
    assert_eq!(token.token_type(), TokenType::String);
    assert_eq!(token.lexeme(), "\"string\"");
    assert_literal_string(token.literal(), "string");
    assert_eq!(token.line(), 2);

    let token = &token_list[2];
    assert_eq!(token.token_type(), TokenType::Eof);
    assert_eq!(token.lexeme(), "");
    assert_literal_none(token.literal());
    // Test script file has 6 lines.
    assert_eq!(token.line(), 6);
}

#[test]
fn test_scanner_whitespace() {
    let res = get_script_contents("./tests/data/scanning/whitespace.lox");
    // Read script file was successful.
    assert_eq!(res.is_err(), false);

    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Scanning was successful.
    assert_eq!(res.is_err(), false);

    let token_list = res.unwrap();

    assert_eq!(token_list.len(), 5);

    let token = &token_list[0];
    assert_eq!(token.token_type(), TokenType::Identifier);
    assert_eq!(token.lexeme(), "space");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[1];
    assert_eq!(token.token_type(), TokenType::Identifier);
    assert_eq!(token.lexeme(), "tabs");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[2];
    assert_eq!(token.token_type(), TokenType::Identifier);
    assert_eq!(token.lexeme(), "newlines");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[3];
    assert_eq!(token.token_type(), TokenType::Identifier);
    assert_eq!(token.lexeme(), "end");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 6);

    let token = &token_list[4];
    assert_eq!(token.token_type(), TokenType::Eof);
    assert_eq!(token.lexeme(), "");
    assert_literal_none(token.literal());
    //The blank line is trimmed off.
    assert_eq!(token.line(), 12);
}

#[test]
fn test_scanner_sample() {
    let res = get_script_contents("./tests/data/scanning/sample.lox");
    // Read script file was successful.
    assert_eq!(res.is_err(), false);

    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Scanning was successful.
    assert_eq!(res.is_err(), false);

    let token_list = res.unwrap();

    assert_eq!(token_list.len(), 80);

    let token = &token_list[0];
    assert_eq!(token.token_type(), TokenType::Class);
    assert_eq!(token.lexeme(), "class");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[8];
    assert_eq!(token.token_type(), TokenType::String);
    assert_eq!(token.lexeme(), "\"Fry until golden brown.\"");
    assert_literal_string(token.literal(), "Fry until golden brown.");
    assert_eq!(token.line(), 3);

    let token = &token_list[13];
    assert_eq!(token.token_type(), TokenType::Identifier);
    assert_eq!(token.lexeme(), "BostonCream");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 7);

    let token = &token_list[27];
    assert_eq!(token.token_type(), TokenType::Print);
    assert_eq!(token.lexeme(), "print");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 10);

    let token = &token_list[55];
    assert_eq!(token.token_type(), TokenType::For);
    assert_eq!(token.lexeme(), "for");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 20);

    let token = &token_list[64];
    assert_eq!(token.token_type(), TokenType::Number);
    assert_eq!(token.lexeme(), "5");
    assert_literal_number(token.literal(), 5.00);
    assert_eq!(token.line(), 20);

    let token = &token_list[79];
    assert_eq!(token.token_type(), TokenType::Eof);
    assert_eq!(token.lexeme(), "");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 22);
}

#[test]
fn test_scanner_utf8_text() {
    let res = get_script_contents("./tests/data/scanning/utf8_text.lox");
    // Read script file was successful.
    assert_eq!(res.is_err(), false);

    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Scanning was successful.
    assert_eq!(res.is_err(), false);

    let token_list = res.unwrap();

    assert_eq!(token_list.len(), 16);

    let token = &token_list[0];
    assert_eq!(token.token_type(), TokenType::Var);
    assert_eq!(token.lexeme(), "var");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[1];
    assert_eq!(token.token_type(), TokenType::Identifier);
    assert_eq!(token.lexeme(), "str");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[2];
    assert_eq!(token.token_type(), TokenType::Equal);
    assert_eq!(token.lexeme(), "=");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[3];
    assert_eq!(token.token_type(), TokenType::String);
    assert_eq!(token.lexeme(), "\"運去英雄飲恨多\"");
    assert_literal_string(token.literal(), "運去英雄飲恨多");
    assert_eq!(token.line(), 1);

    let token = &token_list[4];
    assert_eq!(token.token_type(), TokenType::Semicolon);
    assert_eq!(token.lexeme(), ";");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 1);

    let token = &token_list[5];
    assert_eq!(token.token_type(), TokenType::Var);
    assert_eq!(token.lexeme(), "var");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 3);

    let token = &token_list[6];
    assert_eq!(token.token_type(), TokenType::Identifier);
    assert_eq!(token.lexeme(), "str1");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 3);

    let token = &token_list[7];
    assert_eq!(token.token_type(), TokenType::Equal);
    assert_eq!(token.lexeme(), "=");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 3);

    let token = &token_list[8];
    assert_eq!(token.token_type(), TokenType::String);
    assert_eq!(token.lexeme(), "\"Đông thê thê như gió thổi u hồn 😢\"");
    assert_literal_string(token.literal(), "Đông thê thê như gió thổi u hồn 😢");
    assert_eq!(token.line(), 3);

    let token = &token_list[9];
    assert_eq!(token.token_type(), TokenType::Semicolon);
    assert_eq!(token.lexeme(), ";");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 3);

    let token = &token_list[10];
    assert_eq!(token.token_type(), TokenType::Var);
    assert_eq!(token.lexeme(), "var");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 6);

    let token = &token_list[11];
    assert_eq!(token.token_type(), TokenType::Identifier);
    assert_eq!(token.lexeme(), "str2");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 6);

    let token = &token_list[12];
    assert_eq!(token.token_type(), TokenType::Equal);
    assert_eq!(token.lexeme(), "=");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 6);

    let token = &token_list[13];
    assert_eq!(token.token_type(), TokenType::String);
    assert_eq!(token.lexeme(), "\"秋の終わり\"");
    assert_literal_string(token.literal(), "秋の終わり");
    assert_eq!(token.line(), 6);

    let token = &token_list[14];
    assert_eq!(token.token_type(), TokenType::Semicolon);
    assert_eq!(token.lexeme(), ";");
    assert_literal_none(token.literal());
    assert_eq!(token.line(), 6);

    let token = &token_list[15];
    assert_eq!(token.token_type(), TokenType::Eof);
    assert_eq!(token.lexeme(), "");
    assert_literal_none(token.literal());
    // 24 lines in the script file: but blank line is trimmed off.
    assert_eq!(token.line(), 23);
}

fn get_generic_script_results<'a>() -> TestScriptAndResults<'a> {    
    vec![
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/string
        TestScriptAndResult {
            script_name: "./tests/data/string/unterminated.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at '\0': Unterminated string."],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/
        //
        // I have missed these root master/test/ author-provided scripts till after 
        // completing Chapter 13.
        // 
        // The author does not seem to handle empty source text? The "Source text is empty."
        // message is my own, the logic in the scanner to detect empty source text is also
        // my own.
        TestScriptAndResult {
            script_name: "./tests/data/empty_file.lox",
            expected_result: false,
            expected_output: vec!["Source text is empty."],
        },
        // Note on `unexpected_character.lox` -- This script has two errors:
        //
        //     `[line 3] Error: Unexpected character.` -- is the scanner error.
        //     `[java line 3] Error at 'b': Expect ')' after arguments.` -- is a parser error.
        //
        // In this implementation, if the Scanner is in error, everything stop. And so 
        // on for the Parser and the Resolver.
        // 
        // As such, this is a Scanner test script.
        TestScriptAndResult {
            script_name: "./tests/data/unexpected_character.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at '|': Unexpected character: |."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/scanning/multi_errors.lox",
            expected_result: false,
            expected_output: vec!["[line 7] Error at '|': Unexpected character: |.",
                "[line 9] Error at '?': Unexpected character: ?.",
                "[line 13] Error at '%': Unexpected character: %."],
        },
    ] // cargo test test_scanner_generics -- --exact
}

#[test]
// On author's https://github.com/munificent/craftinginterpreters/tree/master/test/
//
// I have missed these root master/test/ author-provided scripts till after 
// completing Chapter 13.
// 
// On `empty_file.lox`:
//
// The author does not seem to handle empty source text? The "Source text is empty."
// message is my own, the logic in the scanner to detect empty source text is also
// my own.
//
// On `unexpected_character.lox` -- This script has two errors:
//
//     `[line 3] Error: Unexpected character.` -- is the scanner error.
//     `[java line 3] Error at 'b': Expect ')' after arguments.` -- is a parser error.
//
// In this implementation, if the Scanner is in error, everything stop. And so 
// on for the Parser and the Resolver.
// 
// As such, this is a Scanner test script.
fn test_scanner_generics() {
    let generic_script_results = get_generic_script_results();

    for entry in generic_script_results {
        let res = get_script_contents(entry.script_name);
        // Read script file was successful.
        assert!(res.is_ok(), "Error loading {}", entry.script_name);

        let res = Scanner::new(&res.unwrap()).scan_tokens();
        assert_scanner_result(&entry, &res);
   }    
}
