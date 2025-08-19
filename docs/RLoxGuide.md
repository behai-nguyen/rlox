<!-- Date Created: 13/07/2025. -->
<!-- https://github.com/behai-nguyen/rlox/blob/main/docs/RLoxGuide.md -->

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
* [Control Flow](#control-flow)
    - [Conditional Execution: if & else](#conditional-execution-if--else)
        * [Example 1](#example-1-1)
        * [Example 2](#example-2-1)
    - [Logical Operators: and & or](#logical-operators-and--or)
        * [Example 1](#example-1-2)
        * [Example 2](#example-2-2)
        * [Example 3](#example-3-1)
    - [While Loops: while](#while-loops-while)
        * [Example 1](#example-1-3)
        * [Example 2](#example-2-3)
    - [For Loops: for](#for-loops-for)
        * [Example 1](#example-1-4)
        * [Example 2](#example-2-4)
        * [Example 3](#example-3-2)
* [Functions](#functions)
    - [Example 1](#example-1-5)
    - [Example 2](#example-2-5)
    - [Example 3: Scope Resolution](#example-3-scope-resolution)
    - [Example 4: Invalid Variable Initialisation](#example-4-invalid-variable-initialisation)
* [Classes](#classes)
    - [Example 1](#example-1-6)
    - [Example 2](#example-2-6)
    - [Examples Using `this`](#examples-using-this)
        * [Example 3a](#example-3a)
        * [Example 3b](#example-3b)
        * [Example 3c](#example-3c)
    - [Example 4](#example-4-1)

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

## Control Flow

### Conditional Execution: `if` & `else`

Taken from this [author test script area](https://github.com/munificent/craftinginterpreters/tree/master/test/if).

#### Example 1

```lox
// Executes the 'then' branch when the condition is truthy.
if (true) print "good"; // expect: good
if (false) print "bad";

// Allow block body.
if (true) { print "block"; } // expect: block

// Assignment within the condition expression.
var a = false;
if (a = true) print a; // expect: true
```

#### Example 2

```lox
// Evaluate the 'else' expression if the condition is false.
if (true) print "good"; else print "bad"; // expect: good
if (false) print "bad"; else print "good"; // expect: good

// Allow block body.
if (false) nil; else { print "block"; } // expect: block
```

### Logical Operators: `and` & `or`

#### Example 1

Logical operators support short-circuit evaluation and return the last evaluated operand.

```lox
print true and false; // expect: false
print false or true; // expect: true
print "text" or nil; // expect: text
print true or false and nil; // expect: true
```

Taken from this [author test script area](https://github.com/munificent/craftinginterpreters/tree/master/test/logical_operator).

#### Example 2

```lox
// Note: These examples assume that integers are treated as truthy values.

// Return the first non-true argument.
print false and 1; // expect: false
print true and 1; // expect: 1
print 1 and 2 and false; // expect: false

// Return the last argument if all are true.
print 1 and true; // expect: true
print 1 and 2 and 3; // expect: 3

// Short-circuit at the first false argument.
var a = "before";
var b = "before";
(a = true) and
    (b = false) and
    (a = "bad");
print a; // expect: true
print b; // expect: false
```

#### Example 3

```lox
// Note: These tests implicitly depend on ints being truthy.

// Return the first true argument.
print 1 or true; // expect: 1
print false or 1; // expect: 1
print false or false or true; // expect: true

// Return the last argument if all are false.
print false or false; // expect: false
print false or false or false; // expect: false

// Short-circuit at the first true argument.
var a = "before";
var b = "before";
(a = false) or
    (b = true) or
    (a = "bad");
print a; // expect: false
print b; // expect: true
```

### While Loops: `while`

#### Example 1

```lox
var i = 1;
while (i <= 5) {
    print i;
    i = i + 1;
}
```

#### Example 2

Adapted from the [official test script](https://github.com/munificent/craftinginterpreters/tree/master/test/while/syntax.lox):

```lox
// Single-expression body.
var c = 0;
while (c < 3) print c = c + 1;

print "----";

// Block body.
var a = 0;
while (a < 3) {
    print a;
    a = a + 1;
}

print "----";

// Statement bodies.
while (false) if (true) 1; else 2;
while (false) while (true) 1;
```

### For Loops: `for`

#### Example 1

From the end of the [For Loops](https://craftinginterpreters.com/control-flow.html#for-loops) section: prints the first 21 Fibonacci numbers.

```lox
var a = 0;
var temp;

for (var b = 1; a < 10000; b = temp + b) {
    print a;
    temp = a;
    a = b;
}
```

#### Example 2

```lox
for (var i = 0; i < 3; i = i + 1) {
    for (var j = 0; j < 2; j = j + 1) {
        print i * j;
    }
}
```

#### Example 3

```lox
for (var i = 1; i <= 10; i = i + 1) {
    print i;
	
	if ((i/2) > 2.5) print "more than halfway through";
	
	if ((i/2) > 2.5) {
        print "also more than halfway through";	
    }
	
	var b = (i/2) > 2.5;
	print b;
}
```
## Functions

### Example 1

This Lox script appears at the beginning of the [Local Functions and Closures](https://craftinginterpreters.com/functions.html#local-functions-and-closures) section.

```lox
fun makeCounter() {
  var i = 0;
  fun count() {
    i = i + 1;
    print i;
  }

  return count;
}

var counter = makeCounter();
counter(); // "1.0".
counter(); // "2.0".
```

### Example 2

This example demonstrates the usage of the native function `clock()`. 

This example demonstrates the use of the native `clock()` function.  
It comes from one of the author's [benchmark scripts](https://github.com/munificent/craftinginterpreters/tree/master/test/benchmark/fib.lox).

```lox
fun fib(n) {
  if (n < 2) return n;
  return fib(n - 2) + fib(n - 1);
}

var start = clock();
print fib(22) == 17711;
print clock() - start;
```

### Example 3: Scope Resolution

Consider the Lox script at the end of the 
[Static Scope](https://craftinginterpreters.com/resolving-and-binding.html#static-scope)
section:

```lox
var a = "global";
{
  fun showA() {
    print a;
  }

  showA();
  var a = "block";
  showA();
}
```

It correctly outputs:

```
global
global
```

### Example 4: Invalid Variable Initialisation

This Lox script is also from the 
[Static Scope](https://craftinginterpreters.com/resolving-and-binding.html#static-scope)
section:

```lox
var a = "outer";
{
  var a = a;
}
```

It raises the following resolution error:

```
Error: [line 3] Error at 'a': Can't read local variable in its own initializer.
```

## Classes

The examples below demonstrate basic class features. For more examples, refer to the following author-provided test script directories on GitHub:

- [master/test/class](https://github.com/munificent/craftinginterpreters/tree/master/test/class)
- [master/test/field](https://github.com/munificent/craftinginterpreters/tree/master/test/field)
- [master/test/method](https://github.com/munificent/craftinginterpreters/tree/master/test/method)
- [master/test/constructor](https://github.com/munificent/craftinginterpreters/tree/master/test/constructor)

### Example 1

This example demonstrates class instantiation and class identity.

Taken from the end of the section [Chapter 12 | Creating Instances](https://craftinginterpreters.com/classes.html#creating-instances):

```lox
class Bagel {}
var bagel = Bagel();
print bagel; // Prints "Bagel instance".
print Bagel; // Prints "Bagel".
```

### Example 2

This example demonstrates methods defined on classes.

Taken from the end of the section [Chapter 12 | Methods on Classes](https://craftinginterpreters.com/classes.html#methods-on-classes):

```
class Bacon {
  eat() {
    print "Crunch crunch crunch!";
  }
}

Bacon().eat(); // Prints "Crunch crunch crunch!".

var bacon = Bacon();
bacon.eat(); // Prints "Crunch crunch crunch!".
```

### Examples Using `this`

The following examples are from the section [Chapter 12 | Methods on Classes](https://craftinginterpreters.com/classes.html#this).

#### Example 3a

```lox
class Egotist {
  speak() {
    print this;
  }
}

var method = Egotist().speak;
method(); // Prints "Egotist instance".
```

#### Example 3b

```lox
class Cake {
  taste() {
    var adjective = "delicious";
    print "The " + this.flavor + " cake is " + adjective + "!";
  }
}

var cake = Cake();
cake.flavor = "German chocolate";
cake.taste(); // Prints "The German chocolate cake is delicious!".
```

#### Example 3c

```lox
class Thing {
  getCallback() {
    fun localFunction() {
      print this;
    }

    return localFunction;
  }
}

var callback = Thing().getCallback();
callback(); // Prints "Thing instance".
```

### Example 4

This example demonstrates constructors (`init()`) and initializers.

Taken from the end of the section [Chapter 12 | Constructors and Initializers](https://craftinginterpreters.com/classes.html#constructors-and-initializers):

```lox
class Foo {
  init() {
    this.name = "behai";
  }

  hello() {
    print "Hello " + this.name + ". How are you?";
  }
}

var foo = Foo();
foo.hello();
```

> This guide will expand as more of the Lox language is implemented—including statements, functions, and classes.
