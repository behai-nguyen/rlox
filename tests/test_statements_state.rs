// Date Created: 15/07/2025.

//! Uses data from `./data/`.
//! 
//! Tests use [`rlox::interpreter::Interpreter`] like `test_interpreter.rs`, but 
//! focuses on the later Chapter 8 -- [Statements and State](https://craftinginterpreters.com/statements-and-state.html).
//! 
//! To run test for this module only: 
//! 
//!     * cargo test --test test_statements_state
//! 
//! To run a specific test method: 
//! 
//!     * cargo test test_interpreter_print_stmt -- --exact [--nocapture]
//!     * cargo test test_interpreter_var_stmt -- --exact [--nocapture]
//!     * cargo test test_interpreter_assign_stmt -- --exact [--nocapture]
//!     * cargo test test_interpreter_block_stmt -- --exact [--nocapture]
//! 

use std::io::Cursor;

mod test_common;
use crate::test_common::{
    make_interpreter,
    assert_parse_script_statements,
    TestScriptAndResult,
    TestScriptAndResults,
    assert_interpreter_result,
};

fn get_print_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        TestScriptAndResult {
            script_name: "./tests/data/statements_state/lox-01.lox",
            expected_result: true,
            expected_output: vec!["one", "true", "3.0"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/statements_state/lox-02.lox",
            expected_result: true,
            expected_output: vec!["Mắt trừng gửi mộng qua biên giới", 
			        "Đêm mơ Hà Nội dáng kiều thơm"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/statements_state/lox-03.lox",
            expected_result: true,
            expected_output: vec!["espresso"],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/nil
        TestScriptAndResult {
            script_name: "./tests/data/nil/literal.lox",
            expected_result: true,
            expected_output: vec!["nil"],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/number
        TestScriptAndResult {
            script_name: "./tests/data/number/literals.lox",
            expected_result: true,
            // Note: scanner normalises f64.
            expected_output: vec!["123.0", "987654.0", "0.0", "-0.0", "123.456", "-0.001"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/number/nan_equality.lox",
            expected_result: true,
            expected_output: vec!["false", "true", "false", "true"],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/operator
        TestScriptAndResult {
            script_name: "./tests/data/operator/add.lox",
            expected_result: true,
            // Note: scanner normalises f64.
            expected_output: vec!["579.0", "string"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/comparison.lox",
            expected_result: true,
            expected_output: vec!["true", "false", "false", 
                "true", "true", "false", 
                "false", "false", "true", 
                "false", "true", "true", 
                "false", "false", "false", "false", "true", "true", "true", "true"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/divide.lox",
            expected_result: true,
            // Note: scanner normalises f64.
            expected_output: vec!["4.0", "1.0"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/equals.lox",
            expected_result: true,
            expected_output: vec!["true", 
                "true", "false", 
                "true", "false", 
                "true", "false", 
                "false", "false", "false"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/multiply.lox",
            expected_result: true,
            // Note: scanner normalises f64.
            expected_output: vec!["15.0", "3.702"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/negate.lox",
            expected_result: true,
            // Note: scanner normalises f64.
            expected_output: vec!["-3.0", "3.0", "-3.0"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/not_equals.lox",
            expected_result: true,
            expected_output: vec!["false", 
                "false", "true", 
                "false", "true", 
                "false", "true", 
                "true", "true", "true"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/subtract.lox",
            expected_result: true,
            // Note: scanner normalises f64.
            expected_output: vec!["1.0", "0.0"],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/string
        TestScriptAndResult {
            script_name: "./tests/data/string/literals.lox",
            expected_result: true,
            expected_output: vec!["()", "a string", "A~¶Þॐஃ"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/string/multiline.lox",
            expected_result: true,
            expected_output: vec!["1", "2", "3"],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/bool
        TestScriptAndResult {
            script_name: "./tests/data/bool/equality.lox",
            expected_result: true,
            expected_output: vec!["true", "false", "false", "true", 
                "false", "false", "false", "false", "false", 
                "false", "true", "true", "false", 
                "true", "true", "true", "true", "true"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/bool/not.lox",
            expected_result: true,
            // Note: scanner normalises f64.
            expected_output: vec!["false", "true", "true"],
        },
    ]
} // cargo test test_interpreter_print_stmt -- --exact

fn get_var_script_results<'a>() -> TestScriptAndResults<'a> {
    // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/variable
    vec![
        TestScriptAndResult {
            script_name: "./tests/data/variable/redeclare_global.lox",
            expected_result: true,
            expected_output: vec!["nil"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/variable/redefine_global.lox",
            expected_result: true,
            expected_output: vec!["2"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/variable/undefined_global.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at 'notDefined': Undefined variable 'notDefined'."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/variable/uninitialized.lox",
            expected_result: true,
            expected_output: vec!["nil"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/variable/use_global_in_initializer.lox",
            expected_result: true,
            expected_output: vec!["value"],
        },
    ]
} // cargo test test_interpreter_var_stmt -- --exact

fn get_assign_script_results<'a>() -> TestScriptAndResults<'a> {
    // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/assignment
    vec![
        TestScriptAndResult {
            script_name: "./tests/data/assignment/associativity.lox",
            expected_result: true,
            expected_output: vec!["c", "c", "c"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/assignment/global.lox",
            expected_result: true,
            expected_output: vec!["before", "after", "arg", "arg"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/assignment/local.lox",
            expected_result: true,
            expected_output: vec!["before", "after", "arg", "arg"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/assignment/syntax.lox",
            expected_result: true,
            expected_output: vec!["var", "var"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/assignment/undefined.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at 'unknown': Undefined variable 'unknown'."],
        },
    ]
} // cargo test test_interpreter_assign_stmt -- --exact [--nocapture]

fn get_block_script_results<'a>() -> TestScriptAndResults<'a> {    
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/variable
        TestScriptAndResult {
            script_name: "./tests/data/variable/in_middle_of_block.lox",
            expected_result: true,
            expected_output: vec!["a", "a b", "a c", "a b d"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/variable/in_nested_block.lox",
            expected_result: true,
            expected_output: vec!["outer"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/variable/scope_reuse_in_different_blocks.lox",
            expected_result: true,
            expected_output: vec!["first", "second"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/variable/shadow_and_local.lox",
            expected_result: true,
            expected_output: vec!["outer", "inner"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/variable/shadow_global.lox",
            expected_result: true,
            expected_output: vec!["shadow", "global"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/variable/shadow_local.lox",
            expected_result: true,
            expected_output: vec!["shadow", "local"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/variable/undefined_local.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'notDefined': Undefined variable 'notDefined'."],
        },
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/block
        TestScriptAndResult {
            script_name: "./tests/data/block/scope.lox",
            expected_result: true,
            expected_output: vec!["inner", "outer"],
        },        
    ]
} // cargo test test_interpreter_block_stmt -- --exact [--nocapture]

#[test]
fn test_interpreter_print_stmt() {
    // Ensure script is loaded, scanned and parsed successfully.
    let print_script_results = get_print_script_results();
    
    for entry in print_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Test interpreting/evaluating.
        let mut interpreter = make_interpreter(Cursor::new(Vec::new()));
        let res = interpreter.interpret(statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
} 

#[test]
fn test_interpreter_var_stmt() {
    let var_script_results = get_var_script_results();

    for entry in var_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Test interpreting/evaluating.
        let mut interpreter = make_interpreter(Cursor::new(Vec::new()));
        let res = interpreter.interpret(statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
}

#[test]
fn test_interpreter_assign_stmt() {
    let assign_script_results = get_assign_script_results();

    for entry in assign_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Test interpreting/evaluating.
        let mut interpreter = make_interpreter(Cursor::new(Vec::new()));
        let res = interpreter.interpret(statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
}

#[test]
fn test_interpreter_block_stmt() {
    let assign_script_results = get_block_script_results();

    for entry in assign_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Test interpreting/evaluating.
        let mut interpreter = make_interpreter(Cursor::new(Vec::new()));
        let res = interpreter.interpret(statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
}