## 控制测试运行方式

就像 `cargo run` 编译您的代码并运行生成的二进制文件一样，`cargo test` 以测试模式编译您的代码并运行生成的测试二进制文件。您可以指定命令行选项来更改 `cargo test` 的默认行为。例如，`cargo test` 生成的二进制文件的默认行为是并行运行所有测试并捕获测试运行期间生成的输出，防止输出被显示，从而更容易阅读与测试结果相关的输出。

一些命令行选项是给 `cargo test` 的，一些是给生成的测试二进制文件的。为了区分这两种参数，您可以列出给 `cargo test` 的参数，后跟分隔符 `--`，然后是给测试二进制文件的参数。运行 `cargo test --help` 显示您可以与 `cargo test` 一起使用的选项，运行 `cargo test -- --help` 显示您可以在分隔符 `--` 后使用的选项。

### 并行或顺序运行测试

当您运行多个测试时，默认情况下它们是使用线程并行运行的。这意味着测试会更快完成，因此您可以更快地获得代码是否工作的反馈。由于测试在同时运行，请确保您的测试不依赖于彼此或任何共享状态，包括共享环境，例如当前工作目录或环境变量。

例如，假设您的每个测试都运行一些代码，该代码在磁盘上创建一个名为 *test-output.txt* 的文件并向该文件写入一些数据。然后每个测试读取该文件中的数据并断言该文件包含一个特定的值，每个测试中的值不同。因为测试同时运行，一个测试可能在另一个测试写入和读取文件之间覆盖文件。这时第二个测试就会失败，并不是因为代码不正确，而是因为测试在并行运行时相互干扰。一个解决方案是确保每个测试写入不同的文件；另一个解决方案是一个接一个地运行测试。

如果您不希望并行运行测试或希望对使用的线程数量有更细粒度的控制，您可以将 `--test-threads` 标志和您想使用的线程数量传递给测试二进制文件。看看下面的例子：

```console
$ cargo test -- --test-threads=1
```

我们将测试线程的数量设置为 `1`，告知程序不使用并行性。使用一个线程运行测试会比并行运行测试花费更多时间，但如果多个测试共享状态，使用一个线程则不会相互干扰。

### 显示函数输出

默认情况下，如果测试通过，Rust 的测试库会捕获任何打印到标准输出的内容。例如，如果我们在测试中调用 `println!` 且测试通过，我们不会在终端中看到 `println!` 的输出；我们只会看到指示测试通过的那一行。如果测试失败，我们会看到打印到标准输出的内容和其他失败信息。

例如，下面的代码有一个简单的函数，它打印其参数的值并返回 10，还有一个通过的测试和一个失败的测试。

```rust,panics
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

##### 测试调用 `println!` 的函数

当我们使用 `cargo test` 运行这些测试时，我们会看到以下输出：

```text
running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

请注意，在这个输出中没有看到 `I got the value 4`，这是通过的测试运行时打印的内容。该输出已被捕获。失败测试的输出 `I got the value 8` 出现在测试摘要输出部分，该部分也显示了测试失败的原因。

如果我们希望看到通过测试的打印值，也可以让 Rust 在末尾显示成功测试的输出，只需使用 `--show-output`。

```text
$ cargo test -- --show-output
```

当我们在 "测试调用 `println!` 的函数"中再次运行带有 `--show-output` 标志的测试时，我们会看到以下输出：

```text
running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

successes:

---- tests::this_test_will_pass stdout ----
I got the value 4

successes:
    tests::this_test_will_pass

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

### 按名称运行部分测试

有时，运行完整的测试套件可能需要很长时间。如果您正在处理特定区域的代码，您可能希望只运行与该代码相关的测试。您可以通过将您希望运行的测试的名称作为参数传递给 `cargo test` 来选择运行哪些测试。

为了演示如何运行部分测试，我们将为我们的 `add_two` 函数创建三个测试，如下所示，并选择要运行的测试。

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

##### 三个名称不同的测试

如果在没有传递任何参数的情况下运行测试，就像之前看到的一样，所有测试将并行运行：

```text
running 3 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests running_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

#### 运行单个测试

我们可以将任何测试函数的名称传递给 `cargo test` 以仅运行该测试，例如，`cargo test one_hundred`：

```text
running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
```

只有名为 `one_hundred` 的测试运行了；其他两个测试与该名称不匹配。测试输出让我们知道运行的测试比这个命令运行的测试更多，因为在摘要行的末尾显示了 `2 filtered out`。

以这种方式，无法指定多个测试的名称；给 `cargo test` 的值中只有第一个会被使用。但有一种方法可以运行多个测试。

#### 过滤运行多个测试

我们可以指定测试名称的一部分，任何名称匹配该值的测试都会被运行。例如，因为我们的两个测试名称包含 `add`，所以可以通过运行 `cargo test add` 来运行这两个测试：

```text
running 2 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

此命令运行了名称中包含 `add` 的所有测试，并过滤掉了名为 `one_hundred` 的测试。还需注意，测试出现的模块名称成为测试名称的一部分，因此可以通过过滤模块名称来运行所有模块中的测试。

### 除非特别请求，否则忽略某些测试

有时一些特定的测试执行起来可能非常耗时，所以您可能希望在 `cargo test` 的大多数运行中排除它们。您可以通过 `ignore` 属性注释耗时的测试来排除它们，而不是列出所有想运行的测试作为参数，如下所示：

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // 需要一个小时才能运行的代码
}
```

在 `#[test]` 后添加 `#[ignore]` 行到要排除的测试中。现在当我们运行测试时，`it_works` 会运行，而 `expensive_test` 不会运行：

```text
running 2 tests
test expensive_test ... ignored
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out

   Doc-tests running_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

`expensive_test` 函数被列为 `ignored`。如果我们只想运行被忽略的测试，可以使用 `cargo test -- --ignored`：

```text
running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out

   Doc-tests running_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

通过控制运行哪些测试，您可以确保 `cargo test` 结果快速。当您处在一个合理检查 `ignored` 测试结果的阶段，并且有时间等待结果时，可以运行 `cargo test -- --ignored`。 

_您可以参考Rust编程语言书中的以下章节：[控制测试运行方式](https://doc.rust-lang.org/stable/book/ch11-02-running-tests.html)_