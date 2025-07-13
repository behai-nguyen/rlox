// Date Created: 05/06/2025.

use std::fs::read_to_string;

use rlox::token::{LiteralValue, Token};
use rlox::scanner::Scanner;
use rlox::expr::Expr;
use rlox::parser::Parser;

pub fn get_script_contents(scriptfile: &str) -> Result<String, std::io::Error> {
    let contents = read_to_string(scriptfile)?;

    Ok(contents)
}

// Load script and scan it.
pub fn assert_scan_script(scriptfile: &str) -> Vec<Token> {
    let res = read_to_string(scriptfile);
    // Check that the script file was loaded successfully.
    assert!(res.is_ok(), "Failed to load or scan script: {}", scriptfile);

    // Attempt to scan the loaded script file.
    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Check that the script file was scanned successfully.
    assert_eq!(false, res.is_err());

    res.unwrap()
}

// Load script and parse it.
pub fn assert_parse_script(scriptfile: &str) -> Expr {
    // Ensure scripted loaded and scanned successfully.
    let tokens = assert_scan_script(scriptfile);

    // Attempt to parse.
    let parser = Parser::new(&tokens);
    let res = parser.parse();

    // Ensure parsing is successful.
    assert_eq!(res.is_err(), false);
    
    res.unwrap()
}

// Parse a single line script.
pub fn assert_parse_line(line: &str) -> Expr {
    // Attempt to scan the script line.
    let res = Scanner::new(line).scan_tokens();

    // Check that the script line was scanned successfully.
    assert!(res.is_ok(), "Failed to scan line: {}", line);

    let tokens = res.unwrap();

    // Attempt to parse.
    let parser = Parser::new(&tokens);
    let res = parser.parse();

    // Ensure parsing is successful.
    assert_eq!(res.is_err(), false);
    
    res.unwrap()
}

pub fn assert_literal_number(val: &Option<LiteralValue>, number: f64) {
    assert_eq!(false, val.is_none());

    if let LiteralValue::Number(n) = val.as_ref().unwrap() {
        assert_eq!(number, *n);
    } else {
        assert_eq!(false, true, "Expected a number value {}", number);
    }
}

pub fn assert_literal_string(val: &Option<LiteralValue>, string: &str) {
    assert_eq!(false, val.is_none());

    if let LiteralValue::String(s) = val.as_ref().unwrap() {
        assert_eq!(string, *s);
    } else {
        assert_eq!(false, true, "Expected a string value {}", string);
    }
}

#[allow(dead_code)]
pub fn assert_literal_boolean(val: &Option<LiteralValue>, boolean: bool) {
    assert_eq!(false, val.is_none());

    if let LiteralValue::Boolean(b) = val.as_ref().unwrap() {
        assert_eq!(boolean, *b);
    } else {
        assert_eq!(false, true, "Expected a Boolean value {}", boolean);
    }    
}

pub fn assert_literal_none(val: &Option<LiteralValue>) {
    assert_eq!(true, val.is_none());
}
