### 使用`panic!`的回溯

让我们看看另一个例子，了解当来自库的`panic!`调用是由于我们代码中的错误，而不是因为我们的代码直接调用宏时，会发生什么。下面的代码片段中尝试通过索引访问向量中的一个元素。

```rust
    fn main() {
        let v = vec![1, 2, 3];

        v[99];
    }
```

##### 尝试访问向量末尾之外的元素，这将导致调用panic！

在这里，我们尝试访问向量的第100个元素（索引为99，因为索引从零开始），但是它只有3个元素。在这种情况下，Rust会产生恐慌。使用`[]`应当返回一个元素，但是如果传递了一个无效的索引，Rust在这里无法返回一个正确的元素。

其他语言，如C，会尝试在这种情况下给你请求的东西，即使这并不是你需要的：你会得到内存中对应于向量该元素位置的内容，尽管该内存并不属于该向量。这被称为_缓冲区过度读取_，如果攻击者能够操控索引以读取他们不应被允许读取的数据，这可能会导致安全漏洞。

为了保护您的程序不受这种漏洞的影响，如果您尝试读取一个不存在的索引的元素，Rust会停止执行并拒绝继续。我们来试试看：

```text
   Compiling test_rust_project v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/test_rust_project`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /rustc/3c235d5600393dfe6c36eeed34042efad8d4f26e/src/libcore/slice/mod.rs:2686:10
```
这个错误指向了一个我们没有编写的文件，_libcore/slice/mod.rs_。那是Rust源代码中`slice`的实现。当我们对向量`v`使用`[]`时执行的代码在_libcore/slice/mod.rs_中，那就是`panic!`实际发生的地方。

接下来提示的行告诉我们，可以设置`RUST_BACKTRACE`环境变量以获取导致错误的详细回溯。_回溯_是所有被调用的函数列表，以到达这个点。Rust中的回溯工作原理与其他语言类似：阅读回溯的关键是从顶部开始阅读，直到看到您编写的文件。那是问题起源的地方。提到您文件的行上方的行是您的代码调用的代码；下面的行是调用您的代码的代码。这些行可能包含核心Rust代码、标准库代码或您正在使用的包。我们尝试通过将`RUST_BACKTRACE`环境变量设置为除0以外的任何值来获取回溯。下面的代码片段显示了类似您将看到的输出。

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:483
   1: core::panicking::panic_fmt
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:85
   2: core::panicking::panic_bounds_check
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:62
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:255
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:15
   5: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/vec.rs:1982
   6: panic::main
             at ./src/main.rs:4
   7: core::ops::function::FnOnce::call_once
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

##### 当环境变量`RUST_BACKTRACE`被设置时，由调用panic!生成的回溯

这是一大段输出！您看到的确切输出可能会根据您的操作系统和Rust版本的不同而有所不同。为了用这些信息获取回溯，必须启用调试符号。使用`cargo build`或`cargo run`而不带`--release`标志时，调试符号默认被启用，就像我们在这里做的那样。

在上面代码片段的输出中，回溯的第6行指向我们项目中导致问题的那一行：_src/main.rs_的第4行。如果我们不希望程序产生恐慌，那么提到我们编写的第一个文件的行就是我们应该开始调查的问题所在。在第一个代码片段中，我们故意编写了会产生恐慌的代码，以演示如何使用回溯，修复恐慌的方法是不从仅包含3个项目的向量中请求索引99的元素。当您的代码在将来产生恐慌时，您需要找出代码在用什么值做什么动作导致了恐慌，以及代码应该做些什么来避免恐慌。

我们将在本章后面[“是使用`panic!`还是不使用`panic!`”](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic)部分中再次讨论`panic!`以及什么时候应该和不应该用`panic!`来处理错误条件。接下来，我们将看看如何使用`Result`从错误中恢复。

_您可以参考Rust编程语言书中的以下章节：
[无法恢复的错误与panic!](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unrecoverable-errors-with-panic)_