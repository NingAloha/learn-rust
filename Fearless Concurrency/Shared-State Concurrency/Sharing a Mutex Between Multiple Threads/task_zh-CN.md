#### 在多个线程之间共享 Mutex<T>

现在，让我们尝试使用 `Mutex<T>` 在多个线程之间共享一个值。我们将启动 10 个线程，每个线程将计数器的值增加 1，因此计数器从 0 增加到 10。接下来的示例会出现编译错误，我们将利用该错误深入了解 `Mutex<T>` 的使用以及 Rust 如何帮助我们正确使用它。以下代码是我们的初始示例：

```rust
    use std::sync::Mutex;
    use std::thread;

    fn main() {
        let counter = Mutex::new(0);
        let mut handles = vec![];

        for _ in 0..10 {
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("结果: {}", *counter.lock().unwrap());
    }
```

##### 十个线程每个都对由 Mutex<T> 保护的计数器进行递增

我们创建了一个 `counter` 变量，将一个 `i32` 放在 `Mutex<T>` 中，正如我们在第一个示例中所做的那样。接下来，通过迭代一系列数字，我们创建了 10 个线程。我们使用 `thread::spawn`，并将所有线程都赋予相同的闭包，该闭包将计数器移入线程中，通过调用 `lock` 方法获取 `Mutex<T>` 的锁，然后将互斥锁中的值加 1。当一个线程完成其闭包运行后，`num` 会超出其作用域并释放该锁，这样另一个线程就可以获取它。

在主线程中，我们收集所有连接句柄。然后我们调用每个句柄的 `join`，确保所有线程全部完成。此时，主线程将获取锁并打印这个程序的结果。

我们暗示此示例不会编译。现在让我们看看为什么！

```text
error[E0382]: use of moved value: `counter`
  --> src/main.rs:9:36
   |
5  |     let counter = Mutex::new(0);
   |         ------- 出现移动因为 `counter` 的类型是 `Mutex<i32>`，它没有实现 `Copy` 特征
...
9  |         let handle = thread::spawn(move || {
   |                                    ^^^^^^^ 值在这里移动到闭包中，在循环的上一次迭代中
10 |             let mut num = counter.lock().unwrap();
   |                           ------- 因为在闭包中使用而发生使用
```

错误消息表明 `counter` 值在循环的前一次迭代中被移动了。因此 Rust 告诉我们不能将锁 `counter` 的所有权移动到多个线程中。让我们使用 Rust 书中[第 15 章](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)中讨论的多重所有权方法来修复编译错误。