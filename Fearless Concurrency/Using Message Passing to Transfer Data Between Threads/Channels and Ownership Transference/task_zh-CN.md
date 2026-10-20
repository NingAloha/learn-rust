### 通道和所有权转移

所有权规则在消息传递中起着至关重要的作用，因为它们帮助您编写安全的并发代码。在您的 Rust 程序中始终考虑所有权是防止并发编程错误的优势。让我们做一个实验，展示通道和所有权如何协同工作以防止问题：我们尝试在将一个 `val` 值发送到通道之后，在生成的线程中使用它。尝试编译下一个示例中的代码，看看为什么此代码不被允许：

```rust
    use std::thread;
    use std::sync::mpsc;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            println!("val is {}", val);
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
```

##### 尝试在将 val 发送到通道后使用它

在这里，我们尝试在通过 `tx.send` 将 `val` 发送到通道后打印它。允许这样做是一个坏主意：一旦该值已被发送到另一个线程，该线程可能会在我们再次尝试使用该值之前修改或删除它。潜在地，另一个线程的修改可能会导致错误或由于数据不一致或不存在而导致意外结果。然而，如果我们尝试编译下面示例中的代码，Rust 会给我们一个错误：

```text
    error[E0382]: borrow of moved value: `val`
      --> src/main.rs:10:31
       |
    8  |         let val = String::from("hi");
       |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
    9  |         tx.send(val).unwrap();
       |                 --- value moved here
    10 |         println!("val is {}", val);
       |                               ^^^ value borrowed here after move

```

我们的并发错误导致了一个编译时错误。`send` 函数接管其参数的所有权，并且当值被移动时，接收方接管它的所有权。这阻止了我们在发送之后意外地再次使用该值；所有权系统会检查一切是否正常。