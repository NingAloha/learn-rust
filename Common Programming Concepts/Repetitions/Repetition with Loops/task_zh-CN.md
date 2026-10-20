## 使用循环重复

执行代码块多次通常是有用的。为此任务，Rust 提供了几种 _循环_。循环会遍历循环体内的代码直到结束，然后立即从开头重新开始。

Rust 有三种循环：`loop`、`while` 和 `for`。让我们尝试一下每一种。

### 使用 loop 重复代码

关键字 `loop` 告诉 Rust 重复执行一个代码块，直到你明确告诉它停止。

例如，将 src/main.rs 文件修改为如下：

```rust
fn main() {
     loop {
         println!("again!");
     }
 }
```

当我们运行这个程序时，我们将看到 `again!` 不断地被打印出来，直到我们手动停止程序。大多数终端支持键盘快捷键 <span class="keystroke">ctrl-c</span>，以中断陷入无限循环的程序：

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

符号 `^C` 表示你按下了 <span class="keystroke">ctrl-c</span>。你可能会看到或不看到在 `^C` 后打印出 `again!`，这取决于代码接收到中断信号时所在的循环位置。

幸运的是，Rust 提供了一个更可靠的方法来跳出循环。你可以在循环中放置 `break` 关键字，以告诉程序何时停止执行循环。

### 从循环返回值

`loop` 的一个用途是重试你知道可能会失败的操作，例如检查一个线程是否已完成其工作。然而，你可能需要将该操作的结果传递给你的其他代码。为此，你可以在停止循环时在 `break` 表达式之后添加你想返回的值；该值将从循环中返回，以便你可以使用它，如下所示：

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

在循环之前，我们声明了一个名为 `counter` 的变量，并将其初始化为 `0`。然后我们声明一个名为 `result` 的变量，用于存储从循环返回的值。在循环的每一次迭代中，我们在 `counter` 变量上加 `1`，然后检查 `counter` 是否等于 `10`。当它等于 `10` 时，我们使用 `break` 关键字加上值 `counter * 2`。循环结束后，我们用分号结束将值赋值给 `result` 的语句。最后，我们打印出 `result` 中的值，在此例中是 20。

_你可以参考 Rust 编程语言书中的以下章节：[使用循环重复](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#repetition-with-loops)_