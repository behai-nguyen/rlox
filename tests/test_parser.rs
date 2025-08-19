// Date Created: 07/07/2025.

//! Uses data from `./data/expressions/`.
//! 
//! To run test for this module only: 
//! 
//!     * cargo test --test test_parser
//! 
//! To run a specific test method: 
//! 
//!     * cargo test test_parser_expr_01 -- --exact [--nocapture]
//!     * cargo test test_parser_expr_02 -- --exact [--nocapture]
//!     * cargo test test_parser_expr_03 -- --exact [--nocapture]
//!
//!     * cargo test test_parser_generic_stmt -- --exact [--nocapture] 
//!     * cargo test test_parser_var_stmt -- --exact [--nocapture]
//!     * cargo test test_parser_assign_stmt -- --exact [--nocapture]
//!     * cargo test test_parser_conditional_execution_stmt -- --exact [--nocapture]
//!     * cargo test test_parser_while_loops_stmt -- --exact [--nocapture]
//!     * cargo test test_parser_for_loops_stmt -- --exact [--nocapture]
//!     * cargo test test_parser_function_objects_stmt -- --exact [--nocapture]
//!     * cargo test test_parser_classes_field_and_property -- --exact [--nocapture]
//!     * cargo test test_parser_classes_methods_on_classes -- --exact [--nocapture]
//!     * cargo test test_parser_classes_this -- --exact [--nocapture]
//! 

use std::rc::Rc;

mod test_common;

use crate::test_common::{
    assert_scan_script,
    TestScriptAndResult,
    TestScriptAndResults,
    assert_parser_result,
};

use rlox::token::Token;
use rlox::parser::Parser;
use rlox::ast_printer::AstPrinter;

fn make_parser(tokens: &Vec<Token>) -> Parser {
    Parser::new(tokens)
}

fn get_generic_script_results<'a>() -> TestScriptAndResults<'a> {    
    vec![
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/comments
        TestScriptAndResult {
            script_name: "./tests/data/comments/line_at_eof.lox",
            expected_result: true,
            expected_output: vec!["ok"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/comments/only_line_comment.lox",
            expected_result: true,
            expected_output: vec![],
        },
        TestScriptAndResult {
            script_name: "./tests/data/comments/only_line_comment_and_line.lox",
            expected_result: true,
            expected_output: vec![],
        },
        TestScriptAndResult {
            script_name: "./tests/data/comments/unicode.lox",
            expected_result: true,
            expected_output: vec!["ok"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/number/leading_dot.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at '.': Expect expression."],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/print
        TestScriptAndResult {
            script_name: "./tests/data/print/missing_argument.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at ';': Expect expression."],
        },
    ] 
} // cargo test test_parser_generic_stmt -- --exact

fn get_var_script_results<'a>() -> TestScriptAndResults<'a> {    
    vec![
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/variable
        TestScriptAndResult {
            script_name: "./tests/data/variable/use_false_as_var.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'false': Expect variable name."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/variable/use_nil_as_var.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'nil': Expect variable name."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/variable/use_this_as_var.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'this': Expect variable name."],
        },
    ]
} // cargo test test_parser_var_stmt -- --exact [--nocapture]

fn get_assign_script_results<'a>() -> TestScriptAndResults<'a> {
    // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/assignment
    vec![
        TestScriptAndResult { 
            script_name: "./tests/data/assignment/grouping.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at '=': Invalid assignment target."],
        }, 
        TestScriptAndResult {
            script_name: "./tests/data/assignment/infix_operator.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at '=': Invalid assignment target."],
        }, 
        TestScriptAndResult {
            script_name: "./tests/data/assignment/prefix_operator.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at '=': Invalid assignment target."],
        },
    ]
} // cargo test test_parser_assign_stmt -- --exact [--nocapture] 

// Section https://craftinginterpreters.com/control-flow.html#conditional-execution
fn get_conditional_execution_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/if
        TestScriptAndResult {
            script_name: "./tests/data/if/var_in_else.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'var': Expect expression."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/if/var_in_then.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'var': Expect expression."],
        },
    ]
} // cargo test test_parser_conditional_execution_stmt -- --exact [--nocapture]

fn get_while_loops_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/while
        TestScriptAndResult {
            script_name: "./tests/data/while/var_in_body.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'var': Expect expression."],
        },
    ]
} // cargo test test_parser_while_loops_stmt -- --exact [--nocapture]

fn get_for_loops_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/for
        TestScriptAndResult {
            script_name: "./tests/data/for/statement_condition.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at '{': Expect expression."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/for/statement_increment.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at '{': Expect expression."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/for/statement_initializer.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at '{': Expect expression."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/for/var_in_body.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'var': Expect expression."],
        },
    ]
} // cargo test test_parser_for_loops_stmt -- --exact [--nocapture]

fn get_function_objects_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/for
        TestScriptAndResult {
            script_name: "./tests/data/for/fun_in_body.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'fun': Expect expression."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/for/class_in_body.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'class': Expect expression."],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/function
        TestScriptAndResult {
            script_name: "./tests/data/function/body_must_be_block.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at '123': Expect '{' before function body."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/function/missing_comma_in_parameters.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at 'c': Expect ')' after parameters."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/function/too_many_arguments.lox",
            expected_result: false,
            expected_output: vec!["[line 260] Error at 'a': Can't have more than 255 arguments."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/function/too_many_parameters.lox",
            expected_result: false,
            expected_output: vec!["[line 257] Error at 'a': Can't have more than 255 parameters."],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/if
        TestScriptAndResult {
            script_name: "./tests/data/if/fun_in_else.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'fun': Expect expression."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/if/fun_in_then.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'fun': Expect expression."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/if/class_in_else.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'class': Expect expression."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/if/class_in_then.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'class': Expect expression."],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/while
        TestScriptAndResult {
            script_name: "./tests/data/while/fun_in_body.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'fun': Expect expression."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/while/class_in_body.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'class': Expect expression."],
        },        
    ]
} // cargo test test_parser_function_objects_stmt -- --exact [--nocapture]

// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#properties-on-instances
fn get_classes_field_and_property_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/number
        TestScriptAndResult {
            script_name: "./tests/data/number/decimal_point_at_eof.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at '': Expect property name after '.'."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/number/trailing_dot.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at ';': Expect property name after '.'."],
        },
    ]
} // cargo test test_parser_classes_field_and_property -- --exact [--nocapture]

// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#methods-on-classes
fn get_classes_methods_on_classes_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/method
        TestScriptAndResult {
            script_name: "./tests/data/method/too_many_arguments.lox",
            expected_result: false,
            expected_output: vec!["[line 259] Error at 'a': Can't have more than 255 arguments."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/method/too_many_parameters.lox",
            expected_result: false,
            expected_output: vec!["[line 258] Error at 'a': Can't have more than 255 parameters."],
        },
    ]
} // cargo test test_parser_classes_methods_on_classes -- --exact [--nocapture]

// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#this
fn get_classes_this_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/assignment
        TestScriptAndResult {
            script_name: "./tests/data/assignment/to_this.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at '=': Invalid assignment target."],
        },
    ]
} // cargo test test_parser_classes_this -- --exact [--nocapture]

#[test]
// The test script is from 
//     https://github.com/munificent/craftinginterpreters/blob/master/test/expressions/parse.lox
fn test_parser_expr_01() {
    let tokens = assert_scan_script("./tests/data/expressions/parse.lox");

    // Parsing test.
    let mut parser = make_parser(&tokens);
    let res = parser.parse_single_expression();

    assert_eq!(res.is_err(), false);

    let expr = res.unwrap();

    // Note: the scanner normalises numeric (f64) literals, and printer uses 
    // {:?} to print out f64 value, see method visit_literal_expr().
    assert_eq!("(+ (group (- 5.0 (group (- 3.0 1.0)))) (- 1.0))", AstPrinter{}.print_expression(expr).unwrap());
}

#[test]
// See also ./src/ast_printer.rs tests::it_works().
// The tested expression (and expected output) is from the the section 
//     https://craftinginterpreters.com/representing-code.html#a-not-very-pretty-printer
fn test_parser_expr_02() {
    let tokens = assert_scan_script("./tests/data/expressions/parse-02.lox");

    // Parsing test.
    let mut parser = make_parser(&tokens);
    let res = parser.parse_single_expression();

    assert_eq!(res.is_err(), false);

    let expr = res.unwrap();

    // Note: the scanner normalises numeric (f64) literals, and printer uses 
    // {:?} to print out f64 value, see method visit_literal_expr().

    //
    // In the stated section, the author does not state what the natural arithmetic 
    // expression is. The author expected this output. However, I am unable to get 
    // this: Might be my interpreted input expression is not what the author had in
    // mind?
    // 
    assert_ne!("(* (- 123.0) (group 45.67))", AstPrinter{}.print_expression(Rc::clone(&expr)).unwrap());

    //
    // It get this:
    //
    assert_eq!("(group (* (- 123.0) 45.67))", AstPrinter{}.print_expression(expr).unwrap());
}

#[test]
// See also ./src/ast_printer.rs tests::it_works().
// The tested expression (and expected output) is from the the section 
//     https://craftinginterpreters.com/representing-code.html#a-not-very-pretty-printer
fn test_parser_expr_03() {
    let tokens = assert_scan_script("./tests/data/expressions/parse-03.lox");

    // Parsing test.
    let mut parser = make_parser(&tokens);
    let res = parser.parse_single_expression();

    assert_eq!(res.is_err(), false);

    let expr = res.unwrap();

    // Note: the scanner normalises numeric (f64) literals, and printer uses 
    // {:?} to print out f64 value, see method visit_literal_expr().

    //
    // In the stated section, the author does not state what the natural arithmetic 
    // expression is. The author expected this output. 
    // 
    assert_eq!("(* (- 123.0) (group 45.67))", AstPrinter{}.print_expression(expr).unwrap());
}

#[test]
fn test_parser_generic_stmt() {
    let generic_script_results = get_generic_script_results();

    for entry in generic_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let tokens = assert_scan_script(entry.script_name);

        // Parsing test.
        let mut parser = make_parser(&tokens);
        let res = parser.parse();

        assert_parser_result(&entry, &res);
   }    
}

#[test]
fn test_parser_var_stmt() {
    let var_script_results = get_var_script_results();

    for entry in var_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let tokens = assert_scan_script(entry.script_name);

        // Parsing test.
        let mut parser = make_parser(&tokens);
        let res = parser.parse();

        assert_parser_result(&entry, &res);
   }
}

#[test]
fn test_parser_assign_stmt() {
    let var_script_results = get_assign_script_results();

    for entry in var_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let tokens = assert_scan_script(entry.script_name);

        // Parsing test.
        let mut parser = make_parser(&tokens);
        let res = parser.parse();

        assert_parser_result(&entry, &res);
   }
}

#[test]
fn test_parser_conditional_execution_stmt() {
    let cond_exec_script_results = get_conditional_execution_script_results();

    for entry in cond_exec_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let tokens = assert_scan_script(entry.script_name);

        // Parsing test.
        let mut parser = make_parser(&tokens);
        let res = parser.parse();

        assert_parser_result(&entry, &res);
   }
}

#[test]
fn test_parser_while_loops_stmt() {
    let while_loops_script_results = get_while_loops_script_results();

    for entry in while_loops_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let tokens = assert_scan_script(entry.script_name);

        // Parsing test.
        let mut parser = make_parser(&tokens);
        let res = parser.parse();

        assert_parser_result(&entry, &res);
   }
}

#[test]
fn test_parser_for_loops_stmt() {
    let for_loops_script_results = get_for_loops_script_results();

    for entry in for_loops_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let tokens = assert_scan_script(entry.script_name);

        // Parsing test.
        let mut parser = make_parser(&tokens);
        let res = parser.parse();

        assert_parser_result(&entry, &res);
   }
}

#[test]
fn test_parser_function_objects_stmt() {
    let func_objs_script_results = get_function_objects_script_results();

    for entry in func_objs_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let tokens = assert_scan_script(entry.script_name);

        // Parsing test.
        let mut parser = make_parser(&tokens);
        let res = parser.parse();

        assert_parser_result(&entry, &res);
   }
}

#[test]
// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#properties-on-instances
fn test_parser_classes_field_and_property() {
    let cls_fld_and_ppt_script_results = get_classes_field_and_property_script_results();

    for entry in cls_fld_and_ppt_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let tokens = assert_scan_script(entry.script_name);

        // Parsing test.
        let mut parser = make_parser(&tokens);
        let res = parser.parse();

        assert_parser_result(&entry, &res);
   }
}

#[test]
// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#methods-on-classes
fn test_parser_classes_methods_on_classes() {
    let cls_mth_on_cls_script_results = get_classes_methods_on_classes_script_results();

    for entry in cls_mth_on_cls_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let tokens = assert_scan_script(entry.script_name);

        // Parsing test.
        let mut parser = make_parser(&tokens);
        let res = parser.parse();

        assert_parser_result(&entry, &res);
   }
}

#[test]
// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#this
fn test_parser_classes_this() {
    let classes_this_script_results = get_classes_this_script_results();

    for entry in classes_this_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let tokens = assert_scan_script(entry.script_name);

        // Parsing test.
        let mut parser = make_parser(&tokens);
        let res = parser.parse();

        assert_parser_result(&entry, &res);
   }
}
