### 用 `spawn` 创建新线程

要创建一个新线程，我们调用 `thread::spawn` 函数，并传递一个闭包（我们在第13章讨论过闭包），其中包含我们希望在新线程中运行的代码。下面代码片段中的示例从主线程打印一些文本，并从新线程打印其他文本：

```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
```

##### 创建一个新线程来打印某些内容，同时主线程打印其他内容

注意，使用该函数时，无论新线程是否运行完毕，只要主线程结束，新线程就会被停止。这个程序的输出每次可能会有所不同，但通常看起来类似如下：

```text
    hi number 1 from the main thread!
    hi number 1 from the spawned thread!
    hi number 2 from the main thread!
    hi number 2 from the spawned thread!
    hi number 3 from the main thread!
    hi number 3 from the spawned thread!
    hi number 4 from the main thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!
```

调用 `thread::sleep` 使线程暂停执行一小段时间，允许其他线程运行。线程可能会轮流执行，但这不是保证的：这取决于操作系统如何调度线程。在这次运行中，主线程首先打印，即使代码中启动线程的打印语句先出现。尽管我们让启动线程打印直到 `i` 为9，但它只打印到5，因为主线程关闭了。

如果你运行这个代码时只看到来自主线程的输出，或者看不到任何重叠，试着增大范围中的数字，以创建更多机会让操作系统在线程之间切换。