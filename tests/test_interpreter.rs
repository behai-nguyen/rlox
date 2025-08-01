// Date Created: 11/07/2025.

//! Uses data from `./data/expressions/` TO_DO??.
//! 
//! Tests use [`rlox::interpreter::Interpreter`] like `test_statements_state.rs`, but 
//! focuses on the earlier Chapter 7 -- [Evaluating Expressions](https://craftinginterpreters.com/evaluating-expressions.html).
//! 
//! To run test for this module only: 
//! 
//!     * cargo test --test test_interpreter
//! 
//! To run a specific test method: 
//! 
//!     * cargo test test_interpreter_01 -- --exact [--nocapture]
//! 
//!     * cargo test test_interpreter_02_binary_greater -- --exact [--nocapture]
//!     * cargo test test_interpreter_03_binary_greater_equal -- --exact [--nocapture]
//!     * cargo test test_interpreter_04_binary_less -- --exact [--nocapture]
//!     * cargo test test_interpreter_05_binary_less_equal -- --exact [--nocapture]
//!     * cargo test test_interpreter_06_binary_bang_equal -- --exact [--nocapture]
//!     * cargo test test_interpreter_07_binary_equal_equal -- --exact [--nocapture]
//!     * cargo test test_interpreter_08_binary_minus -- --exact [--nocapture]
//!     * cargo test test_interpreter_09_binary_plus -- --exact [--nocapture]
//!     * cargo test test_interpreter_10_binary_slash -- --exact [--nocapture]
//!     * cargo test test_interpreter_11_binary_star -- --exact [--nocapture]
//! 
//!     * cargo test test_interpreter_12_literal -- --exact [--nocapture]
//! 
//!     * cargo test test_interpreter_13_unary_bang -- --exact [--nocapture]
//!     * cargo test test_interpreter_14_unary_minus -- --exact [--nocapture]
//! 
//!     * cargo test test_interpreter_expr -- --exact [--nocapture]
//! 

mod test_common;

use crate::test_common::{
    assert_parse_script_expression, 
    assert_parse_line_expression,
    TestScriptAndResults,
    TestScriptAndResult,
    make_interpreter_stdout,
    make_interpreter_byte_stream,
    assert_parse_script_statements,
    assert_interpreter_result,
};


#[test]
// The test script is from 
//     https://github.com/munificent/craftinginterpreters/blob/master/test/expressions/evaluate.lox
fn test_interpreter_01() {
    // Ensure script is loaded, scanned and parsed successfully.
    let expr = assert_parse_script_expression("./tests/data/expressions/evaluate.lox");

    // Test interpreting/evaluating.
    let mut interpreter = make_interpreter_stdout();
    let res = interpreter.interpret_single_expression(&expr);

    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false);

    // Note: the scanner normalises numeric (f64) literals, and printer uses 
    // {:?} to print out f64 value, see method visit_literal_expr().
    assert_eq!("2.0", res.unwrap());
} 

#[test]
fn test_interpreter_02_binary_greater() {
    let mut interpreter = make_interpreter_stdout();

    // Test 1.    
    let expr = assert_parse_line_expression("((4.5 / 2) * 2) > 3.5");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("true", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line_expression("((4.5 / 2) * 2) > 8");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("false", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line_expression("((4.5 / 2) * 2) > \"3.5\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), true, "3");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "3");
}

#[test]
fn test_interpreter_03_binary_greater_equal() {
    let mut interpreter = make_interpreter_stdout();

    // Test 1.    
    let expr = assert_parse_line_expression("((4.5 / 2) * 2) >= 4.25");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("true", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line_expression("(4.5 / 2) >= 8");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("false", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line_expression("(4.5 / 2) >= \"3.5\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), true, "3");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "3");
}

#[test]
fn test_interpreter_04_binary_less()  {
    let mut interpreter = make_interpreter_stdout();

    // Test 1.    
    let expr = assert_parse_line_expression("((4.5 / 2) * 2) < 4.52");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("true", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line_expression("(4.5 / 2) < 1.5");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("false", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line_expression("(4.5 / 2) < \"3.5\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), true, "3");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "3");
}

#[test]
fn test_interpreter_05_binary_less_equal()  {
    let mut interpreter = make_interpreter_stdout();

    // Test 1.    
    let expr = assert_parse_line_expression("((4.5 / 2) * 2) <= 4.50");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("true", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line_expression("(4.5 / 2) <= 1.72");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("false", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line_expression("(4.5 / 2) <= \"abc\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), true, "3");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "3");
}

#[test]
fn test_interpreter_06_binary_bang_equal()  {
    let mut interpreter = make_interpreter_stdout();

    // Test 1.    
    let expr = assert_parse_line_expression("((4.5 / 2) * 2) != 4.50");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    // They are equal.
    assert_eq!("false", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line_expression("(4.5 / 2) != 1.72");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    // They are not equal.
    assert_eq!("true", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line_expression("\"abc\" != \"abc\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), false, "3");
    // They are equal.
    assert_eq!("false", res.unwrap(), "3");

    // Test 4.
    let expr = assert_parse_line_expression("nil != nil");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), false, "4");
    // They are equal.
    assert_eq!("false", res.unwrap(), "4");

    // Test 5.
    let expr = assert_parse_line_expression("nil != 134");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), false, "5");
    // They are not equal.
    assert_eq!("true", res.unwrap(), "5");
}

#[test]
fn test_interpreter_07_binary_equal_equal()  {
    let mut interpreter = make_interpreter_stdout();

    // Test 1.    
    let expr = assert_parse_line_expression("((4.5 / 2) * 2) == 4.50");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    // They are equal.
    assert_eq!("true", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line_expression("(4.5 / 2) == 1.72");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    // They are not equal.
    assert_eq!("false", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line_expression("\"abc\" == \"abc\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), false, "3");
    // They are equal.
    assert_eq!("true", res.unwrap(), "3");

    // Test 4.
    let expr = assert_parse_line_expression("nil == nil");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), false, "4");
    // They are equal.
    assert_eq!("true", res.unwrap(), "4");

    // Test 5.
    let expr = assert_parse_line_expression("nil == 134");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), false, "5");
    // They are not equal.
    assert_eq!("false", res.unwrap(), "5");

    // Test 6.
    let expr = assert_parse_line_expression("\"abc\" == \"ABC\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating completed successfully.
    assert_eq!(res.is_err(), false, "6");
    // They are not equal.
    assert_eq!("false", res.unwrap(), "6");

    // Test 7.
    let expr = assert_parse_line_expression("\"abc\" == 123");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating completed successfully.
    assert_eq!(res.is_err(), false, "7");
    // They are not equal.
    assert_eq!("false", res.unwrap(), "7");
}

#[test]
fn test_interpreter_08_binary_minus()  {
    let mut interpreter = make_interpreter_stdout();

    // Test 1.    
    let expr = assert_parse_line_expression("((4.5 / 2) * 2) - 1.25");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("3.25", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line_expression("7 - 9");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("-2.0", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line_expression("\"abc\" - (4.5 / 2)");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), true, "3");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "3");

    // Test 4.
    let expr = assert_parse_line_expression("\"abc\" - \"xyz\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), true, "4");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "4");
}

#[test]
fn test_interpreter_09_binary_plus()  {
    let mut interpreter = make_interpreter_stdout();

    // Test 1.    
    let expr = assert_parse_line_expression("((4.5 / 2) * 2) + 1.25");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("5.75", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line_expression("7 + 9");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("16.0", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line_expression("\"abc\" + (4.5 / 2)");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), true, "3");
    let err = res.unwrap_err();
    assert_eq!("Operands must be two numbers or two strings.", err.get_err_msg(), "3");

    // Test 4.
    let expr = assert_parse_line_expression("\"abc \" + \"def\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "4");
    assert_eq!("abc def", res.unwrap(), "4");

    // Test 5.
    let expr = assert_parse_line_expression("\"abc \" + 42");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), true, "5");
    let err = res.unwrap_err();
    assert_eq!("Operands must be two numbers or two strings.", err.get_err_msg(), "5");
}

#[test]
fn test_interpreter_10_binary_slash()  {
    let mut interpreter = make_interpreter_stdout();

    // Test 1.    
    let expr = assert_parse_line_expression("((4.5 / 2) * 2) / 1.25");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("3.6", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line_expression("7 / 9");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("0.7777777777777778", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line_expression("(\"abc\" / (4.5 / 2))");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), true, "3");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "3");
}

#[test]
fn test_interpreter_11_binary_star()  {
    let mut interpreter = make_interpreter_stdout();

    // Test 1.    
    let expr = assert_parse_line_expression("((4.5 / 2) * 2) * 1.25");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("5.625", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line_expression("7 * 9");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("63.0", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line_expression("(\"abc\" * (4.5 / 2))");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), true, "3");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "3");

    // Test 4.
    let expr = assert_parse_line_expression("true * 3");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was result in an error.
    assert_eq!(res.is_err(), true, "4");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "4");
}

#[test]
fn test_interpreter_12_literal() {
    let mut interpreter = make_interpreter_stdout();

    // Test 1.
    let expr = assert_parse_line_expression("7.04");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("7.04", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line_expression("\"abc def\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("abc def", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line_expression("false");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "3");
    assert_eq!("false", res.unwrap(), "3");

    // Test 4.
    let expr = assert_parse_line_expression("nil");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "4");
    assert_eq!("nil", res.unwrap(), "4");
}

#[test]
fn test_interpreter_13_unary_bang() {
    let mut interpreter = make_interpreter_stdout();

    // Test 1.
    let expr = assert_parse_line_expression("!7.04");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("false", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line_expression("!\"abc def\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("false", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line_expression("!true");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "3");
    assert_eq!("false", res.unwrap(), "3");

    // Test 4.
    let expr = assert_parse_line_expression("!false");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "4");
    assert_eq!("true", res.unwrap(), "4");

    // Test 5.
    let expr = assert_parse_line_expression("!nil");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "5");
    assert_eq!("true", res.unwrap(), "5");
}

#[test]
fn test_interpreter_14_unary_minus() {
    let mut interpreter = make_interpreter_stdout();

    // Test 1.
    let expr = assert_parse_line_expression("-7.04");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "1");
    assert_eq!("-7.04", res.unwrap(), "1");

    // Test 2.
    let expr = assert_parse_line_expression("-(((4.5 / 2) * 2) * 1.25)");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "2");
    assert_eq!("-5.625", res.unwrap(), "2");

    // Test 3.
    let expr = assert_parse_line_expression("-(-7.04)");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), false, "3");
    assert_eq!("7.04", res.unwrap(), "3");

    // Test 4.
    let expr = assert_parse_line_expression("-false");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), true, "4");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "4");

    // Test 5.
    let expr = assert_parse_line_expression("-\"abc\"");
    // Test interpreting/evaluating.
    let res = interpreter.interpret_single_expression(&expr);
    // Interpreting/evaluating was successful.
    assert_eq!(res.is_err(), true, "5");
    let err = res.unwrap_err();
    assert_eq!("Operand must be a number.", err.get_err_msg(), "5");
}

fn get_expression_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/operator
        TestScriptAndResult {
            script_name: "./tests/data/operator/add_bool_nil.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at '+': Operands must be two numbers or two strings."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/add_bool_num.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at '+': Operands must be two numbers or two strings."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/add_bool_string.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at '+': Operands must be two numbers or two strings."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/add_nil_nil.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at '+': Operands must be two numbers or two strings."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/add_num_nil.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at '+': Operands must be two numbers or two strings."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/add_string_nil.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at '+': Operands must be two numbers or two strings."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/divide_nonnum_num.lox",
            expected_result: false,
            // Java and Rust implement different error messages.
            expected_output: vec!["[line 1] Error at '/': Operand must be a number."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/divide_num_nonnum.lox",
            expected_result: false,
            // Java and Rust implement different error messages.
            expected_output: vec!["[line 1] Error at '/': Operand must be a number."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/greater_nonnum_num.lox",
            expected_result: false,
            // Java and Rust implement different error messages.
            expected_output: vec!["[line 1] Error at '>': Operand must be a number."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/greater_num_nonnum.lox",
            expected_result: false,
            // Java and Rust implement different error messages.
            expected_output: vec!["[line 1] Error at '>': Operand must be a number."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/greater_or_equal_nonnum_num.lox",
            expected_result: false,
            // Java and Rust implement different error messages.
            expected_output: vec!["[line 1] Error at '>=': Operand must be a number."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/greater_or_equal_num_nonnum.lox",
            expected_result: false,
            // Java and Rust implement different error messages.
            expected_output: vec!["[line 1] Error at '>=': Operand must be a number."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/less_nonnum_num.lox",
            expected_result: false,
            // Java and Rust implement different error messages.
            expected_output: vec!["[line 1] Error at '<': Operand must be a number."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/less_num_nonnum.lox",
            expected_result: false,
            // Java and Rust implement different error messages.
            expected_output: vec!["[line 1] Error at '<': Operand must be a number."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/less_or_equal_nonnum_num.lox",
            expected_result: false,
            // Java and Rust implement different error messages.
            expected_output: vec!["[line 1] Error at '<=': Operand must be a number."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/less_or_equal_num_nonnum.lox",
            expected_result: false,
            // Java and Rust implement different error messages.
            expected_output: vec!["[line 1] Error at '<=': Operand must be a number."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/multiply_nonnum_num.lox",
            expected_result: false,
            // Java and Rust implement different error messages.
            expected_output: vec!["[line 1] Error at '*': Operand must be a number."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/multiply_num_nonnum.lox",
            expected_result: false,
            // Java and Rust implement different error messages.
            expected_output: vec!["[line 1] Error at '*': Operand must be a number."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/negate_nonnum.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at '-': Operand must be a number."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/subtract_nonnum_num.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at '-': Operand must be a number."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/subtract_num_nonnum.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at '-': Operand must be a number."],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/string
        TestScriptAndResult {
            script_name: "./tests/data/string/error_after_multiline.lox",
            expected_result: false,
            expected_output: vec!["[line 7] Error at 'err': Undefined variable 'err'."],
        },
    ]
} // cargo test test_interpreter_expr -- --exact [--nocapture]

#[test]
fn test_interpreter_expr() {
    let expression_script_results = get_expression_script_results();

    for entry in expression_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Test interpreting/evaluating.
        let mut interpreter = make_interpreter_byte_stream();
        let res = interpreter.interpret(statements);

        // assert_interpreter_expression_result(&entry, &res);
        assert_interpreter_result(&entry, &res, &interpreter);
    }    
}