## 条件

决定是否根据某个条件为真来运行某些代码是大多数编程语言中的基本构建块。控制 Rust 代码执行流程的最常见结构是 `if` 表达式。

### if 表达式

`if` 表达式允许您根据条件对代码进行分支。您提供一个条件，然后说明，“如果满足该条件，运行这段代码块。如果条件不满足，则不运行这段代码块。”

在 _src/main.rs_ 文件中输入以下内容：

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

所有 `if` 表达式都以关键字 `if` 开始，后面跟着一个条件。在这个例子中，条件检查变量 `number` 是否小于 5。如果条件为真，我们希望执行的代码块紧接在条件后面的花括号内。与 `if` 表达式中的条件相关联的代码块有时称为分支，就像 `match` 表达式中的分支一样。

我们还可以选择包含一个 `else` 表达式，我们在这里选择这样做，以便在条件计算为假时让程序执行另一个代码块。如果没有提供 `else` 表达式且条件为假，程序将跳过 `if` 块并继续执行下面的代码。

尝试运行这段代码，您应该看到以下输出：

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
condition was true
```

让我们尝试将 `number` 的值更改为一个使条件为假的值，看看会发生什么：

```rust
let number = 7;
```

再次运行程序，并查看输出：

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
condition was false
```

还值得注意的是，代码中的条件必须是一个 `bool`。如果条件不是 `bool`，我们将会得到一个错误。例如，尝试运行以下代码：

```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

这次 `if` 条件计算为 `3`，Rust 会报错：

```text
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

error: aborting due to previous error
```

错误表明 Rust 预期的是一个 `bool`，但得到的是一个整数。与 Ruby 和 JavaScript 等语言不同，Rust 不会自动尝试将非布尔类型转换为布尔类型。您必须明确并始终为 `if` 提供一个布尔值作为条件。如果我们希望 `if` 代码块仅在数字不等于 `0` 时运行，例如，可以将 `if` 表达式更改为以下内容：

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

运行这段代码将打印 `number was something other than zero`。

_您可以参考 Rust 编程语言书的以下章节：[控制流 - if 表达式](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#if-expressions)_