/* Date Created: 05/06/2025. */

// Requires by ../tests/ modules, so that use declarations such 
// as "use rlox::scanner" is possible.

pub mod lox_error;
pub mod lox_error_helper;
pub mod scanner_index;
pub mod token_type;
pub mod token;
pub mod lox_callable;
pub mod value;
pub mod lox_clock;
pub mod scanner;
pub mod expr;
pub mod stmt;
pub mod parser;
pub mod ast_printer;
pub mod environment;
pub mod interpreter;
pub mod lox_function;
pub mod lox_return;
pub mod lox_runtime_error;
pub mod resolver;
pub mod lox_class;
pub mod lox_instance;

#[macro_export]
macro_rules! unwrap_expr {
    ($expr:expr, $variant:ident) => {
        match $expr.as_ref() {
            expr::Expr::$variant(inner) => inner,
            _ => unreachable!(concat!("Expected Expr::", stringify!($variant))),
        }
    };
}

#[macro_export]
macro_rules! unwrap_stmt {
    ($stmt:expr, $variant:ident) => {
        match $stmt.as_ref() {
            stmt::Stmt::$variant(inner) => inner,
            _ => unreachable!(concat!("Expected Stmt::", stringify!($variant))),
        }
    };
}
