// Date Created: 07/07/2025.

//! Uses data from `./data/expressions/`.
//! 
//! To run test for this module only: 
//! 
//!     * cargo test --test test_parser
//! 
//! To run a specific test method: 
//! 
//!     * cargo test parse_expr_01 -- --exact [--nocapture]
//!     * cargo test parse_expr_02 -- --exact [--nocapture]
//!     * cargo test parse_expr_03 -- --exact [--nocapture]

mod test_common;

use crate::test_common::assert_scan_script;

use rlox::parser::Parser;
use rlox::ast_printer::AstPrinter;

#[test]
// The test script is from 
//     https://github.com/munificent/craftinginterpreters/blob/master/test/expressions/parse.lox
fn parse_expr_01() {
    let tokens = assert_scan_script("./tests/data/expressions/parse.lox");

    // Parsing test.
    let parser = Parser::new(&tokens);
    let res = parser.parse();

    assert_eq!(res.is_err(), false);

    let expr = res.unwrap();

    // Note: the scanner normalises numeric (f64) literals, and printer uses 
    // {:?} to print out f64 value, see method visit_literal_expr().
    assert_eq!("(+ (group (- 5.0 (group (- 3.0 1.0)))) (- 1.0))", AstPrinter{}.print_expression(&expr).unwrap());
}

#[test]
// See also ./src/ast_printer.rs tests::it_works().
// The tested expression (and expected output) is from the the section 
//     https://craftinginterpreters.com/representing-code.html#a-not-very-pretty-printer
fn parse_expr_02() {
    let tokens = assert_scan_script("./tests/data/expressions/parse-02.lox");

    // Parsing test.
    let parser = Parser::new(&tokens);
    let res = parser.parse();

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
    assert_ne!("(* (- 123.0) (group 45.67))", AstPrinter{}.print_expression(&expr).unwrap());

    //
    // It get this:
    //
    assert_eq!("(group (* (- 123.0) 45.67))", AstPrinter{}.print_expression(&expr).unwrap());
}

#[test]
// See also ./src/ast_printer.rs tests::it_works().
// The tested expression (and expected output) is from the the section 
//     https://craftinginterpreters.com/representing-code.html#a-not-very-pretty-printer
fn parse_expr_03() {
    let tokens = assert_scan_script("./tests/data/expressions/parse-03.lox");

    // Parsing test.
    let parser = Parser::new(&tokens);
    let res = parser.parse();

    assert_eq!(res.is_err(), false);

    let expr = res.unwrap();

    // Note: the scanner normalises numeric (f64) literals, and printer uses 
    // {:?} to print out f64 value, see method visit_literal_expr().

    //
    // In the stated section, the author does not state what the natural arithmetic 
    // expression is. The author expected this output. 
    // 
    assert_eq!("(* (- 123.0) (group 45.67))", AstPrinter{}.print_expression(&expr).unwrap());
}
