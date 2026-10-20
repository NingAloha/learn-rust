### 匹配不同的错误

接下来的代码示例中，无论 `File::open` 失败的原因是什么，程序都会 `panic!`。我们真正想实现的是针对不同的失败原因采取不同的操作：如果 `File::open` 是因为文件不存在而失败，我们希望创建该文件并返回新文件的句柄。如果 `File::open` 因其他任何原因失败——例如因为我们没有权限打开该文件——我们还是希望代码像前面的片段中那样 `panic!`。看看下面的代码，它添加了一个内部 `match` 表达式。

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建文件时出现问题: {:?}", e),
            },
            other_error => {
                panic!("打开文件时出现问题: {:?}", other_error)
            }
        },
    };
}
```

##### 以不同方式处理不同类型的错误

`File::open` 返回的 `Err` 变体值的类型是 `io::Error`，这是标准库提供的一个结构体。该结构体有一个方法 `kind`，我们可以调用它来获取一个 `io::ErrorKind` 值。枚举 `io::ErrorKind` 由标准库提供，具有表示可能由 `io` 操作导致的不同类型错误的变体。我们想用的变体是 `ErrorKind::NotFound`，这表明我们尝试打开的文件还不存在。因此我们匹配 `f`，同时对 `error.kind()` 进行内部匹配。

我们想在内部匹配中检查的条件是返回的值是否为 `ErrorKind` 枚举的 `NotFound` 变体。如果是，我们就尝试使用 `File::create` 创建文件。然而，由于 `File::create` 也可能失败，我们需要在内部 `match` 表达式中添加一个第二个分支。当文件无法创建时，会打印不同的错误消息。外部 `match` 的第二个分支保持不变，因此程序在除文件丢失错误以外的任何错误时都会崩溃。

这有很多 `match`！`match` 表达式非常有用，但也是很基础的。在 ["Standard Library Types/Closures"](course://Standard Library Types/Closures) 中，你将学习关于闭包的内容；`Result<T, E>` 类型有许多接受闭包的方法，并使用 `match` 表达式实现。使用这些方法可以让你的代码更简洁。一位经验丰富的 Rust 开发者可能会这样编写代码，而不是之前的示例：

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("创建文件时出现问题: {:?}", error);
            })
        } else {
            panic!("打开文件时出现问题: {:?}", error);
        }
    });
}
```

虽然这段代码与前一段的行为相同，但它不包含任何 `match` 表达式，看起来更简洁。在你阅读 ["Standard Library Types/Closures"](course://Standard Library Types/Closures) 之后，回来看看这个示例，并在标准库文档中查找 `unwrap_or_else` 方法。还有更多这样的方法可以在处理错误时清理大量嵌套的 `match` 表达式。