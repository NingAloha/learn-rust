### 发送多个值并查看接收器等待

关于在主线程接收“hi”的代码片段编译并运行，但它没有清楚地向我们展示两个独立的线程通过通道互相通信。在下面的代码片段中，我们进行了一些修改，以证明代码是并发运行的：创建的新线程现在将发送多条消息，并在每条消息之间暂停一秒钟。

```rust
    use std::thread;
    use std::sync::mpsc;
    use std::time::Duration;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
```

##### 发送多条消息并在每条之间暂停

这一次，创建的新线程有一个要发送到主线程的字符串向量。我们遍历它们，逐个发送，并通过调用 `thread::sleep` 函数并传入 1 秒 `Duration` 的值来在每个之间暂停。

在主线程中，我们不再显式调用 `recv` 函数：而是将 `rx` 视为一个迭代器。对于每个接收到的值，我们打印它。当通道关闭时，迭代将结束。

运行上一个示例中的代码时，您应该看到如下输出，每行之间暂停 1 秒：

```text
    Got: hi
    Got: from
    Got: the
    Got: thread
```

因为我们在主线程的 `for` 循环中没有任何暂停或延迟的代码，所以可以判断主线程正在等待从新线程接收值。