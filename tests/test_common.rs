// Date Created: 05/06/2025.

use std::fs::read_to_string;

use rlox::token::LiteralValue;

pub fn get_script_contents(scriptfile: &str) -> Result<String, std::io::Error> {
    let contents = read_to_string(scriptfile)?;

    Ok(contents)
}

pub fn assert_literal_number(val: &Option<LiteralValue>, number: f64) {
    assert_eq!(false, val.is_none());

    if let LiteralValue::Number(n) = val.as_ref().unwrap() {
        assert_eq!(number, *n);
    } else {
        assert_eq!(false, true, "Expected a number value {}", number);
    }
}

pub fn assert_literal_string(val: &Option<LiteralValue>, string: &str) {
    assert_eq!(false, val.is_none());

    if let LiteralValue::String(s) = val.as_ref().unwrap() {
        assert_eq!(string, *s);
    } else {
        assert_eq!(false, true, "Expected a string value {}", string);
    }
}

#[allow(dead_code)]
pub fn assert_literal_boolean(val: &Option<LiteralValue>, boolean: bool) {
    assert_eq!(false, val.is_none());

    if let LiteralValue::Boolean(b) = val.as_ref().unwrap() {
        assert_eq!(boolean, *b);
    } else {
        assert_eq!(false, true, "Expected a Boolean value {}", boolean);
    }    
}

pub fn assert_literal_none(val: &Option<LiteralValue>) {
    assert_eq!(true, val.is_none());
}
