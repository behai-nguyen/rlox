<!-- Date Created: 13/07/2025. -->

# Guide to the Lox Language

A brief guide to the Lox language, documenting only the features currently implemented in this project.

For a complete documentation, please see the author original [The Lox Language](https://craftinginterpreters.com/the-lox-language.html) chapter.

## Data Types

See also [Data Types](https://craftinginterpreters.com/the-lox-language.html#data-types).

**Number**: Supports only double-precision floating point:

- An integer: `1234` → normalized to `1234.0`
- A decimal number: `12.34`

**Boolean**: `true` and `false` — used in logical expressions.

**String**: `"I am a string"`, `""`, `"123"`

**Nil**: `nil` — represents the absence of a value.

## Keywords

- `and`
- `class`
- `else`
- `false`
- `for`
- `fun`
- `if`
- `nil`
- `or`
- `print`
- `return`
- `super`
- `this`
- `true`
- `var`
- `while`

> **Note**: Some keywords (such as `class`, `fun`, `while`, etc.) are reserved but not yet implemented.

## Expressions and Operators

### Binary Expressions

| Operators | Arithmetic | Evaluated To |
|-----------|------------|--------------|
| +         | 1 + 1;     | 2            |
| -         | 1 - 1;     | 0            |
| *         | 1 * 1;     | 1            |
| /         | 1 / 1;     | 1            |

| Operators | String | Evaluated To |
|-----------|------------|--------------|
| +         | "abc " + "def"; | "abc def" |

Note: The `+` operator supports string concatenation only when both operands are strings. Mixing types will result in an error.

| Operators | Logical | Evaluated To |
|-----------|------------|--------------|
| >         | ((4.5 / 2) * 2) > 3.5; | true            |
| >=        | (4.5 / 2) >= 8; | false |
| <         | ((4.5 / 2) * 2) < 4.52; | true |
| <=        | (4.5 / 2) <= 1.72; | false |
| !=        | (4.5 / 2) != 1.72; | true |
| !=        | "abc" != "abc"; | false |
| !=        | nil != nil; | false |
| !=        | nil != 134; | true |
| ==        | ((4.5 / 2) * 2) == 4.50; | true |
| ==        | "abc" == "abc"; | true |
| ==        | nil == nil; | true |
| ==        | nil == 134; | false |
| ==        | "abc" == 123; | false |

### Unary Expressions

| Operators | Logical | Evaluated To |
|-----------|------------|--------------|
| !         | !7.04; | false |
| !         | !"abc def"; | false |
| !         | !true; | false |
| !         | !false; | true |
| !         | !nil; | true |

| Operators | Arithmetic | Evaluated To |
|-----------|------------|--------------|
| -         | -7.04;     | -7.04 |
| -         | -(((4.5 / 2) * 2) * 1.25); | -5.625 |
| -         | -(-7.04); | 7.04 | 

### Literal Expressions

> Literal expressions evaluate to their direct value.

| Data Type | Value | Evaluated To |
|-----------|------------|--------------|
| Number    | 7.04; | 7.04 |
| String    | "abc def"; | "abc def" |
| Boolean   | false; | false |
| Nil       | nil; | nil |

> This guide will expand as more of the Lox language is implemented—including statements, functions, and classes.