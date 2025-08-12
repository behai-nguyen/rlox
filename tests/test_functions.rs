// Date Created: 27/07/2025.

//! Uses data from `./data/`.
//! 
//! Tests use [`rlox::interpreter::Interpreter`] like `test_interpreter.rs`, but 
//! focuses on the later Chapter 10 -- [Functions](https://craftinginterpreters.com/functions.html).
//! 
//! To run test for this module only: 
//! 
//!     * cargo test --test test_functions
//! 
//! To run a specific test method: 
//!
//!     * cargo test test_interpreter_function_objects_stmt -- --exact [--nocapture]
//!     * cargo test test_interpreter_return_statements_stmt -- --exact [--nocapture]
//!     * cargo test test_interpreter_local_funs_and_closures_stmt -- --exact [--nocapture]
//! 

mod test_common;
use crate::test_common::{
    make_interpreter_byte_stream,
    assert_parse_script_statements,
    TestScriptAndResult,
    TestScriptAndResults,
    assert_interpreter_result,
};

fn get_function_objects_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/call
        TestScriptAndResult {
            script_name: "./tests/data/call/bool.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at ')': Can only call functions and classes."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/call/nil.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at ')': Can only call functions and classes."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/call/num.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at ')': Can only call functions and classes."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/call/string.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at ')': Can only call functions and classes."],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/function
        TestScriptAndResult {
            script_name: "./tests/data/function/empty_body.lox",
            expected_result: true,
            expected_output: vec!["nil"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/function/extra_arguments.lox",
            expected_result: false,
            expected_output: vec!["[line 6] Error at ')': Expected 2 arguments but got 4."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/function/missing_arguments.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at ')': Expected 2 arguments but got 1."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/function/print.lox",
            expected_result: true,
            expected_output: vec!["<fn foo>", "<native fn>"],
        },                
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/operator
        TestScriptAndResult {
            script_name: "./tests/data/operator/not.lox",
            expected_result: true,
            expected_output: vec!["false", "true", "true",
                "false", "false",
                "true",
                "false",
                "false"],
        },
    ]
} // cargo test test_interpreter_function_objects_stmt -- --exact [--nocapture]

fn get_return_statements_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/return
        TestScriptAndResult {
            script_name: "./tests/data/return/after_else.lox",
            expected_result: true,
            expected_output: vec!["ok"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/return/after_if.lox",
            expected_result: true,
            expected_output: vec!["ok"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/return/after_while.lox",
            expected_result: true,
            expected_output: vec!["ok"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/return/in_function.lox",
            expected_result: true,
            expected_output: vec!["ok"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/return/return_nil_if_no_value.lox",
            expected_result: true,
            expected_output: vec!["nil"],
        },
    ]
} // cargo test test_interpreter_return_statements_stmt -- --exact [--nocapture]

fn get_local_funs_and_closures_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/closure
        // Is also tests/test_resolving_and_binding.rs
        TestScriptAndResult {
            script_name: "./tests/data/closure/unused_closure.lox",
            expected_result: true,
            expected_output: vec!["ok"],
        },
    ]
} // cargo test test_interpreter_local_funs_and_closures_stmt -- --exact [--nocapture]

#[test]
fn test_interpreter_function_objects_stmt() {
    // Ensure script is loaded, scanned and parsed successfully.
    let func_objs_script_results = get_function_objects_script_results();

    for entry in func_objs_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Test interpreting/evaluating.
        let mut interpreter = make_interpreter_byte_stream();
        let res = interpreter.interpret(&statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
}

#[test]
fn test_interpreter_return_statements_stmt() {
    // Ensure script is loaded, scanned and parsed successfully.
    let ret_stmt_script_results = get_return_statements_script_results();

    for entry in ret_stmt_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Test interpreting/evaluating.
        let mut interpreter = make_interpreter_byte_stream();
        let res = interpreter.interpret(&statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
}

#[test]
fn test_interpreter_local_funs_and_closures_stmt() {
    // Ensure script is loaded, scanned and parsed successfully.
    let closures_script_results = get_local_funs_and_closures_results();

    for entry in closures_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Test interpreting/evaluating.
        let mut interpreter = make_interpreter_byte_stream();
        let res = interpreter.interpret(&statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
}