// Date Created: 08/08/2025.

//! Uses data from `./data/`.

//! Tests for [`rlox::resolver::Resolver`] (src/resolver.rs), Chapter 11 [Resolving 
//! and Binding](https://craftinginterpreters.com/resolving-and-binding.html).
//! 
//! To run test for this module only: 
//! 
//!     * cargo test --test test_resolving_and_binding
//! 
//! To run a specific test method: 
//!
//!     * cargo test test_resolver_error -- --exact [--nocapture]
//!     * cargo test test_resolver_interpreter -- --exact [--nocapture]
//! 

mod test_common;
use rlox::resolver::Resolver;
use crate::test_common::{
    make_interpreter_byte_stream,
    assert_parse_script_statements,
    TestScriptAndResult,
    TestScriptAndResults,
    assert_resolver_result,
    assert_interpreter_result,
};

fn get_error_script_results<'a>() -> TestScriptAndResults<'a> {    
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/return
        TestScriptAndResult {
            script_name: "./tests/data/return/at_top_level.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at 'return': Can't return from top-level code."],
        },
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/variable
        TestScriptAndResult {
            script_name: "./tests/data/variable/collide_with_parameter.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'a': Already a variable with this name in this scope."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/variable/duplicate_local.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at 'a': Already a variable with this name in this scope."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/variable/duplicate_parameter.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'arg': Already a variable with this name in this scope."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/variable/use_local_in_initializer.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at 'a': Can't read local variable in its own initializer."],
        },
    ]
} // cargo test test_resolver_error -- --exact [--nocapture]

fn get_resolver_interpreter_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/assignment
        TestScriptAndResult {
            script_name: "./tests/data/assignment/local.lox",
            expected_result: true,
            expected_output: vec!["before", "after", "arg", "arg"],
        },
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
            script_name: "./tests/data/variable/early_bound.lox",
            expected_result: true,
            expected_output: vec!["outer", "outer"],
        },
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/block
        TestScriptAndResult {
            script_name: "./tests/data/block/scope.lox",
            expected_result: true,
            expected_output: vec!["inner", "outer"],
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
        TestScriptAndResult {
            script_name: "./tests/data/function/local_recursion.lox",
            expected_result: true,
            // Normalise f64.
            expected_output: vec!["21.0"],
        },
        // It is here because it is an evaluation error, which means the Interpreter 
        // could not evaluate it.
        TestScriptAndResult {
            script_name: "./tests/data/function/local_mutual_recursion.lox",
            expected_result: false,
            expected_output: vec!["[line 4] Error at 'isOdd': Undefined variable 'isOdd'."],
        },        
        // Author's https://craftinginterpreters.com/functions.html#local-functions-and-closures
        TestScriptAndResult {
            script_name: "./tests/data/function/book_make_counter.lox",
            expected_result: true,
            expected_output: vec!["1.0", "2.0"],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/while
        TestScriptAndResult {
            script_name: "./tests/data/while/return_inside.lox",
            expected_result: true,
            expected_output: vec!["i"],
        },
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
            script_name: "./tests/data/closure/unused_later_closure.lox",
            expected_result: true,
            expected_output: vec!["a"],
        },
        // Also in tests/test_functions.rs.
        TestScriptAndResult {
            script_name: "./tests/data/closure/unused_closure.lox",
            expected_result: true,
            expected_output: vec!["ok"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/closure/assign_to_shadowed_later.lox",
            expected_result: true,
            expected_output: vec!["inner", "assigned"],
        },
        // Author's https://craftinginterpreters.com/resolving-and-binding.html#static-scope
        TestScriptAndResult {
            script_name: "./tests/data/closure/book_fun_in_closure.lox",
            expected_result: true,
            expected_output: vec!["global", "global"],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/regression
        TestScriptAndResult {
            script_name: "./tests/data/regression/40.lox",
            expected_result: true,
            expected_output: vec!["false"],
        },
    ]
} // cargo test test_resolver_interpreter -- --exact [--nocapture]

#[test]
fn test_resolver_error() {
    let error_script_results = get_error_script_results();

    // Resolver needs an mutable Interpreter instance.
    let mut interpreter = make_interpreter_byte_stream();
    // Resolver instance.
    let mut resolver: Resolver = Resolver::new(&mut interpreter);

    for entry in error_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Resolver test.
        let res = resolver.resolve(&statements);

        assert_resolver_result(&entry, &res);
    }    
}

#[test]
fn test_resolver_interpreter() {
    let rsv_itpt_script_results = get_resolver_interpreter_script_results();

    // Resolver needs an mutable Interpreter instance.
    let mut interpreter = make_interpreter_byte_stream();

    for entry in rsv_itpt_script_results {
        interpreter.reset(true);

        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Create a resolver instance for each script file.
        let mut resolver: Resolver = Resolver::new(&mut interpreter);

        // Resolver test.
        let res = resolver.resolve(&statements);

        // Ensure resolving is successful.
        assert!(res.is_ok(), "method() resolve error: {}", entry.script_name);

        // Test interpreting/evaluating.
        let res = interpreter.interpret(&statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }    

}