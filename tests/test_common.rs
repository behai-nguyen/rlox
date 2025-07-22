// Date Created: 05/06/2025.

use std::fs::read_to_string;
use std::io::Cursor;

use rlox::lox_error::LoxError;
use rlox::token::{LiteralValue, Token};
use rlox::scanner::Scanner;
use rlox::expr::Expr;
use rlox::stmt::Stmt;
use rlox::parser::Parser;
use rlox::interpreter::Interpreter;

#[allow(dead_code)]
pub struct TestScriptAndResult<'a> {
    pub script_name: &'a str,
    pub expected_result: bool,
    pub expected_output: Vec<&'a str>,
}

#[allow(dead_code)]
pub type TestScriptAndResults<'a> = Vec<TestScriptAndResult<'a>>;

#[allow(dead_code)]
pub type ScannerResult = Result<Vec<Token>, LoxError>;

#[allow(dead_code)]
pub type ParserResult = Result<Vec<Stmt>, LoxError>;

#[allow(dead_code)]
pub type InterpreterResult = Result<(), LoxError>;

#[allow(dead_code)]
pub fn get_script_contents(script_name: &str) -> Result<String, std::io::Error> {
    let contents = read_to_string(script_name)?;

    Ok(contents)
}

#[allow(dead_code)]
// Load script and scan it.
pub fn assert_scan_script(script_name: &str) -> Vec<Token> {
    let res = read_to_string(script_name);
    // Check that the script file was loaded successfully.
    assert!(res.is_ok(), "assert_scan_script() read error: {}", script_name);

    // Attempt to scan the loaded script file.
    let res = Scanner::new(&res.unwrap()).scan_tokens();
    // Check that the script file was scanned successfully.
    assert!(res.is_ok(), "assert_scan_script() scan error: {}", script_name);

    res.unwrap()
}

#[allow(dead_code)]
// Load script and parse it.
pub fn assert_parse_script_expression(script_name: &str) -> Expr {
    // Ensure scripted loaded and scanned successfully.
    let tokens = assert_scan_script(script_name);

    // Attempt to parse.
    let mut parser = Parser::new(&tokens);
    let res = parser.parse_single_expression();

    // Ensure parsing is successful.
    assert!(res.is_ok(), "assert_parse_script_expression() error: {}", script_name);
    
    res.unwrap()
}

#[allow(dead_code)]
// Load script and parse it.
pub fn assert_parse_script_statements(script_name: &str) -> Vec<Stmt> {
    // Ensure scripted loaded and scanned successfully.
    let tokens = assert_scan_script(script_name);

    // Attempt to parse.
    let mut parser = Parser::new(&tokens);
    let res = parser.parse();

    // Ensure parsing is successful.
    assert!(res.is_ok(), "assert_parse_script_statements() error: {}", script_name);
    
    res.unwrap()
}

#[allow(dead_code)]
// Parse a single line script.
pub fn assert_parse_line_expression(line: &str) -> Expr {
    // Attempt to scan the script line.
    let res = Scanner::new(line).scan_tokens();

    // Check that the script line was scanned successfully.
    assert!(res.is_ok(), "assert_parse_line_expression() scan error: {}", line);

    let tokens = res.unwrap();

    // Attempt to parse.
    let mut parser = Parser::new(&tokens);
    let res = parser.parse_single_expression();

    // Ensure parsing is successful.
    assert!(res.is_ok(), "assert_parse_line_expression() parse error: {}", line);
    
    res.unwrap()
}

#[allow(dead_code)]
pub fn assert_literal_number(val: &Option<LiteralValue>, number: f64) {
    assert_eq!(false, val.is_none());

    if let LiteralValue::Number(n) = val.as_ref().unwrap() {
        assert_eq!(number, *n);
    } else {
        assert_eq!(false, true, "Expected a number value {}", number);
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn assert_literal_none(val: &Option<LiteralValue>) {
    assert_eq!(true, val.is_none());
}

#[allow(dead_code)]
pub fn assert_scanner_result(tested_entry: &TestScriptAndResult, test_result: &ScannerResult) {
    // Check scanning result.
    match tested_entry.expected_result {
        true => {
            assert!(test_result.is_ok(), "1. Error in {}", tested_entry.script_name);
            // TO_DO: there are no true cases yet...
        },
        false => {
            assert!(test_result.is_err(), "3. Error in {}", tested_entry.script_name);
            assert_eq!(test_result.as_ref().unwrap_err().to_string(), 
                tested_entry.expected_output[0], "4. Error in {}", tested_entry.script_name);
        }
    }
}

#[allow(dead_code)]
pub fn assert_parser_result(tested_entry: &TestScriptAndResult, test_result: &ParserResult) {
    // Check Interpreting/evaluating result.
    match tested_entry.expected_result {
        true => {
            assert!(test_result.is_ok(), "1. Error in {}", tested_entry.script_name);
            // TO_DO: there are no true cases yet...
        },
        false => {
            assert!(test_result.is_err(), "3. Error in {}", tested_entry.script_name);
            assert_eq!(test_result.as_ref().unwrap_err().to_string(), 
                tested_entry.expected_output[0], "4. Error in {}", tested_entry.script_name);
        }
    }
}

#[allow(dead_code)]
pub fn make_interpreter<W: std::io::Write>(writer: W) -> Interpreter<W> {
    Interpreter::new(writer)
}

#[allow(dead_code)]
pub fn extract_output_lines(interpreter: &Interpreter<Cursor<Vec<u8>>>) -> Vec<String> {
    let output = interpreter.get_output().clone().into_inner();
    String::from_utf8(output).unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}

#[allow(dead_code)]
pub fn assert_interpreter_result(tested_entry: &TestScriptAndResult, 
    test_result: &InterpreterResult,
    interpreter: &Interpreter<Cursor<Vec<u8>>>) {
    // Check Interpreting/evaluating result.
    match tested_entry.expected_result {
        true => {
            assert!(test_result.is_ok(), "1. Error in {}", tested_entry.script_name);
            // Extract output and ensure matching expected output.
            let lines = extract_output_lines(&interpreter);
            assert_eq!(lines, tested_entry.expected_output, 
                "2. Error in {}", tested_entry.script_name);
        },
        false => {
            assert!(test_result.is_err(), "3. Error in {}", tested_entry.script_name);
            assert_eq!(test_result.as_ref().unwrap_err().to_string(), 
                tested_entry.expected_output[0], "4. Error in {}", tested_entry.script_name);
        }
    }
}
