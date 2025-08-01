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
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/for
        TestScriptAndResult {
            script_name: "./tests/data/for/return_inside.lox",
            expected_result: true,
            expected_output: vec!["i"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/for/syntax.lox",
            expected_result: true,
            expected_output: vec!["1.0", "2.0", "3.0", "0.0", "1.0", "2.0", 
                "done", "0.0", "1.0", "0.0", "1.0", "2.0", "0.0", "1.0"],
        },        
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/function
        TestScriptAndResult {
            script_name: "./tests/data/function/mutual_recursion.lox",
            expected_result: true,
            expected_output: vec!["true", "true"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/function/nested_call_with_arguments.lox",
            expected_result: true,
            expected_output: vec!["hello world"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/function/parameters.lox",
            expected_result: true,
            // Normalise f64.
            expected_output: vec!["0.0", "1.0", "3.0", "6.0", "10.0", 
                "15.0", "21.0", "28.0", "36.0"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/function/recursion.lox",
            expected_result: true,
            // Normalise f64.
            expected_output: vec!["21.0"],
        },        
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/while
        TestScriptAndResult {
            script_name: "./tests/data/while/return_inside.lox",
            expected_result: true,
            expected_output: vec!["i"],
        },
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
        TestScriptAndResult {
            script_name: "./tests/data/closure/assign_to_closure.lox",
            expected_result: true,
            expected_output: vec!["local", "after f", "after f", "after g"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/closure/closed_closure_in_function.lox",
            expected_result: true,
            expected_output: vec!["local"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/closure/close_over_function_parameter.lox",
            expected_result: true,
            expected_output: vec!["param"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/closure/close_over_later_variable.lox",
            expected_result: true,
            expected_output: vec!["b", "a"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/closure/nested_closure.lox",
            expected_result: true,
            expected_output: vec!["a", "b", "c"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/closure/open_closure_in_function.lox",
            expected_result: true,
            expected_output: vec!["local"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/closure/reference_closure_multiple_times.lox",
            expected_result: true,
            expected_output: vec!["a", "a"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/closure/reuse_closure_slot.lox",
            expected_result: true,
            expected_output: vec!["a"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/closure/shadow_closure_with_local.lox",
            expected_result: true,
            expected_output: vec!["closure", "shadow", "closure"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/closure/unused_closure.lox",
            expected_result: true,
            expected_output: vec!["ok"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/closure/unused_later_closure.lox",
            expected_result: true,
            expected_output: vec!["a"],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/for
        TestScriptAndResult {
            script_name: "./tests/data/for/closure_in_body.lox",
            expected_result: true,
            // Normalise f64.
            expected_output: vec!["4.0", "1.0", "4.0", "2.0", "4.0", "3.0"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/for/return_closure.lox",
            expected_result: true,
            expected_output: vec!["i"],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/function
        TestScriptAndResult {
            script_name: "./tests/data/function/local_recursion.lox",
            expected_result: true,
            // Normalise f64.
            expected_output: vec!["21.0"],
        },
        // Author's https://craftinginterpreters.com/functions.html#local-functions-and-closures
        TestScriptAndResult {
            script_name: "./tests/data/function/book_make_counter.lox",
            expected_result: true,
            expected_output: vec!["1.0", "2.0"],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/while
        TestScriptAndResult {
            script_name: "./tests/data/while/closure_in_body.lox",
            expected_result: true,
            // Normalise f64.
            expected_output: vec!["1.0", "2.0", "3.0"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/while/return_closure.lox",
            expected_result: true,
            expected_output: vec!["i"],
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
        let res = interpreter.interpret(statements);

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
        let res = interpreter.interpret(statements);

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
        let res = interpreter.interpret(statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
}