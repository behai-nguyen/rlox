// Date Created: 20/08/2025.

//! Uses data from `./data/`.

//! Tests for [`rlox::resolver??::Resolver??`] (src/resolver.rs??), Chapter 13 [Inheritance]
//! (https://craftinginterpreters.com/inheritance.html#superclasses-and-subclasses).
//! 
//! To run test for this module only: 
//! 
//!     * cargo test --test test_inheritance
//! 
//! To run a specific test method: 
//!     * cargo test test_inheritance_super_sub_class_invalid_resolving -- --exact [--nocapture]
//!     * cargo test test_inheritance_super_sub_class -- --exact [--nocapture]
//!     * cargo test test_inheritance_inheriting_method -- --exact [--nocapture]
//!     * cargo test test_inheritance_calling_superclass_method -- --exact [--nocapture]
//!     * cargo test test_inheritance_calling_superclass_method_invalid_super -- --exact [--nocapture]
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

// Scripts in this method test code up to section:
// https://craftinginterpreters.com/inheritance.html#superclasses-and-subclasses
// Resolver error.
fn get_inheritance_super_sub_class_invalid_resolving_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/class
        TestScriptAndResult {
            script_name: "./tests/data/class/inherit_self.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at 'Foo': A class can't inherit from itself."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/class/local_inherit_self.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'Foo': A class can't inherit from itself."],
        },
    ]
} // cargo test test_inheritance_super_sub_class_invalid_resolving -- --exact [--nocapture]

// Scripts in this method test code up to section:
// https://craftinginterpreters.com/inheritance.html#superclasses-and-subclasses
fn get_inheritance_super_sub_class_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/class
        TestScriptAndResult {
            script_name: "./tests/data/class/local_inherit_other.lox",
            expected_result: true,
            expected_output: vec!["B"],
        },
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/regression
        TestScriptAndResult {
            script_name: "./tests/data/regression/394.lox",
            expected_result: true,
            expected_output: vec!["B"],
        },
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/inheritance
        TestScriptAndResult {
            script_name: "./tests/data/inheritance/inherit_from_function.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at 'Subclass': Superclass must be a class."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/inheritance/inherit_from_nil.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'Foo': Superclass must be a class."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/inheritance/inherit_from_number.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'Foo': Superclass must be a class."],
        },        
    ]
} // cargo test test_inheritance_super_sub_class -- --exact [--nocapture]

// Scripts in this method test code up to section:
// https://craftinginterpreters.com/inheritance.html#inheriting-methods
fn get_inheritance_inheriting_method_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/inheritance
        TestScriptAndResult {
            script_name: "./tests/data/inheritance/constructor.lox",
            expected_result: true,
            expected_output: vec!["value"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/inheritance/inherit_methods.lox",
            expected_result: true,
            expected_output: vec!["foo", "bar", "bar"],
        },        
        TestScriptAndResult {
            script_name: "./tests/data/inheritance/set_fields_from_base_class.lox",
            expected_result: true,
            expected_output: vec!["foo 1", "foo 2", 
                "bar 1", "bar 2", 
                "bar 1", "bar 2"],
        },
    ]
} // cargo test test_inheritance_inheriting_method -- --exact [--nocapture]

// Scripts in this method test code up to section:
// https://craftinginterpreters.com/inheritance.html#calling-superclass-methods
// THE SUB-SECTION HAS NOT BEEN IMPLEMENTED YET: 
//    https://craftinginterpreters.com/inheritance.html#invalid-uses-of-super
fn get_calling_superclass_method_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/class
        TestScriptAndResult {
            script_name: "./tests/data/class/inherited_method.lox",
            expected_result: true,
            expected_output: vec!["in foo", "in bar", "in baz"],
        },
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/inheritance
        TestScriptAndResult {
            script_name: "./tests/data/super/bound_method.lox",
            expected_result: true,
            expected_output: vec!["A.method(arg)"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/super/call_other_method.lox",
            expected_result: true,
            expected_output: vec!["Derived.bar()", "Base.foo()"],
        },        
        TestScriptAndResult {
            script_name: "./tests/data/super/call_same_method.lox",
            expected_result: true,
            expected_output: vec!["Derived.foo()", "Base.foo()"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/super/closure.lox",
            expected_result: true,
            expected_output: vec!["Base"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/super/constructor.lox",
            expected_result: true,
            expected_output: vec!["Derived.init()", "Base.init(a, b)"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/super/extra_arguments.lox",
            expected_result: false,
            expected_output: vec!["Derived.foo()", 
                "[line 10] Error at ')': Expected 2 arguments but got 4."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/super/indirectly_inherited.lox",
            expected_result: true,
            expected_output: vec!["C.foo()", "A.foo()"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/super/missing_arguments.lox",
            expected_result: false,
            expected_output: vec!["[line 9] Error at ')': Expected 2 arguments but got 1."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/super/no_superclass_method.lox",
            expected_result: false,
            expected_output: vec!["[line 5] Error at 'doesNotExist': Undefined property 'doesNotExist'."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/super/reassign_superclass.lox",
            expected_result: true,
            expected_output: vec!["Base.method()", "Base.method()"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/super/super_in_closure_in_inherited_method.lox",
            expected_result: true,
            expected_output: vec!["A"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/super/super_in_inherited_method.lox",
            expected_result: true,
            expected_output: vec!["A"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/super/this_in_superclass_method.lox",
            expected_result: true,
            expected_output: vec!["a", "b"],
        },
    ]
} // cargo test test_inheritance_calling_superclass_method -- --exact --nocapture

// Scripts in this method test code up to section:
// https://craftinginterpreters.com/inheritance.html#calling-superclass-methods
// https://craftinginterpreters.com/inheritance.html#invalid-uses-of-super
// Resolver error.
fn get_inheritance_calling_superclass_method_invalid_super_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/super
        TestScriptAndResult {
            script_name: "./tests/data/super/no_superclass_bind.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at 'super': Can't use 'super' in a class with no superclass."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/super/no_superclass_call.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at 'super': Can't use 'super' in a class with no superclass."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/super/super_at_top_level.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at 'super': Can't use 'super' outside of a class.",
                "[line 2] Error at 'super': Can't use 'super' outside of a class."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/super/super_in_top_level_function.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at 'super': Can't use 'super' outside of a class."],
        },
    ]
} // cargo test test_inheritance_calling_superclass_method_invalid_super -- --exact --nocapture


#[test]
// Scripts in this method test code up to section:
// https://craftinginterpreters.com/inheritance.html#superclasses-and-subclasses
// Resolver error.
fn test_inheritance_super_sub_class_invalid_resolving() {
    let iht_invalid_resolving_script_results = get_inheritance_super_sub_class_invalid_resolving_script_results();

    // Resolver needs an mutable Interpreter instance.
    let mut interpreter = make_interpreter_byte_stream();
    // Resolver instance.
    let mut resolver: Resolver = Resolver::new(&mut interpreter);

    for entry in iht_invalid_resolving_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Resolver test.
        let res = resolver.resolve(&statements);

        assert_resolver_result(&entry, &res);
    }    
}

#[test]
// Scripts in this method test code up to section:
// https://craftinginterpreters.com/inheritance.html#superclasses-and-subclasses
fn test_inheritance_super_sub_class() {
    let class_this_script_results = get_inheritance_super_sub_class_script_results();

    // Resolver needs an mutable Interpreter instance.
    let mut interpreter = make_interpreter_byte_stream();

    for entry in class_this_script_results {
        interpreter.reset(false);

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

#[test]
// Scripts in this method test code up to section:
// https://craftinginterpreters.com/inheritance.html#inheriting-methods
fn test_inheritance_inheriting_method() {
    let class_this_script_results = get_inheritance_inheriting_method_script_results();

    // Resolver needs an mutable Interpreter instance.
    let mut interpreter = make_interpreter_byte_stream();

    for entry in class_this_script_results {
        interpreter.reset(false);

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

#[test]
// Scripts in this method test code up to section:
// https://craftinginterpreters.com/inheritance.html#calling-superclass-methods
// THE SUB-SECTION HAS NOT BEEN IMPLEMENTED YET: 
//    https://craftinginterpreters.com/inheritance.html#invalid-uses-of-super
fn test_inheritance_calling_superclass_method() {
    let calling_superclass_method_script_results = get_calling_superclass_method_script_results();

    // Resolver needs an mutable Interpreter instance.
    let mut interpreter = make_interpreter_byte_stream();
    
    for entry in calling_superclass_method_script_results {
        interpreter.reset(false);

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

#[test]
// Scripts in this method test code up to section:
// https://craftinginterpreters.com/inheritance.html#calling-superclass-methods
// https://craftinginterpreters.com/inheritance.html#invalid-uses-of-super
// Resolver error.
//
// Trying to get the `interpreter.clear_output()` output to work.
// The code of this method is a bit different to every others: an 
// Interpreter instance is created once, Resolver instances are created
// inside the loop: My aim is to have all test methods like this.
fn test_inheritance_calling_superclass_method_invalid_super() {
    let calling_superclass_method_invalid_super_script_results = 
        get_inheritance_calling_superclass_method_invalid_super_script_results();

    // Resolver needs an mutable Interpreter instance.
    let mut interpreter = make_interpreter_byte_stream();

    for entry in calling_superclass_method_invalid_super_script_results {
        interpreter.reset(false);

        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        let mut resolver = Resolver::new(&mut interpreter);
        let res = resolver.resolve(&statements);
        assert_resolver_result(&entry, &res);
    }    
}
