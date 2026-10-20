### 通过克隆发送端创建多个生产者

之前我们提到过，`mpsc` 是 _多个生产者，单个消费者_ 的缩写。让我们利用 `mpsc` 并扩展先前代码段中的代码，以创建多个线程，所有这些线程都将值发送到同一个接收端。我们可以通过克隆通道的发送端来做到这一点，如下所示：

```rust
    // --代码省略--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // --代码省略--
```

##### 多个生产者发送多个消息

这一次，在我们创建第一个生成线程之前，我们在通道的发送端调用 `clone`。这样可以让我们获得一个新的发送句柄，可以传给第一个生成线程。我们将通道的原始发送端传递给第二个生成线程。这为我们提供了两个线程，每个线程都向通道的接收端发送不同的消息。

运行代码后，输出应类似如下：

```text
    Got: hi
    Got: more
    Got: from
    Got: messages
    Got: for
    Got: the
    Got: thread
    Got: you
```

您可能会看到值的顺序不同；这取决于您的系统。这也正是并发既有趣又困难的地方。如果您尝试 `thread::sleep`，在不同线程给它不同的值，每次运行将变得更加不确定，并每次创建不同的输出。

现在我们已经了解了通道如何工作，接下来让我们看看另一种并发方法。

您可以参考《Rust 编程语言》中的以下章节：_[使用消息传递在线程之间传输数据](https://doc.rust-lang.org/book/ch16-02-message-passing.html#using-message-passing-to-transfer-data-between-threads)_