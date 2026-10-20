#### 传播错误的捷径：? 运算符

以下代码片段显示了 `read_username_from_file` 的实现，其功能与前一个示例中相同，但此实现使用了 `?` 运算符。

```rust
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
```

##### 使用 ? 运算符向调用代码返回错误的函数

放置在 `Result` 值后的 `?` 运算符的工作方式与我们定义的处理 `Result` 值的 `match` 表达式几乎相同。如果 `Result` 的值是 `Ok`，则返回表达式中的 `Ok` 内部值，并且程序将继续执行。如果值是 `Err`，则 `Err` 将从整个函数中返回，就像使用了 `return` 关键字一样，因此错误值会传播到调用代码。

这个示例中的 `match` 表达式与 `?` 运算符之间有一个区别：调用 `?` 运算符的错误值将通过标准库中 `From` 特性中定义的 `from` 函数，该函数用于将错误从一种类型转换为另一种类型。当 `?` 运算符调用 `from` 函数时，接收到的错误类型会被转换为当前函数返回类型中定义的错误类型。当函数要返回一种错误类型以表示函数可能失败的所有途径时，即使某些部分可能因多种不同原因而失败，这会很有用。只要每种错误类型都实现了 `from` 函数并定义如何将自身转换为返回的错误类型，`?` 运算符便可自动处理转换。

在最后一个代码片段中，在 `File::open` 调用后的 `?` 操作符会将 `Ok` 中的值返回给变量 `f`。发生错误时，`?` 运算符会快速返回整个函数，并将任何 `Err` 值传递给调用代码。同样的情况也适用于 `read_to_string` 调用后的 `?` 运算符。

`?` 运算符消除了许多样板代码，使得此函数的实现更简单。我们甚至可以通过在 `?` 之后立即链接方法调用来进一步缩短此代码，正如以下示例所示。

```rust
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }
```

##### 在 ? 运算符后链接方法调用

我们将新 `String` 的创建移动到函数开始，部分没有变化。我们没有创建变量 `f`，而是直接将 `read_to_string` 调用链接到 `File::open("hello.txt")?` 的结果。我们仍然在 `read_to_string` 调用的末尾使用了 `?`，并且在 `File::open` 和 `read_to_string` 均成功时返回包含用户名的 `Ok` 值，而不是返回错误。功能与前面的示例相同，这只是书写它的另一种更合适的方式。

说到书写此函数的不同方式，以下代码片段显示了一种使其更短的方法。

```rust
    use std::io;
    use std::fs;

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
```

##### 使用 fs::read_to_string 代替打开然后读取文件

将文件内容读取到字符串中是一个相当常见的操作，所以 Rust 提供了便捷的 `fs::read_to_string` 函数，该函数会打开文件，创建一个新的 `String`，读取文件的内容，将内容放入该 `String` 中并将其返回。当然，使用 `fs::read_to_string` 无法让我们有机会解释所有的错误处理，所以我们首先使用了较长的方法。