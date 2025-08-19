// Date Created: 12/08/2025.

//! Uses data from `./data/`.

//! Tests for [`rlox::resolver??::Resolver??`] (src/resolver.rs??), Chapter 12 [Classes]
//! (https://craftinginterpreters.com/classes.html).
//! 
//! To run test for this module only: 
//! 
//!     * cargo test --test test_classes
//! 
//! To run a specific test method: 
//!
//!     * cargo test test_class_and_instance -- --exact [--nocapture]
//!     * cargo test test_class_field_and_property -- --exact [--nocapture]
//!     * cargo test test_class_methods_on_classes -- --exact [--nocapture]
//!     * cargo test test_class_this -- --exact [--nocapture]
//!     * cargo test test_class_invalid_uses_of_this -- --exact [--nocapture]
//!     * cargo test test_class_constructor_and_initializer -- --exact [--nocapture]
//!     * cargo test test_class_invalid_returning_from_init -- --exact [--nocapture]
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
// https://craftinginterpreters.com/classes.html#creating-instances
fn get_class_and_instance_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/class
        TestScriptAndResult {
            script_name: "./tests/data/class/empty.lox",
            expected_result: true,
            expected_output: vec!["Foo"],
        },
        // From https://craftinginterpreters.com/classes.html#creating-instances
        TestScriptAndResult {
            script_name: "./tests/data/class/book_creating_instances.lox",
            expected_result: true,
            expected_output: vec!["Bagel instance"],
        },
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/call
        TestScriptAndResult {
            script_name: "./tests/data/call/object.lox",
            expected_result: false,
            expected_output: vec!["[line 4] Error at ')': Can only call functions and classes."],
        },
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/operator
        TestScriptAndResult {
            script_name: "./tests/data/operator/equals_class.lox",
            expected_result: true,
            expected_output: vec!["true", "false", "false", "true",
                "false", "false", "false", "false"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/operator/not_class.lox",
            expected_result: true,
            expected_output: vec!["false", "false"],
        },        
    ]
} // cargo test test_class_and_instance -- --exact [--nocapture]

// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#properties-on-instances
fn get_class_field_and_property_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/field
        TestScriptAndResult {
            script_name: "./tests/data/field/set_on_bool.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at 'foo': Only instances have fields."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/set_on_class.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'bar': Only instances have fields."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/set_on_function.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at 'bar': Only instances have fields."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/set_on_num.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at 'foo': Only instances have fields."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/set_on_string.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at 'foo': Only instances have fields."],
        },
    ]
} // cargo test test_class_field_and_property -- --exact [--nocapture]

// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#methods-on-classes
fn get_class_methods_on_classes_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/variable
        TestScriptAndResult {
            script_name: "./tests/data/variable/local_from_method.lox",
            expected_result: true,
            expected_output: vec!["variable"],
        },
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/return
        TestScriptAndResult {
            script_name: "./tests/data/return/in_method.lox",
            expected_result: true,
            expected_output: vec!["ok"],
        },
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/closure
        TestScriptAndResult {
            script_name: "./tests/data/closure/close_over_method_parameter.lox",
            expected_result: true,
            expected_output: vec!["param"],
        },
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/class
        TestScriptAndResult {
            script_name: "./tests/data/class/local_reference_self.lox",
            expected_result: true,
            expected_output: vec!["Foo"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/class/reference_self.lox",
            expected_result: true,
            expected_output: vec!["Foo"],
        },
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/field
        TestScriptAndResult {
            script_name: "./tests/data/field/call_function_field.lox",
            expected_result: true,
            // Normalise f64.
            expected_output: vec!["bar", "1.0", "2.0"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/call_nonfunction_field.lox",
            expected_result: false,
            expected_output: vec!["[line 6] Error at ')': Can only call functions and classes."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/get_and_set_method.lox",
            expected_result: true,
            // Normalise f64.
            expected_output: vec!["other", "1.0", "method", "2.0"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/get_on_bool.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at 'foo': Only instances have properties."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/get_on_class.lox",
            expected_result: false,
            expected_output: vec!["[line 2] Error at 'bar': Only instances have properties."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/get_on_function.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at 'bar': Only instances have properties."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/get_on_nil.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at 'foo': Only instances have properties."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/get_on_num.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at 'foo': Only instances have properties."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/get_on_string.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at 'foo': Only instances have properties."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/many.lox",
            expected_result: true,
            expected_output: vec!["apple", "apricot", "avocado", "banana", "bilberry", 
                "blackberry", "blackcurrant", "blueberry", "boysenberry", "cantaloupe", 
                "cherimoya", "cherry", "clementine", "cloudberry", "coconut", 
                "cranberry", "currant", "damson", "date", "dragonfruit", 
                "durian", "elderberry", "feijoa", "fig", "gooseberry", 
                "grape", "grapefruit", "guava", "honeydew", "huckleberry", 
                "jabuticaba", "jackfruit", "jambul", "jujube", "juniper", 
                "kiwifruit", "kumquat", "lemon", "lime", "longan", 
                "loquat", "lychee", "mandarine", "mango", "marionberry", 
                "melon", "miracle", "mulberry", "nance", "nectarine", 
                "olive", "orange", "papaya", "passionfruit", "peach", 
                "pear", "persimmon", "physalis", "pineapple", "plantain", 
                "plum", "plumcot", "pomegranate", "pomelo", "quince", 
                "raisin", "rambutan", "raspberry", "redcurrant", "salak", 
                "salmonberry", "satsuma", "strawberry", "tamarillo", "tamarind", 
                "tangerine", "tomato", "watermelon", "yuzu"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/method.lox",
            expected_result: true,
            expected_output: vec!["got method", "arg"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/on_instance.lox",
            expected_result: true,
            expected_output: vec!["bar value", "baz value", "bar value", "baz value"],
        }, //
        TestScriptAndResult {
            script_name: "./tests/data/field/set_evaluation_order.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at 'undefined1': Undefined variable 'undefined1'."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/set_on_nil.lox",
            expected_result: false,
            expected_output: vec!["[line 1] Error at 'foo': Only instances have fields."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/field/undefined.lox",
            expected_result: false,
            expected_output: vec!["[line 4] Error at 'bar': Undefined property 'bar'."],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/operator
        TestScriptAndResult {
            script_name: "./tests/data/operator/equals_method.lox",
            expected_result: true,
            expected_output: vec!["true", "false"],
        },
        // Author's https://github.com/munificent/craftinginterpreters/tree/master/test/method
        TestScriptAndResult {
            script_name: "./tests/data/method/arity.lox",
            expected_result: true,
            // Normalise f64.
            expected_output: vec!["no args", "1.0", "3.0", "6.0", "10.0", 
                "15.0", "21.0", "28.0", "36.0"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/method/empty_block.lox",
            expected_result: true,
            expected_output: vec!["nil"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/method/extra_arguments.lox",
            expected_result: false,
            expected_output: vec!["[line 8] Error at ')': Expected 2 arguments but got 4."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/method/missing_arguments.lox",
            expected_result: false,
            expected_output: vec!["[line 5] Error at ')': Expected 2 arguments but got 1."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/method/not_found.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at 'unknown': Undefined property 'unknown'."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/method/print_bound_method.lox",
            expected_result: true,
            expected_output: vec!["<fn method>"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/method/refer_to_name.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at 'method': Undefined variable 'method'."],
        },
    ]
} // cargo test test_class_methods_on_classes -- --exact [--nocapture]

// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#this
fn get_class_this_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/field
        TestScriptAndResult {
            script_name: "./tests/data/field/method_binds_this.lox",
            expected_result: true,
            expected_output: vec!["foo1", "1.0"],
        },
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/this
        TestScriptAndResult {
            script_name: "./tests/data/this/closure.lox",
            expected_result: true,
            expected_output: vec!["Foo"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/this/nested_class.lox",
            expected_result: true,
            expected_output: vec!["Outer instance", "Outer instance", "Inner instance"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/this/nested_closure.lox",
            expected_result: true,
            expected_output: vec!["Foo"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/this/this_in_method.lox",
            expected_result: true,
            expected_output: vec!["baz"],
        },
    ]
} // cargo test test_class_this -- --exact [--nocapture]

// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#this
// Resolver error:
// https://craftinginterpreters.com/classes.html#invalid-uses-of-this
fn get_class_invalid_uses_of_this_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/this
        TestScriptAndResult {
            script_name: "./tests/data/this/this_at_top_level.lox",
            expected_result: false,
            expected_output: vec!["Error: [line 1] Error at 'this': Can't use 'this' outside of a class."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/this/this_in_top_level_function.lox",
            expected_result: false,
            expected_output: vec!["Error: [line 2] Error at 'this': Can't use 'this' outside of a class."],
        },
    ]
} // cargo test test_class_invalid_uses_of_this -- --exact [--nocapture]


// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#constructors-and-initializers
fn get_class_constructor_and_initializer_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/constructor
        TestScriptAndResult {
            script_name: "./tests/data/constructor/arguments.lox",
            expected_result: true,
            // Normalise f64.
            expected_output: vec!["init", "1.0", "2.0"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/constructor/call_init_early_return.lox",
            expected_result: true,
            expected_output: vec!["init", "init", "Foo instance"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/constructor/call_init_explicitly.lox",
            expected_result: true,
            expected_output: vec!["Foo.init(one)", "Foo.init(two)", "Foo instance", "init"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/constructor/default.lox",
            expected_result: true,
            expected_output: vec!["Foo instance"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/constructor/default_arguments.lox",
            expected_result: false,
            expected_output: vec!["[line 3] Error at ')': Expected 0 arguments but got 3."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/constructor/early_return.lox",
            expected_result: true,
            expected_output: vec!["init", "Foo instance"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/constructor/extra_arguments.lox",
            expected_result: false,
            expected_output: vec!["[line 8] Error at ')': Expected 2 arguments but got 4."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/constructor/init_not_method.lox",
            expected_result: true,
            expected_output: vec!["not initializer"],
        },
        TestScriptAndResult {
            script_name: "./tests/data/constructor/missing_arguments.lox",
            expected_result: false,
            expected_output: vec!["[line 5] Error at ')': Expected 2 arguments but got 1."],
        },
        TestScriptAndResult {
            script_name: "./tests/data/constructor/return_in_nested_function.lox",
            expected_result: true,
            expected_output: vec!["bar", "Foo instance"],
        },
    ]
} // cargo test test_class_constructor_and_initializer -- --exact [--nocapture]

// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#constructors-and-initializers
// Resolver error:
// https://craftinginterpreters.com/classes.html#returning-from-init
fn get_class_invalid_returning_from_init_script_results<'a>() -> TestScriptAndResults<'a> {
    vec![
        // From author's https://github.com/munificent/craftinginterpreters/tree/master/test/constructor
        TestScriptAndResult {
            script_name: "./tests/data/constructor/return_value.lox",
            expected_result: false,
            expected_output: vec!["Error: [line 3] Error at 'return': Can't return a value from an initializer."],
        },
    ]
} // cargo test test_class_invalid_returning_from_init -- --exact [--nocapture]

#[test]
// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#creating-instances
fn test_class_and_instance() {
    let cls_and_inst_script_results = get_class_and_instance_script_results();

    // Resolver needs an mutable Interpreter instance.
    // let mut interpreter = make_interpreter_byte_stream();
    // Resolver instance.
    // let mut resolver: Resolver = Resolver::new(&mut interpreter);

    for entry in cls_and_inst_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Resolver needs an mutable Interpreter instance.
        let mut interpreter = make_interpreter_byte_stream();

        // Create a resolver instance for each script file.
        let mut resolver: Resolver = Resolver::new(&mut interpreter);

        // Resolver test.
        let res = resolver.resolve(&statements);

        // Ensure resolving is successful.
        assert!(res.is_ok(), "method() resolve error: {}", entry.script_name);

        // Test interpreting/evaluating.
        // interpreter.clear_output();
        let res = interpreter.interpret(&statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
}

#[test]
// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#properties-on-instances
fn test_class_field_and_property() {
    let fld_and_ppt_script_results = get_class_field_and_property_script_results();

    // Resolver needs an mutable Interpreter instance.
    // let mut interpreter = make_interpreter_byte_stream();
    // Resolver instance.
    // let mut resolver: Resolver = Resolver::new(&mut interpreter);

    for entry in fld_and_ppt_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Resolver needs an mutable Interpreter instance.
        let mut interpreter = make_interpreter_byte_stream();

        // Create a resolver instance for each script file.
        let mut resolver: Resolver = Resolver::new(&mut interpreter);

        // Resolver test.
        let res = resolver.resolve(&statements);

        // Ensure resolving is successful.
        assert!(res.is_ok(), "method() resolve error: {}", entry.script_name);

        // Test interpreting/evaluating.
        // interpreter.clear_output();
        let res = interpreter.interpret(&statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
}

#[test]
// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#methods-on-classes
fn test_class_methods_on_classes() {
    let mtds_on_clses_script_results = get_class_methods_on_classes_script_results();

    // Resolver needs an mutable Interpreter instance.
    // let mut interpreter = make_interpreter_byte_stream();
    // Resolver instance.
    // let mut resolver: Resolver = Resolver::new(&mut interpreter);

    for entry in mtds_on_clses_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Resolver needs an mutable Interpreter instance.
        let mut interpreter = make_interpreter_byte_stream();

        // Create a resolver instance for each script file.
        let mut resolver: Resolver = Resolver::new(&mut interpreter);

        // Resolver test.
        let res = resolver.resolve(&statements);

        // Ensure resolving is successful.
        assert!(res.is_ok(), "method() resolve error: {}", entry.script_name);

        // Test interpreting/evaluating.
        // interpreter.clear_output();
        let res = interpreter.interpret(&statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
}

#[test]
// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#this
fn test_class_this() {
    let class_this_script_results = get_class_this_script_results();

    // Resolver needs an mutable Interpreter instance.
    // let mut interpreter = make_interpreter_byte_stream();
    // Resolver instance.
    // let mut resolver: Resolver = Resolver::new(&mut interpreter);

    for entry in class_this_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Resolver needs an mutable Interpreter instance.
        let mut interpreter = make_interpreter_byte_stream();

        // Create a resolver instance for each script file.
        let mut resolver: Resolver = Resolver::new(&mut interpreter);

        // Resolver test.
        let res = resolver.resolve(&statements);

        // Ensure resolving is successful.
        assert!(res.is_ok(), "method() resolve error: {}", entry.script_name);

        // Test interpreting/evaluating.
        // interpreter.clear_output();
        let res = interpreter.interpret(&statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
}

#[test]
// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#this
// Resolver error:
// https://craftinginterpreters.com/classes.html#invalid-uses-of-this
fn test_class_invalid_uses_of_this() {
    let cls_invalid_uses_of_this_script_results = get_class_invalid_uses_of_this_script_results();

    // Resolver needs an mutable Interpreter instance.
    let mut interpreter = make_interpreter_byte_stream();
    // Resolver instance.
    let mut resolver: Resolver = Resolver::new(&mut interpreter);

    for entry in cls_invalid_uses_of_this_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Resolver test.
        let res = resolver.resolve(&statements);

        assert_resolver_result(&entry, &res);
    }    
}


#[test]
// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#constructors-and-initializers
fn test_class_constructor_and_initializer() {
    let class_const_init_script_results = get_class_constructor_and_initializer_script_results();

    // Resolver needs an mutable Interpreter instance.
    // let mut interpreter = make_interpreter_byte_stream();
    // Resolver instance.
    // let mut resolver: Resolver = Resolver::new(&mut interpreter);

    for entry in class_const_init_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Resolver needs an mutable Interpreter instance.
        let mut interpreter = make_interpreter_byte_stream();

        // Create a resolver instance for each script file.
        let mut resolver: Resolver = Resolver::new(&mut interpreter);

        // Resolver test.
        let res = resolver.resolve(&statements);

        // Ensure resolving is successful.
        assert!(res.is_ok(), "method() resolve error: {}", entry.script_name);

        // Test interpreting/evaluating.
        // interpreter.clear_output();
        let res = interpreter.interpret(&statements);

        assert_interpreter_result(&entry, &res, &interpreter);
    }
}

#[test]
// Scripts in this method test code up to section:
// https://craftinginterpreters.com/classes.html#constructors-and-initializers
// Resolver error:
// https://craftinginterpreters.com/classes.html#returning-from-init
fn test_class_invalid_returning_from_init() {
    let cls_invalid_returning_from_init_script_results = get_class_invalid_returning_from_init_script_results();

    // Resolver needs an mutable Interpreter instance.
    let mut interpreter = make_interpreter_byte_stream();
    // Resolver instance.
    let mut resolver: Resolver = Resolver::new(&mut interpreter);

    for entry in cls_invalid_returning_from_init_script_results {
        // Ensure script is loaded, scanned and parsed successfully.
        let statements = assert_parse_script_statements(entry.script_name);

        // Resolver test.
        let res = resolver.resolve(&statements);

        assert_resolver_result(&entry, &res);
    }    
}