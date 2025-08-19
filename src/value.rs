/* Date Created: 15/07/2025. */

//! Lox supported data types. This is Rust-specific, this module is not in the 
//! original Java version.

use super::lox_callable::LoxCallable;


// Rust-specific.
// Rust doesn’t support deriving Clone, PartialEq automatically for trait objects, 
// which Box<dyn LoxCallable> is one.
#[derive(Debug)]
pub enum DataType {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    LoxCallable(Box<dyn LoxCallable>),
}

pub type Value = DataType;

// Box<dyn LoxCallable> require manual implementations because trait objects don’t 
// support Clone or PartialEq by default.
impl Clone for DataType {
    fn clone(&self) -> Self {
        match self {
            DataType::Number(n) => DataType::Number(*n),
            DataType::String(s) => DataType::String(s.clone()),
            DataType::Boolean(b) => DataType::Boolean(*b),
            DataType::Nil => DataType::Nil,
            DataType::LoxCallable(callable) => DataType::LoxCallable(callable.clone_box()),
        }
    }
}

// Box<dyn LoxCallable> require manual implementations because trait objects don’t 
// support Clone or PartialEq by default.
impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DataType::Number(a), DataType::Number(b)) => a == b,
            (DataType::String(a), DataType::String(b)) => a == b,
            (DataType::Boolean(a), DataType::Boolean(b)) => a == b,
            (DataType::Nil, DataType::Nil) => true,
            (DataType::LoxCallable(a), DataType::LoxCallable(b)) => a == b,
            _ => false,
        }
    }
}
