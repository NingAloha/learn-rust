### 多线程中的多重所有权

在Rust书籍的[第十五章](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)中，作者通过使用智能指针 `Rc<T>` 创建一个具有引用计数的值，赋予一个值多个所有者。我们在这里也做一样的事情，看看会发生什么。我们将在以下示例中将 `Mutex<T>` 包装在 `Rc<T>` 中，并在将所有权移动到线程之前克隆 `Rc<T>`。既然我们已经看到了错误，我们还将切换回使用 `for` 循环，并在闭包中保留 `move` 关键字。

```rust
    use std::rc::Rc;
    use std::sync::Mutex;
    use std::thread;

    fn main() {
        let counter = Rc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Rc::clone(&counter);
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

##### 尝试使用 Rc<T> 允许多线程拥有 Mutex<T>

再次进行编译，我们得到了... 不同的错误！编译器教会我们很多。

```text
error[E0277]: `Rc<Mutex<i32>>` 无法在线程间安全传送
   --> src/main.rs:11:22
    |
11  |           let handle = thread::spawn(move || {
    |  ______________________^^^^^^^^^^^^^_-
    | |                      |
    | |                      `Rc<Mutex<i32>>` 无法在线程间安全传送
12  | |             let mut num = counter.lock().unwrap();
13  | |
14  | |             *num += 1;
15  | |         });
    | |_________- 位于此 `[closure@src/main.rs:11:36: 15:10]`
    |
    = 帮助: 在 `[closure@src/main.rs:11:36: 15:10]` 中，trait `Send` 未针对于 `Rc<Mutex<i32>>` 实现
    = 注意: 必须因为它出现在类型 `[closure@src/main.rs:11:36: 15:10]` 内
```

哇，错误信息非常啰嗦！这里要关注的重要部分是：`` `Rc<Mutex<i32>>` 无法在线程间安全传送 ``。编译器也告诉我们原因：``trait `Send`未针对于 `Rc<Mutex<i32>>` 实现 ``。在下一节中我们将讨论 `Send`，它是用于确保我们在线程中使用的类型适于并发环境的trait之一。

不幸的是，`Rc<T>` 并不能安全地在线程之间共享。当 `Rc<T>` 管理引用计数时，它在每次调用 `clone` 时都会增加计数，并在每个克隆被删除时减去计数。但它没有使用任何并发原语来确保对计数的更改不会被其他线程中断。这可能导致错误的计数——这种微妙的错误可能导致内存泄漏或在我们使用完一个值之前就被丢弃。我们需要的是一个与 `Rc<T>` 相似但能在线程安全的方式下更改引用计数的类型。