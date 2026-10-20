## 用 panic! 处理不可恢复的错误

有时候，代码中会发生不好的事情，而且你无法做出任何处理。在这种情况下，Rust 提供了 `panic!` 宏。当 `panic!` 宏执行时，程序会打印一条失败信息，回退并清理堆栈，然后退出。最常见的是，当程序检测到某种错误且程序员不清楚如何处理时，就会发生这种情况。

> ### 在 panic 响应中回溯堆栈或直接中止
>
> 默认情况下，当发生 panic 时，程序开始_回溯_，这意味着 Rust 会逐层回退堆栈并清理遇到的每个函数的数据。但这种回退和清理需要做很多工作。替代方案是立即_中止_程序，这样程序不会进行清理。程序使用的内存随后需要由操作系统来清理。如果你的项目需要生成尽可能小的二进制文件，可以通过在 _Cargo.toml_ 文件的相应 `[profile]` 部分添加 `panic = 'abort'` 来将 panic 时的行为从回溯切换为中止。例如，如果要在 release 模式下遇到 panic 时中止，请添加以下内容：
>
>     [profile.release]
>     panic = 'abort'

让我们尝试在一个简单的程序中调用 `panic!`：

```rust
    fn main() {
        panic!("crash and burn");
    }
```

当你运行程序时，你会看到如下信息：

```text
  Compiling test_rust_project v0.1.0
      Finished dev [unoptimized + debuginfo] target(s) in 0.42s
       Running `target/debug/test_rust_project`
  thread 'main' panicked at 'crash and burn', src/main.rs:2:5
```

调用 `panic!` 导致最后两行中包含错误信息。第一行显示了我们的 panic 消息以及源代码中发生 panic 的位置：_src/main.rs:2:5_ 表示在我们的 _src/main.rs_ 文件的第二行，第五个字符。

在此情况下，被标记的行是我们代码的一部分，并且如果我们查看该行，会看到 `panic!` 宏调用。在其他情况下，`panic!` 调用可能在我们的代码调用的代码中发生，错误信息报告的文件名和行号将是某人编写的代码，而不是最终导致 `panic!` 调用的那行代码。我们可以使用 `panic!` 调用所在函数的回溯来找出导致问题的代码部分。接下来我们会更详细地讨论什么是回溯。