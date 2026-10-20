## 函数

函数在Rust代码中无处不在。您已经见过该语言中最重要的函数之一：`main`函数，它是许多程序的入口点。您还见过`fn`关键字，它允许您声明新函数。

Rust代码使用_蛇形命名法_作为函数和变量名的惯用风格。在蛇形命名法中，所有字母都是小写字母，并用下划线分隔单词。下面是一个包含函数定义示例的程序：

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Rust中的函数定义以`fn`开头，在函数名后跟一对括号。花括号告诉编译器函数体的开始和结束位置。

我们可以通过输入函数名后跟一对括号来调用我们定义的任何函数。因为`another_function`是在程序中定义的，所以可以从`main`函数内部调用。请注意，我们在源代码中是在`main`函数之后定义的`another_function`；也可以在之前定义。Rust不在乎你在哪里定义函数，只要它们被定义在某个地方即可。

让我们运行上面示例中的代码来进一步探索函数。将`another_function`示例放入src/main.rs并运行它。您应该会看到以下输出：

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28 secs
     Running `target/debug/functions`
Hello, world!
Another function.
```

这些行按照它们在`main`函数中出现的顺序执行。首先打印“Hello, world!”消息，然后调用`another_function`并打印其消息。