### 使用 JoinHandle 等待所有线程完成

上面的代码由于主线程的结束，经常会提前停止生成的线程，而且也不能保证生成的线程能够运行。原因是无法保证线程运行的顺序！

我们可以通过将 `thread::spawn` 的返回值保存在变量中来解决生成的线程不能运行或不能完整运行的问题。`thread::spawn` 的返回类型是 `JoinHandle`。`JoinHandle` 是一个拥有所有权的值，当我们在其上调用 `join` 方法时，它会等待其线程结束。下面的例子展示了如何使用我们之前创建的线程的 `JoinHandle` 并调用 `join` 来确保生成的线程在 `main` 退出前完成：

```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }
```

##### 从 thread::spawn 保存一个 JoinHandle 以保证线程运行至完成

在句柄上调用 `join` 会阻塞当前运行的线程，直到该句柄代表的线程终止为止。_阻塞_ 一个线程意味着该线程被阻止执行工作或退出。因为我们在主线程的 `for` 循环之后调用了 `join`，运行此代码应产生如下类似的输出：

```text
    hi number 1 from the main thread!
    hi number 2 from the main thread!
    hi number 1 from the spawned thread!
    hi number 3 from the main thread!
    hi number 2 from the spawned thread!
    hi number 4 from the main thread!
    hi number 3 from the spawned thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!
    hi number 6 from the spawned thread!
    hi number 7 from the spawned thread!
    hi number 8 from the spawned thread!
    hi number 9 from the spawned thread!
```

两个线程继续交替运行，但由于调用了 `handle.join()`，主线程等待，并在生成的线程完成之前不会结束。

但我们来看一下当我们将 `handle.join()` 移到 `main` 中的 `for` 循环前会发生什么，如下所示：

```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
```

主线程将等待生成的线程完成，然后运行其 `for` 循环，因此输出将不再交错，如下所示：

```text
    hi number 1 from the spawned thread!
    hi number 2 from the spawned thread!
    hi number 3 from the spawned thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!
    hi number 6 from the spawned thread!
    hi number 7 from the spawned thread!
    hi number 8 from the spawned thread!
    hi number 9 from the spawned thread!
    hi number 1 from the main thread!
    hi number 2 from the main thread!
    hi number 3 from the main thread!
    hi number 4 from the main thread!
```

像 `join` 调用位置这样的小细节会影响线程是否同时运行。