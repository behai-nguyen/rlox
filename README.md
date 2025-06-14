# rlox: A Rust Implementation of “Crafting Interpreters”

A work-in-progress Rust implementation of Robert Nystrom's Lox language, as presented in [*Crafting Interpreters*](https://craftinginterpreters.com/contents.html).

For each completed stage, I document my progress in a post, which is listed in the [Related post(s)](#related-posts) section below.

## To Run

Clone the repository to your local machine:

```
$ git clone https://github.com/behai-nguyen/rlox.git
```

Change to the ``rlox`` directory. The application can be run interactively or with a Lox script file.

💥 The application's behavior will evolve as development progresses. As a result, the discussions below will be updated accordingly.

### To Run Interactively

```
$ cargo run
```

Enter something like ``var str2 = "秋の終わり";``, and press ``Enter`` — you will see the tokens printed out.

Currently, inputs are treated independently, meaning each new input has no relation to the previous one.

To exit, simply press ``Enter`` without entering anything.

### To Run with a Lox Script File

```
$ cargo run ./tests/data/scanning/numbers.lox
```

If there are no errors, you will see the tokens printed out.

## Related post(s)

1. [rlox: A Rust Implementation of “Crafting Interpreters” – Scanner](https://behainguyen.wordpress.com/2025/06/14/rlox-a-rust-implementation-of-crafting-interpreters-scanner/)

The code version for the above post has been tagged with **v0.1.0**. It can be cloned with:
  
```
git clone -b v0.1.0 https://github.com/behai-nguyen/rlox.git
```

I am attempting a Rust implementation of Robert Nystrom's Lox language discussed in <a href="https://craftinginterpreters.com/" title="Crafting Interpreters" target="_blank">Crafting Interpreters</a>. This post describes my Rust code equivalence for the <a href="https://craftinginterpreters.com/scanning.html" title="Scanning" target="_blank">Scanning</a> chapter.

## License
[MIT license](http://www.opensource.org/licenses/mit-license.php)
and the [Creative Commons](  https://creativecommons.org/licenses/by-nc-nd/4.0/).
