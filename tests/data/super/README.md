<!--
Date Created: 20/08/2025.
-->

**Please note:**

Scripts is from 

[ https://github.com/munificent/craftinginterpreters/tree/master/test/super ](https://github.com/munificent/craftinginterpreters/tree/master/test/super).

- `parenthesized.lox`, `super_without_dot.lox`, `super_without_name.lox`: Used in [tests/test_parser.rs](https://github.com/behai-nguyen/rlox/blob/main/tests/test_parser.rs)

- `bound_method.lox`, `call_other_method.lox`, `call_same_method.lox`, `closure.lox`, `constructor.lox`, `extra_arguments.lox` (💥), `indirectly_inherited.lox`, `missing_arguments.lox`, `no_superclass_method.lox`, `reassign_superclass.lox`, `super_in_closure_in_inherited_method.lox`, `super_in_inherited_method.lox`, `this_in_superclass_method.lox`, `no_superclass_bind.lox`, `no_superclass_call.lox`, `super_at_top_level.lox`, and `super_in_top_level_function.lox`: Used in [tests/test_inheritance.rs](https://github.com/behai-nguyen/rlox/blob/main/tests/test_inheritance.rs)

**💥 Notes on extra_arguments.lox**

Running via CLI as:

```
cargo run ./tests/data/super/extra_arguments.lox
```

It prints:

```
Derived.foo()
Evaluation error: [line 10] Error at ')': Expected 2 arguments but got 4.
```

`Evaluation error:` is from the CLI, not part of the `Interpreter`'s result.

The test can only see `[line 10] Error at ')': Expected 2 arguments but got 4.`

