## 如何编写测试

测试是 Rust 函数，用于验证非测试代码是否按预期运行。测试函数的主体通常执行以下三个操作：

1. 设置任何所需的数据或状态。
2. 运行你要测试的代码。
3. 断言结果是你所期望的。

让我们看看 Rust 专门为执行这些操作而编写测试所提供的功能，这些功能包括 `test` 属性、一些宏以及 `should_panic` 属性。

### 测试函数的结构

在最简单的情况下，Rust 中的测试是一个用 `test` 属性注解的函数。属性是关于 Rust 代码片段的元数据；一个例子是我们在第五章与结构体一起使用的 `derive` 属性。要将一个函数变成测试函数，需要在 `fn` 之前的行中添加 `#[test]`。当你使用 `cargo test` 命令运行测试时，Rust 会构建一个测试运行程序，运行那些用 `test` 属性标记的函数，并报告每个测试函数是否通过。

当我们使用 Cargo 创建一个新的库项目时，系统为我们自动生成一个包含测试函数的测试模块。这个模块帮助你开始编写测试，这样每次启动新项目时就不必查找测试函数的具体结构和语法。你可以根据需要添加任意数量的测试函数和测试模块！

我们将通过实验自动生成的测试模板来探索测试如何工作，而不实际测试任何代码。然后我们将编写一些调用我们编写的代码并断言其行为正确的真实测试。

让我们修改 _src/lib.rs_ 文件。其内容应如下代码片段所示。

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

##### cargo new 自动生成的测试模块和函数示例

目前，让我们忽略前两行，重点关注这个函数以了解它如何工作。注意 `fn` 行之前的 `#[test]` 注解：这个属性表明这是一个测试函数，因此测试运行程序知道将这个函数视为测试。我们也可以在 `tests` 模块中添加非测试函数，以帮助设置通用情境或执行常见操作，因此我们需要通过使用 `#[test]` 属性来表明哪些函数是测试。

函数主体使用 `assert_eq!` 宏来断言 2 + 2 等于 4。这个断言作为典型测试格式的示例。让我们运行它以确保此测试通过。

右键点击“如何编写测试”任务并选择**在终端打开**，然后运行 `cargo test` 命令。你会看到类似于下面所示的输出。

```text
$ cargo test
  Compiling how_to_write_tests v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

##### 运行自动生成测试的输出示例

Cargo 编译并运行了测试。在 `Compiling`、`Finished` 和 `Running` 行之后是 `running 1 test` 行。下一行显示生成的测试函数名称，称为 `it_works`，以及运行该测试的结果，`ok`。接下来出现的是运行测试的整体摘要。`test result: ok.` 这部分说明所有测试都通过了，而 `1 passed; 0 failed` 部分统计了通过或失败的测试数。

因为我们没有任何标记为忽略的测试，摘要中显示 `0 ignored`。我们也没有过滤正在运行的测试，所以摘要末尾显示 `0 filtered out`。我们将在“运行测试”部分谈论忽略和过滤掉测试。

`0 measured` 统计的是测试基准性能的基准测试。基准测试，目前仅在 Rust 的 nightly 版本中可用。查看[关于基准测试的文档](https://doc.rust-lang.org/unstable-book/library-features/test.html)了解更多信息。

测试输出的下一部分，从 `Doc-tests how_to_write_tests` 开始，是关于任何文档测试的结果。我们目前还没有任何文档测试，但 Rust 可以编译出现在 API 文档中的任何代码示例。此功能帮助我们保持文档和代码同步！我们将在 Rust 书籍第 14 章的[“文档注释作为测试”](https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests)部分讨论如何编写文档测试。现在，我们将忽略 `Doc-tests` 输出。

让我们更改测试的名称，看看这如何会改变测试输出。将 `it_works` 函数更改为其他名称，如 `exploration`，如下所示：

```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn exploration() {
            assert_eq!(2 + 2, 4);
        }
    }
```

然后再次运行 `cargo test`。输出现在显示 `exploration` 而不是 `it_works`：

```text
Compiling how_to_write_tests v0.1.0
   Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

让我们添加另一个测试，但这次我们将编写一个会失败的测试！当测试函数中的某些内容引发 panic 时，测试就会失败。每个测试都在一个新线程中运行，当主线程看到一个测试线程已死亡时，测试被标记为失败。我们在第 9 章讨论过导致 panic 的最简单方法，就是调用 `panic!` 宏。输入新的测试 `another`，使你的 _src/lib.rs_ 文件看起来像下面的代码片段。

```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn exploration() {
            assert_eq!(2 + 2, 4);
        }

        #[test]
        fn another() {
            panic!("Make this test fail");
        }
    }
```

##### 添加一个因调用 panic! 宏而会失败的第二个测试示例

再次使用 `cargo test` 运行测试。输出应如下所示，它显示我们的 `exploration` 测试通过了，而 `another` 失败了。
```text
Compiling how_to_write_tests v0.1.0
   Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
thread 'tests::another' panicked at 'Make this test fail', Writing Automated Tests/Tests/How to Write Tests/src/lib.rs:9:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

##### 当一个测试通过而一个测试失败时的测试结果示例

不再是 `ok`，`test tests::another` 的行显示为 `FAILED`。在各个结果和摘要之间出现两个新部分：第一部分显示每个测试失败的详细原因。在此示例中，`another` 失败是因为它在 _src/lib.rs_ 文件中的第 10 行 “panicked at "Make this test fail"”。下一部分仅列出所有失败测试的名称，这在有大量测试和大量详细失败测试输出时很有用。我们可以使用失败测试的名称来单独运行该测试以更轻松地调试它；可以在“运行测试”部分找到有关运行测试的方法的更多信息。

摘要行显示在末尾：总体上，我们的测试结果是 `FAILED`。我们有一个测试通过，一个测试失败。

现在你已经看到了不同场景下的测试结果，让我们看看在测试中除了 `panic!` 之外的一些有用的宏。