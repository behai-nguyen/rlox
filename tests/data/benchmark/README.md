<!-- Date Created: 01/08/2025. -->

**Please note:**

These scripts are from the author-provided benchmark suite:  
[https://github.com/munificent/craftinginterpreters/tree/master/test/benchmark](https://github.com/munificent/craftinginterpreters/tree/master/test/benchmark)

They are not included in automated integration tests. To run them manually, use the CLI application. For example:

```
cargo run ./tests/data/benchmark/equality.lox
```

# [Chapter 10](https://craftinginterpreters.com/functions.html)

The following scripts—`equality.lox`, `fib.lox`, and `string_equality.lox`—can be run after completing the code for Chapter 10. You can download this version using:

```
git clone -b v0.5.0 https://github.com/behai-nguyen/rlox.git
```

Note: These scripts may take a while to finish. See this [reference post](https://behainguyen.wordpress.com/2025/08/01/rlox-a-rust-implementation-of-crafting-interpreters-functions/#author-benchmarking-scripts) for more details.

### `equality.lox`

```
loop
41.23750400543213
elapsed
51.01371693611145
equals
9.776212930679321
```

### `fib.lox`

```
true
291.4927499294281
```

### `string_equality.lox`

```
loop
59.191839933395386
elapsed
63.91852593421936
equals
4.726686000823975
```

# [Chapter 13](https://craftinginterpreters.com/inheritance.html)

The remaining scripts require the full implementation of Chapter 13. You can download this version using:

```
git clone -b v0.6.1 https://github.com/behai-nguyen/rlox.git
```

or:

```
git clone https://github.com/behai-nguyen/rlox.git
```

to get the latest code version. I am listing their results below. 

### binary_trees.lox

```
stretch tree of depth:
15.0
check:
-1.0
num trees:
32768.0
depth:
4.0
check:
-32768.0
num trees:
8192.0
depth:
6.0
check:
-8192.0
num trees:
2048.0
depth:
8.0
check:
-2048.0
num trees:
512.0
depth:
10.0
check:
-512.0
num trees:
128.0
depth:
12.0
check:
-128.0
num trees:
32.0
depth:
14.0
check:
-32.0
long lived tree of depth:
14.0
check:
-1.0
elapsed:
209.54166388511658
```

### instantiation.lox

```
70.95994305610657
```

### invocation.lox

```
49.44964599609375
```

### method_call.lox

```
true
false
36.14567303657532
```

### properties.lox

```
88.21277093887329
```

### trees.lox

```
360.9396469593048
```

### zoo.lox

```
10000002.0
60.181785106658936
```

### zoo_batch.lox

```
1800000.0
30.0
10.22654104232788
```