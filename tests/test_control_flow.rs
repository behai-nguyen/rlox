// Date Created: 22/07/2025.

//! Uses data from `./data/`.
//! 
//! Tests use [`rlox::interpreter::Interpreter`] like `test_interpreter.rs`, but 
//! focuses on the later Chapter 9 -- [Control Flow](https://craftinginterpreters.com/control-flow.html).
//! 
//! To run test for this module only: 
//! 
//!     * cargo test --test test_control_flow
//! 
//! To run a specific test method: 
//! 
//!     * cargo test test_interpreter_conditional_execution_stmt -- --exact [--nocapture]
//!     * cargo test test_interpreter_logical_operators_stmt -- --exact [--nocapture]
//!     * cargo test test_interpreter_while_loops_stmt -- --exact [--nocapture]
//!     * cargo test test_interpreter_for_loops_stmt -- --exact [--nocapture]
//! 

mod test_common;
use crate::test_common::{
    make_interpreter_byte_stream,
    assert_parse_script_statements,
    TestScriptAndResult,
    TestScriptAndResults,
    assert_interpreter_result,
};

// Section https://craftinginterpreters.com/control-flow.html#conditional-execution
fn get_conditional_execution_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/block
        TestScriptAndResult {
            script_name: "./tests/data/block/empty.lox",
            expected_result: true,
            expected_output: vec!["ok"],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/variable
        TestScriptAndResult {
            script_name: "./tests/data/variable/unreached_undefined.lox",
            expected_result: true,
            expected_output: vec!["ok"],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/if
        TestScriptAndResult {
            script_name: "./tests/data/if/dangling_else.lox",
            expected_result: true,
            expected_output: vec!["good"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/if/else.lox",
            expected_result: true,
            expected_output: vec!["good", "good", "block"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/if/if.lox",
            expected_result: true,
            expected_output: vec!["good", "block", "true"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/if/truth.lox",
            expected_result: true,
            // Normalising f64.
            expected_output: vec!["false", "nil", "true", "0.0", "empty"],
        },
    ]
} // cargo test test_interpreter_conditional_execution_stmt -- --exact [--nocapture]

fn get_logical_operators_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/logical_operator
        TestScriptAndResult {
            script_name: "./tests/data/logical_operator/and.lox",
            expected_result: true,
            // Normalises f64.
            expected_output: vec!["false", "1.0", "false", 
                "true", "3.0", 
                "true", "false"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/logical_operator/and_truth.lox",
            expected_result: true,
            expected_output: vec!["false", "nil", 
                "ok", "ok", "ok"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/logical_operator/or.lox",
            expected_result: true,
            // Normalises f64.
            expected_output: vec!["1.0", "1.0", "true", 
                "false", "false", 
                "false", "true"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/logical_operator/or_truth.lox",
            expected_result: true,
            // Normalises f64.
            expected_output: vec!["ok", "ok", 
                "true", "0.0", "s"],
        },
        // Author's https://craftinginterpreters.com/control-flow.html#logical-operators
        TestScriptAndResult {
            script_name: "./tests/data/logical_operator/book_end_section.lox",
            expected_result: true,
            expected_output: vec!["hi", "yes"],
        },
        // Mine.
        TestScriptAndResult {
            script_name: "./tests/data/logical_operator/edge_cases.lox",
            expected_result: true,
            expected_output: vec!["false", "true", "text", "true"],
        },                
    ]
} // cargo test test_interpreter_logical_operators_stmt -- --exact [--nocapture]

fn get_while_loops_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // Copied from author's https://github.com/munificent/craftinginterpreters/tree/master/test/while/syntax.lox
        // -- removed last line.
        TestScriptAndResult {
            script_name: "./tests/data/while/syntax-02.lox",
            expected_result: true,
            // Normalises f64.
            expected_output: vec!["1.0", "2.0", "3.0", 
                "0.0", "1.0", "2.0"],
        },        
	    // Mine.
        TestScriptAndResult {
            script_name: "./tests/data/while/while-01.lox",
            expected_result: true,
            // Normalises f64.
            expected_output: vec!["1.0", "2.0", "3.0", "4.0", "5.0"],
        },
    ]
} // cargo test test_interpreter_while_loops_stmt -- --exact [--nocapture]

fn get_for_loops_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/while/syntax.lox
        TestScriptAndResult {
            script_name: "./tests/data/while/syntax.lox",
            expected_result: true,
            // Normalises f64.
            expected_output: vec!["1.0", "2.0", "3.0", 
                "0.0", "1.0", "2.0"],
        },        
	    // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/for
        TestScriptAndResult {
            script_name: "./tests/data/for/scope.lox",
            expected_result: true,
            // Normalises f64.
            expected_output: vec!["0.0", "-1.0", 
                "after",
                "0.0"],
        },
	    // Author's end section https://craftinginterpreters.com/control-flow.html#for-loops
        TestScriptAndResult {
            script_name: "./tests/data/for/book_end_section.lox",
            expected_result: true,
            // Normalises f64.
            expected_output: vec!["0.0", "1.0", "1.0", "2.0", "3.0", 
                "5.0", "8.0", "13.0", "21.0", "34.0", "55.0", "89.0", 
                "144.0", "233.0", "377.0", "610.0", "987.0", 
                "1597.0", "2584.0", "4181.0", "6765.0"],
        },
        // Mine.
        TestScriptAndResult {
            script_name: "./tests/data/for/for-01.lox",
            expected_result: true,
            // Normalises f64.
            expected_output: vec!["0.0", "0.0", "0.0", "1.0", "0.0", "2.0"],
        },        
    ]
} // cargo test test_interpreter_for_loops_stmt -- --exact [--nocapture]

#[test]
fn test_interpreter_conditional_execution_stmt() {
    // Ensure script is loaded, scanned and parsed successfully.
    let cond_exec_script_results = get_conditional_execution_script_results();

    for entry in cond_exec_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Test interpreting/evaluating.
        let mut interpreter = make_interpreter_byte_stream();
        let res = interpreter.interpret(statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
} 

#[test]
fn test_interpreter_logical_operators_stmt() {
    // Ensure script is loaded, scanned and parsed successfully.
    let logical_op_script_results = get_logical_operators_script_results();

    for entry in logical_op_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Test interpreting/evaluating.
        let mut interpreter = make_interpreter_byte_stream();
        let res = interpreter.interpret(statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
}

#[test]
fn test_interpreter_while_loops_stmt() {
    // Ensure script is loaded, scanned and parsed successfully.
    let while_loops_script_results = get_while_loops_script_results();

    for entry in while_loops_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Test interpreting/evaluating.
        let mut interpreter = make_interpreter_byte_stream();
        let res = interpreter.interpret(statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
}

#[test]
fn test_interpreter_for_loops_stmt() {
    // Ensure script is loaded, scanned and parsed successfully.
    let for_loops_script_results = get_for_loops_script_results();

    for entry in for_loops_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Test interpreting/evaluating.
        let mut interpreter = make_interpreter_byte_stream();
        let res = interpreter.interpret(statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
}