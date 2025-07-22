<!-- Date Created: 13/07/2025. -->

# Guide to the Lox Language

A brief guide to the Lox language, documenting only the features currently implemented in this project.

For a complete documentation, please see the author original [The Lox Language](https://craftinginterpreters.com/the-lox-language.html) chapter.

## Table of Contents

* [Data Types](#data-types)
* [Keywords](#keywords)
* [Expressions and Operators](#expressions-and-operators)
    - [Binary Expressions](#binary-expressions)
    - [Unary Expressions](#unary-expressions)
    - [Literal Expressions](#literal-expressions)
* [Global Variables, Assignment and Scope](#global-variables-assignment-and-scope)
    - [Example 1](#example-1)
    - [Example 2](#example-2)
    - [Example 3](#example-3)
    - [Example 4](#example-4)
    - [Example 5](#example-5)

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

## Global Variables, Assignment, and Scope

The following examples demonstrate valid variable declarations, assignments, and scope behavior in Lox.

### Example 1

```lox
var result;
result = -(((4.5 / 2) * 2) * 1.25);
print result;
```

### Example 2

```lox
var result;
{
    result = -(((4.5 / 2) * 2) * 1.25);
    print result;
}
```

### Example 3

```lox
var result;
{
    var factor = 2;
    result = -(((4.5 / factor) * factor) * 1.25);
}
print result;
```

### Example 4

```lox
{
var s1 = "Mắt trừng gửi mộng qua biên giới";
var s2 = "Đêm mơ Hà Nội dáng kiều thơm";

    {
        print s1;
        print s2;

        var s1 = "Rải rác biên cương mồ viễn xứ";
        var s2 = "Chiến trường đi chẳng tiếc đời xanh";

        print s1;
        print s2;
    }

print s1;
print s2;
}
```

**Output:**

```
Mắt trừng gửi mộng qua biên giới
Đêm mơ Hà Nội dáng kiều thơm
Rải rác biên cương mồ viễn xứ
Chiến trường đi chẳng tiếc đời xanh
Mắt trừng gửi mộng qua biên giới
Đêm mơ Hà Nội dáng kiều thơm
```

### Example 5

End-of-chapter example from [Chapter 8: Statements and State](https://craftinginterpreters.com/statements-and-state.html):

```lox
var a = "global a";
var b = "global b";
var c = "global c";
{
    var a = "outer a";
    var b = "outer b";
    {
        var a = "inner a";
        print a;
        print b;
        print c;
    }
    print a;
    print b;
    print c;
}
print a;
print b;
print c;
```

> This guide will expand as more of the Lox language is implemented—including statements, functions, and classes.
