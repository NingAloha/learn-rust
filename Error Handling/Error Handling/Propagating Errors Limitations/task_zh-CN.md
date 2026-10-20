#### `?` 操作符只能用于返回 `Result` 的函数中

`?` 操作符只能用于返回类型为 `Result` 的函数中，因为它被定义为以类似于我们在含有错误匹配表达式的代码示例中的方式工作。`match` 需要 `Result` 返回类型的部分是 `return Err(e)`，因此函数的返回类型必须是 `Result` 以便与此 `return` 兼容。

让我们看看如果在 `main` 函数中使用 `?` 操作符会发生什么，您会记得它的返回类型是 `()`：

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt")?;
    }
```

当我们编译这段代码时，会收到以下错误消息：

```text
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `Try`)
 --> src/main.rs:4:13
  |
3 | / fn main() {
4 | |     let f = File::open("hello.txt")?;
  | |             ^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
5 | | }
  | |_- this function should return `Result` or `Option` to accept `?`
  |
  = help: the trait `Try` is not implemented for `()`
  = note: required by `from_error`
```

该错误指出，我们只能在返回 `Result` 或 `Option` 或其他实现 `std::ops::Try` 类型的函数中使用 `?` 操作符。当您在一个不返回这些类型之一的函数中编写代码，并且希望在调用返回 `Result<T, E>` 的其他函数时使用 `?`，此时您有两种选择来解决这个问题。一种技术是将函数的返回类型更改为 `Result<T, E>`，如果没有限制阻止这么做。另一种技术是使用 `match` 或 `Result<T, E>` 方法以适当的方式处理 `Result<T, E>`。

`main` 函数是特殊的，对其返回类型有特定限制。对于 `main` 的一个有效返回类型是 `()`，而另一个方便的有效返回类型是 `Result<T, E>`，如下所示：

```rust
    use std::error::Error;
    use std::fs::File;

    fn main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?;

        Ok(())
    }
```

`Box<dyn Error>` 类型被称为 _trait 对象_，在本章第17章的 [“使用允许不同类型值的 Trait 对象”](https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types) 部分中讨论。现在，您可以将 `Box<dyn Error>` 理解为“任何类型的错误”。在具有该返回类型的 `main` 函数中使用 `?` 是允许的。

现在我们已经讨论了调用 `panic!` 或返回 `Result` 的细节，让我们回到如何决定在哪种情况下使用哪种方式。

_您可以参考 Rust 编程语言图书中的以下章节：[可恢复错误与 Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#recoverable-errors-with-result)_