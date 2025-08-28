# rlox: A Rust Implementation of “Crafting Interpreters”

A work-in-progress Rust implementation of Robert Nystrom's Lox language, as presented in [*Crafting Interpreters*](https://craftinginterpreters.com/contents.html).

For each completed stage, I document my progress in a post, which is listed in the [Related post(s)](#related-posts) section below.

## 📘 RLox Language Guide

A working guide to the Lox language features currently implemented in this project.  
→ [View RLoxGuide.md](https://github.com/behai-nguyen/rlox/blob/main/docs/RLoxGuide.md)

## Process Flow: `Scanner`, `Parser`, `Resolver`, and `Interpreter`

When the `Scanner` encounters an error, no token list is produced, so the `Parser` cannot and should not run. Similarly, if the `Parser` fails, no statement list is available, and the  `Resolver` should not run. If the `Resolver` fails, the `Interpreter` should not run either. While the statement list from the `Parser` remains available, attempting to evaluate unresolved statements with the `Interpreter` would be futile.

This interaction between the four components is illustrated in the flowchart below:

![scanner-parser-resolver-interpret.png](https://behainguyen.wordpress.com/wp-content/uploads/2025/08/scanner-parser-resolver-interpret.png)

## To Run

Clone the repository to your local machine:

```
$ git clone https://github.com/behai-nguyen/rlox.git
```

Change to the ``rlox`` directory. <!-- The application can be run interactively or with a Lox script file. -->

💥 The application's behavior will evolve as development progresses. As a result, the discussions below will be updated accordingly.

<!--
At this stage, the parser can only parse expressions. Please refer to the `RLox Language Guide`'s [Expressions and Operators](./docs/RLoxGuide.md#expressions-and-operators) for valid expressions.

### To Run Interactively

```
$ cargo run --release
```

Enter something such as `("abc" * (4.5 / 2));`, and press `Enter` — you will see the parser and the evaluation (interpreter) results printed out.

Currently, inputs are treated independently, meaning each new input has no relation to the previous one.

To exit, simply press ``Enter`` without entering anything.

### To Run with a Lox Script File
-->

```
$ cargo run --release ./tests/data/constructor/call_init_explicitly.lox
```

If there are no errors, you will see the <!-- parser and the evaluation (interpreter) --> results printed out.

## Related post(s)

1. [rlox: A Rust Implementation of “Crafting Interpreters” – Scanner](https://behainguyen.wordpress.com/2025/06/14/rlox-a-rust-implementation-of-crafting-interpreters-scanner/)

The code version for the above post has been tagged with **v0.1.0**. It can be cloned with:
  
```
git clone -b v0.1.0 https://github.com/behai-nguyen/rlox.git
```

I am attempting a Rust implementation of Robert Nystrom's Lox language discussed in <a href="https://craftinginterpreters.com/" title="Crafting Interpreters" target="_blank">Crafting Interpreters</a>. This post describes my Rust code equivalence for the <a href="https://craftinginterpreters.com/scanning.html" title="Scanning" target="_blank">Scanning</a> chapter.

2. [Visitor Pattern with Rust](https://behainguyen.wordpress.com/2025/06/28/visitor-pattern-with-rust/)

<a href="https://craftinginterpreters.com/" title="Crafting Interpreters" target="_blank">Crafting Interpreters</a> makes use of the <a href="https://www.google.com/search?q=visitor+pattern&sca_esv=031453346484e3fe&sxsrf=AE3TifMt1Zv09mlol6xvyuLSIe-SuQ8RVw%3A1750945325549&source=hp&ei=LU5daNi5GZjc2roPj7nA-Q0&iflsig=AOw8s4IAAAAAaF1cParYGjGJNLFkiCZUR2Fzc8uzg2FT&ved=0ahUKEwiYhJLRm4-OAxUYrlYBHY8cMN8Q4dUDCBk&uact=5&oq=visitor+pattern&gs_lp=Egdnd3Mtd2l6GgIYAyIPdmlzaXRvciBwYXR0ZXJuMgoQIxiABBgnGIoFMgUQABiABDIFEAAYgAQyBRAAGIAEMgUQABiABDIIEAAYgAQYiwMyCBAAGIAEGIsDMgUQABiABDIIEAAYgAQYiwMyCBAAGIAEGIsDSIkpUABY7iZwAngAkAEAmAGuAqABkRuqAQcwLjkuNy4xuAEDyAEA-AEBmAIToAKeHMICFBAuGIAEGJECGLEDGNEDGMcBGIoFwgILEAAYgAQYkQIYigXCAg4QLhiABBixAxiDARiKBcICCxAuGIAEGNEDGMcBwgIIEAAYgAQYsQPCAhoQLhiABBixAxjRAxjSAxiDARjHARioAxiLA8ICBBAjGCfCAhAQLhiABBhDGMcBGIoFGK8BwgINEAAYgAQYQxiKBRiLA8ICHBAuGIAEGEMYpgMYxwEYqAMYigUYiwMYjgUYrwHCAgoQABiABBhDGIoFwgIdEC4YgAQYkQIYsQMY0QMY0gMYxwEYqAMYigUYiwPCAg4QABiABBiRAhiKBRiLA8ICChAuGIAEGEMYigXCAg0QABiABBixAxhDGIoFwgIUEC4YgAQYpgMYxwEYqAMYiwMYrwHCAhMQLhiABBhDGMcBGIoFGI4FGK8BwgIWEC4YgAQYsQMY0QMYQxiDARjHARiKBcICERAuGIAEGLEDGIMBGMcBGK8BwgIUEC4YgAQYkQIYxwEYigUYjgUYrwHCAgsQLhiABBjHARivAcICDhAuGIAEGLEDGNEDGMcBwgIHECMYsQIYJ8ICChAAGIAEGLEDGArCAgcQABiABBgKwgINEC4YgAQYxwEYChivAcICExAuGIAEGLEDGIMBGMcBGAoYrwHCAgsQABiABBixAxiDAcICCxAAGIAEGLEDGIoFwgIOEC4YgAQYxwEYjgUYrwGYAwCSBwcyLjcuOS4xoAesrAGyBwcwLjcuOS4xuAeHHMIHBjItMTMuNsgHdg&sclient=gws-wiz" title="Google search: visitor pattern" target="_blank">visitor pattern</a>, which I’m not yet familiar with. To better understand it, I’ve attempted to implement the C# and Java examples from the <a href="https://en.wikipedia.org/wiki/Visitor_pattern" title="Wikipedia: Visitor pattern" target="_blank">Wikipedia Visitor pattern</a> article in Rust. Short, isolated examples like these help us grasp the underlying theory more effectively.

We won’t be discussing the theory of the visitor pattern in this post.

3. [rlox: A Rust Implementation of “Crafting Interpreters” – Abstract Syntax Tree (AST) – Representing Code](https://behainguyen.wordpress.com/2025/07/10/rlox-a-rust-implementation-of-crafting-interpreters-abstract-syntax-tree-ast-representing-code/)

The code version for the above post has been tagged with **v0.1.1**. It can be cloned with:
  
```
git clone -b v0.1.1 https://github.com/behai-nguyen/rlox.git
```

The primary focus of this post is Chapter 5: <a href="https://craftinginterpreters.com/representing-code.html" title="Representing Code" target="_blank">Representing Code</a>, in which the author introduces an independent tool to generate ASTs for both expressions and statements, followed by a printer for displaying the AST. This post briefly discusses my Rust implementation of both tools.

4. [rlox: A Rust Implementation of “Crafting Interpreters” – Parsing and Evaluating Expressions](https://behainguyen.wordpress.com/2025/07/13/rlox-a-rust-implementation-of-crafting-interpreters-parsing-and-evaluating-expressions/)

The code version for the above post has been tagged with **v0.2.0**. It can be cloned with:
  
```
git clone -b v0.2.0 https://github.com/behai-nguyen/rlox.git
```

In this post, I briefly describe the implementation of the code in Chapter 6: <a href="https://craftinginterpreters.com/parsing-expressions.html" title="Parsing Expressions" target="_blank">Parsing Expressions</a>, and Chapter 7: <a href="https://craftinginterpreters.com/evaluating-expressions.html" title="Evaluating Expressions" target="_blank">Evaluating Expressions</a>.

5. [rlox: A Rust Implementation of “Crafting Interpreters” – Global Variables, Assignment, and Scope](https://behainguyen.wordpress.com/2025/07/22/rlox-a-rust-implementation-of-crafting-interpreters-global-variables-assignment-and-scope/)

The code version for the above post has been tagged with **v0.3.0**. It can be cloned with:
  
```
git clone -b v0.3.0 https://github.com/behai-nguyen/rlox.git
```

I have completed Chapter 8: <a href="https://craftinginterpreters.com/statements-and-state.html" title="Statements and State" target="_blank">Statements and State</a>. The following additional statements and expressions have been implemented: <code>Stmt::Expression</code>, <code>Stmt::Print</code>, <code>Stmt::Var</code>, <code>Expr::Variable</code>, <code>Expr::Assign</code> and <code>Stmt::Block</code>. We can now declare global variables, define scoped variables, and assign values to variables. This post discusses some implementation issues that deserve attention.

6. [rlox: A Rust Implementation of “Crafting Interpreters” – Control Flow](https://behainguyen.wordpress.com/2025/07/25/rlox-a-rust-implementation-of-crafting-interpreters-control-flow/)

The code version for the above post has been tagged with **v0.4.0**. It can be cloned with:
  
```
git clone -b v0.4.0 https://github.com/behai-nguyen/rlox.git
```

This is Chapter 8: <a href="https://craftinginterpreters.com/control-flow.html" title="Control Flow" target="_blank">Control Flow</a>. The following additional statements and expressions have been implemented: <code>Stmt::If</code>, <code>Expr::Logical</code>, and <code>Stmt::While</code>. Lox now supports <code>if</code>, <code>else</code>, <code>and</code>, <code>or</code>, <code>while</code>, and <code>for</code>. Despite this long list of new features, the implementation remains fairly straightforward.

7. [rlox: A Rust Implementation of “Crafting Interpreters” – Functions](https://behainguyen.wordpress.com/2025/08/01/rlox-a-rust-implementation-of-crafting-interpreters-functions/)

The code version for the above post has been tagged with **v0.5.0**. It can be cloned with:
  
```
git clone -b v0.5.0 https://github.com/behai-nguyen/rlox.git
```

This post covers Chapter 10 of <a href="https://craftinginterpreters.com/functions.html" title="Functions" target="_blank">Crafting Interpreters</a>: <strong>Functions</strong>. The following new syntax elements have been implemented: <code>Expr::Call</code>, <code>Stmt::Function</code>, and <code>Stmt::Return</code>. Lox now supports <code>fun</code>, <code>return</code>, and <code>closures</code>. This post discusses several implementation details that deserve attention.

8. [rlox: A Rust Implementation of “Crafting Interpreters” – Resolving and Binding](https://behainguyen.wordpress.com/2025/08/12/rlox-a-rust-implementation-of-crafting-interpreters-resolving-and-binding/)

The code version for the above post has been tagged with **v0.5.1**. It can be cloned with:
  
```
git clone -b v0.5.1 https://github.com/behai-nguyen/rlox.git
```

This post covers Chapter 11 of <a href="https://craftinginterpreters.com/resolving-and-binding.html" title="Resolving and Binding" target="_blank">Crafting Interpreters</a>: <strong>Resolving and Binding</strong>. No new syntax elements are introduced in this chapter. Instead, Chapter 11 serves as a kind of patch to Chapter 10: it ensures that variables are resolved within their correct <code>closures</code>. The code for this chapter is relatively straightforward, but I made a mistake that introduced a subtle bug—one that took me a long time to diagnose and finally fix.

9. [rlox: A Rust Implementation of “Crafting Interpreters” – Classes](https://behainguyen.wordpress.com/2025/08/19/rlox-a-rust-implementation-of-crafting-interpreters-classes/)

The code version for the above post has been tagged with **v0.6.0**. It can be cloned with:
  
```
git clone -b v0.6.0 https://github.com/behai-nguyen/rlox.git
```

This post covers Chapter 12 of <a href="https://craftinginterpreters.com/classes.html" title="Classes" target="_blank">Crafting Interpreters</a>: <strong>Classes</strong>. The following new syntax elements have been implemented: <code>Stmt::Class</code>, <code>Expr::Get</code>, <code>Expr::Set</code>, and <code>Expr::This</code>. Lox now supports <code>class</code>, <code>this</code>, and <code>init</code>. While implementing this chapter, I encountered two stack overflow bugs and several cases where author-provided test scripts produced incorrect results. This post discusses those issues in detail.

10. [rlox: A Rust Implementation of “Crafting Interpreters” – Inheritance](https://behainguyen.wordpress.com/2025/08/24/rlox-a-rust-implementation-of-crafting-interpreters-inheritance/)

The code version for the above post has been tagged with **v0.6.1**. It can be cloned with:
  
```
git clone -b v0.6.1 https://github.com/behai-nguyen/rlox.git
```

This post covers Chapter 13 of <a href="https://craftinginterpreters.com/inheritance.html" title="Inheritance" target="_blank">Crafting Interpreters</a>: <strong>Inheritance</strong>. Class inheritance syntax <span style="font-weight:bold;font-size:1.5em;"><code>&lt;</code></span> — <code>Class</code> <span style="font-weight:bold;font-size:1.5em;"><code>&lt;</code></span> <code>SuperClass</code> — has been implemented. The final remaining syntax element, <code>Expr::Super</code>, representing the <code>super</code> keyword, has also been added. In this post, we briefly discuss the new code, followed by bug fixes and test updates.

11. Code Revision **v0.6.2**

Clone this version with:

```
git clone -b v0.6.2 https://github.com/behai-nguyen/rlox.git
```

* General refactoring: test structure and error display improvements.
* Bug fix: input script file existence is now checked before execution.

## License
[MIT license](http://www.opensource.org/licenses/mit-license.php)
and the [Creative Commons](  https://creativecommons.org/licenses/by-nc-nd/4.0/).
