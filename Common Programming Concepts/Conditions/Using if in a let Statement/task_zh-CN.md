## 在 let 语句中使用 if

因为 `if` 是一个表达式，我们可以在 `let` 语句的右侧使用它，如下所示：

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("number 的值是: {}", number);
}
```
##### 将 if 表达式的结果赋值给变量的例子

变量 `number` 将根据 `if` 表达式的结果绑定到一个值。运行这段代码看看会发生什么：

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/branches`
number 的值是: 5
```

请记住，代码块将被计算为其中的最后一个表达式，数字本身也是一种表达式。在这个例子中，整个 `if` 表达式的值取决于哪个代码块被执行。这意味着来自 `if` 的每个分支的结果值必须具有相同的类型；在之前的代码片段中，`if` 分支和 `else` 分支的结果都是 `i32` 整数。如果类型不匹配，如下面的例子，我们将会得到一个错误：

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("number 的值是: {}", number);
}
```

当我们尝试编译这段代码时，将会出现一个错误。`if` 和 `else` 分支具有不兼容的值类型，Rust 会精确地指出程序中的问题位置：

```text
error[E0308]: `if` 和 `else` 具有不兼容的类型
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ 预期整数类型，却找到了 `&str`
  |                                 |
  |                                 因为这个而预期
```

`if` 块中的表达式计算为一个整数，而 `else` 块中的表达式计算为一个字符串。这行不通，因为变量必须具有单一类型。Rust 需要在编译时明确知道变量 `number` 的类型，以便在编译时验证 `number` 使用位置的类型是有效的。如果 `number` 的类型只在运行时确定，那么编译器将会更加复杂，并且对代码的保证会更少，因为它不得不跟踪任何变量的多种假设类型。

_你可以在 Rust 编程语言书的以下章节中参考：[Using if in a let Statement](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#using-if-in-a-let-statement)_