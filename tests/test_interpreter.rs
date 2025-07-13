// Date Created: 11/07/2025.

//! Uses data from `./data/expressions/`.
//! 
//! To run test for this module only: 
//! 
//!     * cargo test --test test_interpreter
//! 
//! To run a specific test method: 
//! 
//!     * cargo test interpreter_01 -- --exact [--nocapture]
//! 
//!     * cargo test interpreter_02_binary_greater -- --exact [--nocapture]
//!     * cargo test interpreter_03_binary_greater_equal -- --exact [--nocapture]
//!     * cargo test interpreter_04_binary_less -- --exact [--nocapture]
//!     * cargo test interpreter_05_binary_less_equal -- --exact [--nocapture]
//!     * cargo test interpreter_06_binary_bang_equal -- --exact [--nocapture]
//!     * cargo test interpreter_07_binary_equal_equal -- --exact [--nocapture]
//!     * cargo test interpreter_08_binary_minus -- --exact [--nocapture]
//!     * cargo test interpreter_09_binary_plus -- --exact [--nocapture]
//!     * cargo test interpreter_10_binary_slash -- --exact [--nocapture]
//!     * cargo test interpreter_11_binary_star -- --exact [--nocapture]
//! 
//!     * cargo test interpreter_12_literal -- --exact [--nocapture]
//! 
//!     * cargo test interpreter_13_unary_bang -- --exact [--nocapture]
//!     * cargo test interpreter_14_unary_minus -- --exact [--nocapture]

mod test_common;

use crate::test_common::{assert_parse_script, assert_parse_line};

use rlox::interpreter::Interpreter;

fn make_interpreter() -> Interpreter {
    Interpreter{}
}

#[test]
// The test script is from 
//     https://github.com/munificent/craftinginterpreters/blob/master/test/expressions/evaluate.lox
fn interpreter_01() {
    // Ensure script is loaded, scanned and parsed successfully.
    let expr = assert_parse_script("./tests/data/expressions/evaluate.lox");

    // Test interpreting/evaluating.
    let interpreter = make_interpreter();
    let res = interpreter.interpret(&expr);

    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false);

    // Note: the scanner normalises numeric (f64) literals, and printer uses 
    // {:?} to print out f64 value, see method visit_literal_expr().
    assert_eq!("2.0", res.unwrap());
} 

#[test]
fn interpreter_02_binary_greater() {
    let interpreter = make_interpreter();

    // Test 1.    
    let expr = assert_parse_line("((4.5 / 2) * 2) > 3.5");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("true", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line("((4.5 / 2) * 2) > 8");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("false", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line("((4.5 / 2) * 2) > \"3.5\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), true, "3");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "3");
}

#[test]
fn interpreter_03_binary_greater_equal() {
    let interpreter = make_interpreter();

    // Test 1.    
    let expr = assert_parse_line("((4.5 / 2) * 2) >= 4.25");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("true", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line("(4.5 / 2) >= 8");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("false", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line("(4.5 / 2) >= \"3.5\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), true, "3");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "3");
}

#[test]
fn interpreter_04_binary_less()  {
    let interpreter = make_interpreter();

    // Test 1.    
    let expr = assert_parse_line("((4.5 / 2) * 2) < 4.52");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("true", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line("(4.5 / 2) < 1.5");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("false", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line("(4.5 / 2) < \"3.5\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), true, "3");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "3");
}

#[test]
fn interpreter_05_binary_less_equal()  {
    let interpreter = make_interpreter();

    // Test 1.    
    let expr = assert_parse_line("((4.5 / 2) * 2) <= 4.50");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("true", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line("(4.5 / 2) <= 1.72");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("false", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line("(4.5 / 2) <= \"abc\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), true, "3");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "3");
}

#[test]
fn interpreter_06_binary_bang_equal()  {
    let interpreter = make_interpreter();

    // Test 1.    
    let expr = assert_parse_line("((4.5 / 2) * 2) != 4.50");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    // They are equal.
    assert_eq!("false", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line("(4.5 / 2) != 1.72");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    // They are not equal.
    assert_eq!("true", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line("\"abc\" != \"abc\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), false, "3");
    // They are equal.
    assert_eq!("false", res.unwrap(), "3");

    // Test 4.
    let expr = assert_parse_line("nil != nil");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), false, "4");
    // They are equal.
    assert_eq!("false", res.unwrap(), "4");

    // Test 5.
    let expr = assert_parse_line("nil != 134");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), false, "5");
    // They are not equal.
    assert_eq!("true", res.unwrap(), "5");
}

#[test]
fn interpreter_07_binary_equal_equal()  {
    let interpreter = make_interpreter();

    // Test 1.    
    let expr = assert_parse_line("((4.5 / 2) * 2) == 4.50");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    // They are equal.
    assert_eq!("true", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line("(4.5 / 2) == 1.72");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    // They are not equal.
    assert_eq!("false", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line("\"abc\" == \"abc\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), false, "3");
    // They are equal.
    assert_eq!("true", res.unwrap(), "3");

    // Test 4.
    let expr = assert_parse_line("nil == nil");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), false, "4");
    // They are equal.
    assert_eq!("true", res.unwrap(), "4");

    // Test 5.
    let expr = assert_parse_line("nil == 134");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), false, "5");
    // They are not equal.
    assert_eq!("false", res.unwrap(), "5");

    // Test 6.
    let expr = assert_parse_line("\"abc\" == \"ABC\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating completed successfully.
    assert_eq!(res.is_err(), false, "6");
    // They are not equal.
    assert_eq!("false", res.unwrap(), "6");

    // Test 7.
    let expr = assert_parse_line("\"abc\" == 123");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating completed successfully.
    assert_eq!(res.is_err(), false, "7");
    // They are not equal.
    assert_eq!("false", res.unwrap(), "7");
}

#[test]
fn interpreter_08_binary_minus()  {
    let interpreter = make_interpreter();

    // Test 1.    
    let expr = assert_parse_line("((4.5 / 2) * 2) - 1.25");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("3.25", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line("7 - 9");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("-2.0", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line("\"abc\" - (4.5 / 2)");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), true, "3");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "3");

    // Test 4.
    let expr = assert_parse_line("\"abc\" - \"xyz\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), true, "4");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "4");
}

#[test]
fn interpreter_09_binary_plus()  {
    let interpreter = make_interpreter();

    // Test 1.    
    let expr = assert_parse_line("((4.5 / 2) * 2) + 1.25");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("5.75", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line("7 + 9");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("16.0", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line("\"abc\" + (4.5 / 2)");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), true, "3");
    let err = res.unwrap_err();
    assert_eq!("Operands must be two numbers or two strings.", err.get_err_msg(), "3");

    // Test 4.
    let expr = assert_parse_line("\"abc \" + \"def\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "4");
    assert_eq!("abc def", res.unwrap(), "4");

    // Test 5.
    let expr = assert_parse_line("\"abc \" + 42");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), true, "5");
    let err = res.unwrap_err();
    assert_eq!("Operands must be two numbers or two strings.", err.get_err_msg(), "5");
}

#[test]
fn interpreter_10_binary_slash()  {
    let interpreter = make_interpreter();

    // Test 1.    
    let expr = assert_parse_line("((4.5 / 2) * 2) / 1.25");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("3.6", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line("7 / 9");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("0.7777777777777778", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line("(\"abc\" / (4.5 / 2))");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), true, "3");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "3");
}

#[test]
fn interpreter_11_binary_star()  {
    let interpreter = make_interpreter();

    // Test 1.    
    let expr = assert_parse_line("((4.5 / 2) * 2) * 1.25");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("5.625", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line("7 * 9");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("63.0", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line("(\"abc\" * (4.5 / 2))");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), true, "3");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "3");

    // Test 4.
    let expr = assert_parse_line("true * 3");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), true, "4");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "4");
}

#[test]
fn interpreter_12_literal() {
    let interpreter = make_interpreter();

    // Test 1.
    let expr = assert_parse_line("7.04");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("7.04", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line("\"abc def\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("abc def", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line("false");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "3");
    assert_eq!("false", res.unwrap(), "3");

    // Test 4.
    let expr = assert_parse_line("nil");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "4");
    assert_eq!("nil", res.unwrap(), "4");
}

#[test]
fn interpreter_13_unary_bang() {
    let interpreter = make_interpreter();

    // Test 1.
    let expr = assert_parse_line("!7.04");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("false", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line("!\"abc def\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("false", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line("!true");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "3");
    assert_eq!("false", res.unwrap(), "3");

    // Test 4.
    let expr = assert_parse_line("!false");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "4");
    assert_eq!("true", res.unwrap(), "4");

    // Test 5.
    let expr = assert_parse_line("!nil");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "5");
    assert_eq!("true", res.unwrap(), "5");
}

#[test]
fn interpreter_14_unary_minus() {
    let interpreter = make_interpreter();

    // Test 1.
    let expr = assert_parse_line("-7.04");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("-7.04", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line("-(((4.5 / 2) * 2) * 1.25)");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("-5.625", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line("-(-7.04)");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "3");
    assert_eq!("7.04", res.unwrap(), "3");

    // Test 4.
    let expr = assert_parse_line("-false");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), true, "4");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "4");

    // Test 5.
    let expr = assert_parse_line("-\"abc\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), true, "5");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "5");
}
