### 出错时的快捷提示：`unwrap` 和 `expect`

使用 `match` 表达式效果不错，但可能有些冗长，并且不总是能很好地传达意图。`Result<T, E>` 类型上定义了许多辅助方法，以执行各种任务。其中一种方法，称为 `unwrap`，是一种快捷方式，其实现方式与我们在“使用 match 表达式处理可能返回的 Result 变体”中编写的 `match` 表达式类似。如果 `Result` 值是 `Ok` 变体，`unwrap` 将返回 `Ok` 内的值。如果 `Result` 是 `Err` 变体，`unwrap` 将为我们调用 `panic!` 宏。以下是 `unwrap` 动作的示例：

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt").unwrap();
    }
```

如果我们在没有 _hello.txt_ 文件的情况下运行此代码，我们将看到由 `unwrap` 方法调用的 `panic!` 的错误信息：

```text
    thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
    repr: Os { code: 2, message: "No such file or directory" } }',
    src/libcore/result.rs:906:4
```

另一种方法，`expect`，与 `unwrap` 类似，允许我们选择 `panic!` 的错误信息。使用 `expect` 而不是 `unwrap` 并提供良好的错误信息，可以传达您的意图，并使追踪错误来源更容易。`expect` 的语法如下：

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }
```

我们使用 `expect` 的方式与 `unwrap` 相同：返回文件句柄或调用 `panic!` 宏。`expect` 在调用 `panic!` 时使用的错误信息将是传递给 `expect` 的参数，而不是 `unwrap` 使用的默认 `panic!` 信息。效果如下：

```text
    thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
    2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

因为此错误信息以我们指定的文本“Failed to open hello.txt”开头，因此更容易找到此错误信息来自代码的哪个位置。如果我们在多个地方使用 `unwrap`，可能需要更多时间来确切找出哪个 `unwrap` 导致了 panic，因为所有导致 panic 的 `unwrap` 调用都会打印相同的消息。