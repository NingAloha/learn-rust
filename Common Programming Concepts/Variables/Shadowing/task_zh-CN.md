## 遮蔽

在 Rust 中，您可以声明一个与之前变量同名的新变量，新变量会遮蔽之前的变量。Rust 开发者称，第一个变量被第二个变量_遮蔽_，这意味着当使用这个变量时，出现的是第二个变量的值。我们可以通过使用相同变量名并重复使用 `let` 关键字来遮蔽一个变量，如下所示：

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

这个程序首先将 `x` 绑定到值 `5`。然后通过重复 `let x =` 来遮蔽 `x`，取原始值加 `1`，因此 `x` 的值变为 `6`。第三个 `let` 语句也遮蔽了 `x`，将前一个值乘以 `2`，使得 `x` 的最终值变为 `12`。当我们运行这个程序时，将输出以下内容：

```text
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/variables`
The value of x is: 12
```

遮蔽与将变量标记为 `mut` 不同，因为如果我们不使用 `let` 关键字而意外地尝试对该变量重新赋值，我们会得到编译时错误。通过使用 `let`，我们可以对一个值进行一些转换，但在这些转换完成后变量将是不可变的。

`mut` 和遮蔽之间的另一个区别是，当我们再次使用 `let` 关键字时，我们实际上是在创建一个新变量，因此我们可以更改值的类型，但重用相同的名称。例如，假设我们的程序要求用户通过输入空格字符来显示他们希望在一些文本之间有多少空格，但我们实际上希望将该输入存储为一个数字：

```rust
let spaces = "   ";
let spaces = spaces.len();
```

这个构造是允许的，因为第一个 `spaces` 变量是字符串类型，第二个 `spaces` 变量是一个全新的变量，碰巧与第一个变量同名，是数字类型。这样遮蔽使我们不必想出不同的名称，如 `spaces_str` 和 `spaces_num`，而可以重用更简单的 spaces 名称。然而，如果我们尝试为此使用 `mut`，如下所示，我们将得到编译时错误：

```rust
let mut spaces = "   ";
spaces = spaces.len();
```

错误信息说我们不允许更改变量的类型：

```text
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected &str, found usize
  |
  = note: expected type `&str`
             found type `usize`
```

_您可以参考 Rust 程序设计语言书中的以下章节：[遮蔽](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html#shadowing)_

既然我们已经了解了变量的工作原理，接下来让我们在实践中应用我们的知识。