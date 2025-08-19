/* Date Created: 15/07/2025. */

//! Lox supported data types. This is Rust-specific, this module is not in the 
//! original Java version.

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

use super::lox_callable::LoxCallable;
use super::lox_instance::LoxInstance;

// Rust-specific.

// To run test for this module only: 
// 
//     * cargo test value::tests
//
//     * cargo test value::tests::value_comparison_and_clone -- --exact [--nocapture]
//


// Rust doesn’t support deriving PartialEq automatically for trait objects, 
// which Rc<dyn LoxCallable> is one.
#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    LoxCallable(Rc<dyn LoxCallable>),
    LoxInstance(Rc<RefCell<LoxInstance>>),
}

pub type ValueMap = HashMap<String, Value>;

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "Number({})", n),
            Value::String(s) => write!(f, "String({})", s),
            Value::Boolean(b) => write!(f, "Boolean({})", b),
            Value::Nil => write!(f, "Nil"),
            Value::LoxInstance(_) => write!(f, "LoxInstance"),
            Value::LoxCallable(_) => write!(f, "LoxCallable"),
        }
    }
}

// Rc<dyn LoxCallable> require manual implementations because trait objects don’t 
// support PartialEq by default.
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            // Compare the raw pointer addresses.
            (Value::LoxCallable(a), Value::LoxCallable(b)) =>
                std::ptr::eq(a.as_ref(), b.as_ref()),
            (Value::LoxInstance(i1), Value::LoxInstance(i2)) => i1 == i2,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::LoxCallable(callable) => write!(f, "{}", callable),
            Value::LoxInstance(instance) => write!(f, "{}", instance.borrow().to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_comparison_and_clone() {
        let a = Value::Number(42.0);
        let b = a.clone();
        assert_eq!(a, b);
    }
}