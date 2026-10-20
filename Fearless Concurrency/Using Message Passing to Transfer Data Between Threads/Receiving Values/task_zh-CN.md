## 接收值

在下一个例子中，我们将在主线程中从通道的接收端获取值。这就像从河的尽头捞起橡皮鸭或像接收到聊天消息。

```rust
    use std::thread;
    use std::sync::mpsc;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
```

##### 在主线程中接收值 “hi” 并打印它

通道的接收端有两个有用的方法：`recv` 和 `try_recv`。我们使用的是缩写自 _receive_ 的 `recv`，它会阻塞主线程的执行，等待直到有值通过通道发送过来。一旦有值发送过来，`recv` 将返回一个 `Result<T, E>`。当通道的发送端关闭时，`recv` 将返回一个错误，表示不会再有值发送过来了。

`try_recv` 方法则不会阻塞，而是会立即返回一个 `Result<T, E>`：如果有消息可用则返回包含消息的 `Ok` 值，如果这次没有消息则返回 `Err`。如果这个线程在等待消息的同时还有其他工作要做，那么使用 `try_recv` 很有用：我们可以写一个循环，每隔一段时间调用 `try_recv`，如果有消息则处理它，否则则暂时执行其他工作，直到再次检查是否有消息。

在这个例子中，我们为了简单起见使用了 `recv`；因为除了等待消息外，我们没有其他工作要让主线程执行，所以阻塞主线程是合适的。

运行下面代码片段时，我们将在主线程中看到打印出的值：

```text
    Got: hi
```

完美！