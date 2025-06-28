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

2. [Visitor Pattern with Rust](https://behainguyen.wordpress.com/2025/06/28/visitor-pattern-with-rust/)

<a href="https://craftinginterpreters.com/" title="Crafting Interpreters" target="_blank">Crafting Interpreters</a> makes use of the <a href="https://www.google.com/search?q=visitor+pattern&sca_esv=031453346484e3fe&sxsrf=AE3TifMt1Zv09mlol6xvyuLSIe-SuQ8RVw%3A1750945325549&source=hp&ei=LU5daNi5GZjc2roPj7nA-Q0&iflsig=AOw8s4IAAAAAaF1cParYGjGJNLFkiCZUR2Fzc8uzg2FT&ved=0ahUKEwiYhJLRm4-OAxUYrlYBHY8cMN8Q4dUDCBk&uact=5&oq=visitor+pattern&gs_lp=Egdnd3Mtd2l6GgIYAyIPdmlzaXRvciBwYXR0ZXJuMgoQIxiABBgnGIoFMgUQABiABDIFEAAYgAQyBRAAGIAEMgUQABiABDIIEAAYgAQYiwMyCBAAGIAEGIsDMgUQABiABDIIEAAYgAQYiwMyCBAAGIAEGIsDSIkpUABY7iZwAngAkAEAmAGuAqABkRuqAQcwLjkuNy4xuAEDyAEA-AEBmAIToAKeHMICFBAuGIAEGJECGLEDGNEDGMcBGIoFwgILEAAYgAQYkQIYigXCAg4QLhiABBixAxiDARiKBcICCxAuGIAEGNEDGMcBwgIIEAAYgAQYsQPCAhoQLhiABBixAxjRAxjSAxiDARjHARioAxiLA8ICBBAjGCfCAhAQLhiABBhDGMcBGIoFGK8BwgINEAAYgAQYQxiKBRiLA8ICHBAuGIAEGEMYpgMYxwEYqAMYigUYiwMYjgUYrwHCAgoQABiABBhDGIoFwgIdEC4YgAQYkQIYsQMY0QMY0gMYxwEYqAMYigUYiwPCAg4QABiABBiRAhiKBRiLA8ICChAuGIAEGEMYigXCAg0QABiABBixAxhDGIoFwgIUEC4YgAQYpgMYxwEYqAMYiwMYrwHCAhMQLhiABBhDGMcBGIoFGI4FGK8BwgIWEC4YgAQYsQMY0QMYQxiDARjHARiKBcICERAuGIAEGLEDGIMBGMcBGK8BwgIUEC4YgAQYkQIYxwEYigUYjgUYrwHCAgsQLhiABBjHARivAcICDhAuGIAEGLEDGNEDGMcBwgIHECMYsQIYJ8ICChAAGIAEGLEDGArCAgcQABiABBgKwgINEC4YgAQYxwEYChivAcICExAuGIAEGLEDGIMBGMcBGAoYrwHCAgsQABiABBixAxiDAcICCxAAGIAEGLEDGIoFwgIOEC4YgAQYxwEYjgUYrwGYAwCSBwcyLjcuOS4xoAesrAGyBwcwLjcuOS4xuAeHHMIHBjItMTMuNsgHdg&sclient=gws-wiz" title="Google search: visitor pattern" target="_blank">visitor pattern</a>, which I’m not yet familiar with. To better understand it, I’ve attempted to implement the C# and Java examples from the <a href="https://en.wikipedia.org/wiki/Visitor_pattern" title="Wikipedia: Visitor pattern" target="_blank">Wikipedia Visitor pattern</a> article in Rust. Short, isolated examples like these help us grasp the underlying theory more effectively.

We won’t be discussing the theory of the visitor pattern in this post.

## License
[MIT license](http://www.opensource.org/licenses/mit-license.php)
and the [Creative Commons](  https://creativecommons.org/licenses/by-nc-nd/4.0/).
