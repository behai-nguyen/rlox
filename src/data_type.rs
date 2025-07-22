/* Date Created: 15/07/2025. */

//! Lox supported data types. This is Rust-specific, this module is not in the 
//! original Java version.

// Rust-specific.
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub type Value = DataType;