## 可恢复的错误与Result

大多数错误并不严重到需要让程序完全停止。有时候，当一个函数失败时，原因是可以轻松解释和处理的。例如，如果尝试打开一个文件，而操作失败是因为文件不存在，您可能希望创建文件而不是终止进程。

`Result` 枚举被定义为拥有两个变体，`Ok` 和 `Err`，如下所示：

```rust
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
```

`T` 和 `E` 是泛型类型参数：在课程“泛型类型、特征和生命周期”中我们会更详细地讨论它们。现在您需要知道的是，`T` 代表在 `Ok` 变体中成功情况下返回的值的类型，而 `E` 代表在 `Err` 变体中失败情况下返回的错误的类型。由于 `Result` 拥有这些泛型类型参数，我们可以在不同情况下使用 `Result` 类型以及标准库为其定义的函数，无论我们想返回的成功值和错误值有什么不同。

让我们调用一个返回 `Result` 值的函数，因为该函数可能会失败。以下代码片段中我们尝试打开一个文件。

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt");
    }
```

##### 打开文件

我们如何知道 `File::open` 返回一个 `Result`？我们可以查看[标准库 API 文档](https://doc.rust-lang.org/std/index.html)，也可以询问编译器！如果我们给 `f` 一个我们知道不是函数返回类型的类型注解，然后尝试编译代码，编译器会告诉我们类型不匹配。错误信息会告诉我们 `f` 的类型。我们来试一下！我们知道 `File::open` 的返回类型不是 `u32`，所以让我们把 `let f` 语句改成这样：

```rust
    let f: u32 = File::open("hello.txt");
```

尝试编译后我们得到以下输出：

```text
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |            ---   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found enum `std::result::Result`
  |            |
  |            expected due to this
  |
  = note: expected type `u32`
             found enum `std::result::Result<File, std::io::Error>`
```

这告诉我们 `File::open` 函数的返回类型是 `Result<T, E>`。这里泛型参数 `T` 被填充成成功值的类型，`std::fs::File`，这是一个文件句柄。用在错误值中的 `E` 类型是 `std::io::Error`。

这种返回类型意味着调用 `File::open` 可能成功并返回一个文件句柄，我们可以从中读取或写入。函数调用也可能失败：例如，文件可能不存在，或者我们可能没有访问文件的权限。`File::open` 函数需要一种方式告诉我们它是否成功或失败，同时给我们文件句柄或错误信息。这正是 `Result` 枚举所传达的信息。

在 `File::open` 成功的情况下，变量 `f` 中的值将是一个包含文件句柄的 `Ok` 实例。在失败的情况下，`f` 中的值将是包含有关所发生错误类型的信息的 `Err` 实例。

我们需要在上述代码片段中添加代码，以根据 `File::open` 返回的值采取不同的操作。以下代码片段展示了一种使用基本工具 `match` 表达式处理 `Result` 的变体的方法，我们在["Match 运算符"](course://Enums/Enums and Pattern Matching/The Match Operator)中讨论过。

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("打开文件的问题: {:?}", error),
    };
}
```

##### 使用match表达式处理可能返回的Result变体

请注意，正如 `Option` 枚举一样，`Result` 枚举及其变体已通过导入默认引入到作用域中，因此我们不需要在 `match` 分支中的 `Ok` 和 `Err` 变体之前指定 `Result::`。

这里我们告诉 Rust，当结果是 `Ok` 时，从 `Ok` 变体中返回内部的 `file` 值，然后我们将该文件句柄值赋给变量 `f`。在 `match` 结束后，我们可以用该文件句柄进行读或写操作。

`match` 的另一个分支处理从 `File::open` 获取 `Err` 值的情况。在这个例子中，我们选择调用 `panic!` 宏。如果在当前目录下没有名为 hello.txt 的文件并且我们运行这段代码，我们将看到来自 `panic!` 宏的以下输出：

```text
thread 'main' panicked at '打开文件的问题: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
```

像往常一样，这个输出告诉我们问题出在哪里。