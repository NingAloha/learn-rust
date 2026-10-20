## 返回值的函数

函数可以返回值给调用它们的代码。我们不为返回值命名，但我们在箭头 `(->)` 后声明它们的类型。在 Rust 中，函数的返回值与函数体内块的最后一个表达式的值是同义的。你可以使用 `return` 关键字及指定一个值来提前返回，但大多数函数会隐式返回最后一个表达式。以下是一个返回值的函数示例：

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("x 的值是: {}", x);
}
```

在 `five` 函数中没有函数调用、宏，甚至没有 `let` 语句—只有数字 5 自身。这在 Rust 中是一个完全有效的函数。注意函数的返回类型也被指定为 `-> i32`。尝试运行这个代码，输出应如下所示：

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/functions`
x 的值是: 5
```

`five` 中的 `5` 是函数的返回值，这就是返回类型为 `i32` 的原因。让我们详细检查一下这一点。有两个重要的点：首先，这行 `let x = five();` 表明我们在使用函数的返回值来初始化一个变量。因为函数 `five` 返回 `5`，所以这一行等价于：

```rust
let x = 5;
```

其次，`five` 函数没有参数并定义了返回值的类型，但函数体中只有一个孤独的 `5`，没有分号，因为这是一个我们想返回其值的表达式。

我们来看另一个例子：

```rust
fn main() {
    let x = plus_one(5);

    println!("x 的值是: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

运行这种代码会打印 `x 的值是: 6`。但如果我们在包含 `x + 1` 的那一行末加上一个分号，将其从表达式改为语句，就会得到一个错误。

```rust
fn main() {
    let x = plus_one(5);

    println!("x 的值是: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

编译这段代码会产生一个错误，如下所示：

```text
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    默认返回 `()` 因为其主体没有末尾或 `return` 表达式
8 |     x + 1;
  |          - help: 考虑移除此分号
```

主要的错误信息,“mismatched types”(类型不匹配)，揭示了这段代码的核心问题。`plus_one` 函数的定义声称它将返回 `i32`，但语句不会计算为一个值，这由 `()` 表示，一个空元组。因此，没有返回任何值，这与函数定义相冲突，结果导致错误。在这输出中，Rust 提供了一个消息来可能帮助解决这个问题：它建议移除分号，这将修复错误。

_您可以参考 Rust 程序设计语言书中的以下部分: [返回值的函数](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html#functions-with-return-values)_。