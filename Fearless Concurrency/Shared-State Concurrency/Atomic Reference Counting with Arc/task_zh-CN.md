#### 使用 `Arc<T>` 进行原子引用计数

幸运的是，`Arc<T>` 是一种类似于 `Rc<T>` 的类型，可以安全地用于并发场景中。`a` 代表的是 *atomic*，意味着它是一种 *原子引用计数* 类型。原子类型是另一种并发原语，这里我们不会详细介绍：更多细节请参见标准库文档中的 [`std::sync::atomic`]。此时，您只需了解原子类型的工作方式类似于原始类型，但可以安全地在线程间共享。

[`std::sync::atomic`]: https://doc.rust-lang.org/std/sync/atomic/

您可能会想，为什么所有的原始类型不是原子的，以及为什么标准库类型默认没有实现为使用 `Arc<T>`。原因是线程安全会带来性能损失，只有在确实需要时才值得付出。如果您仅在单个线程中对值进行操作，那么如果不需要强制实现原子提供的保证，您的代码运行会更快。

让我们回到示例中：`Arc<T>` 和 `Rc<T>` 具有相同的 API，因此我们通过更改 `use` 行、调用 `new` 和调用 `clone` 来修复程序。下面的代码将最终编译并运行：

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

##### 使用 Arc<T> 将 Mutex<T> 包裹以便能够在多个线程间共享所有权

这段代码将打印以下内容：

```text
    Result: 10
```

我们成功了！我们从 0 数到了 10，这看起来可能不那么令人印象深刻，但这确实教会了我们很多关于 `Mutex<T>` 和线程安全的知识。您还可以使用这个程序的结构来执行比仅仅增加一个计数器更复杂的操作。通过这种策略，您可以将计算拆分成独立部分，将这些部分分配给不同的线程，然后使用 `Mutex<T>` 让每个线程用其部分更新最终结果。