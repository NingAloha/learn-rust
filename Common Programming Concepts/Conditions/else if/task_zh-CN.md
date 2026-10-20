## 使用 else if 处理多重条件

您可以通过在 `else if` 表达式中组合 `if` 和 `else` 来处理多重条件。例如：

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number 可以被 4 整除");
    } else if number % 3 == 0 {
        println!("number 可以被 3 整除");
    } else if number % 2 == 0 {
        println!("number 可以被 2 整除");
    } else {
        println!("number 不能被 4、3 或 2 整除");
    }
}
```

这个程序有四种可能的路径可以走。运行后，您应该看到以下输出：

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
number 可以被 3 整除
```

当这个程序执行时，它会依次检查每个 `if` 表达式，并执行第一个条件为真的代码块。请注意，即使 6 可以被 2 整除，我们也不会看到输出“number 可以被 2 整除”，也不会看到 `else` 块的“number 不能被 4、3 或 2 整除”文本。这是因为 Rust 只执行第一个为真的条件对应的代码块，一旦找到一个，就不会检查剩下的条件。

使用过多的 `else if` 表达式可能会使代码复杂化，因此如果您有多个 `else if`，您可能需要重构您的代码。[第 6 章](https://doc.rust-lang.org/stable/book/ch06-00-enums.html) 描述了一种用于这些情况的强大的 Rust 分支结构，称为 `match`。

_您可以参考 Rust 编程语言书籍中的以下章: [使用 else if 处理多重条件](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#handling-multiple-conditions-with-else-if)_