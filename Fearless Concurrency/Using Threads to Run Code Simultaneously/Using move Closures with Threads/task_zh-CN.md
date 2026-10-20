### 在线程中使用 move 闭包

`move` 闭包通常与 `thread::spawn` 一起使用，因为它允许在一个线程中使用来自另一个线程的数据。

在第十三章中，我们提到过可以在闭包的参数列表前使用 `move` 关键字，以强制闭包对其使用的环境中的值进行所有权获取。这种技术在创建新线程时尤其有用，因为它可以将值的所有权从一个线程转移到另一个线程。

注意关于创建线程的代码片段中，我们传递给 `thread::spawn` 的闭包不接受任何参数：我们在被创建的线程的代码中没有使用主线程中的数据。要在被创建的线程中使用主线程的数据，创建的线程的闭包必须捕获它需要的值。下面的代码片段展示了试图在主线程中创建一个向量并在被创建的线程中使用它。然而，这种方式还无法工作，稍后你会看到原因。

```rust
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(|| {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }
```

##### 尝试在另一个线程中使用主线程创建的向量

闭包使用了 `v`，因此它会捕获 `v` 并将其作为闭包环境的一部分。因为 `thread::spawn` 在一个新线程中运行这个闭包，我们应该能够在这个新线程中访问 `v`。但是，当我们编译这个示例时，会得到如下错误：

```text
    error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
     --> src/main.rs:6:32
      |
    6 |     let handle = thread::spawn(|| {
      |                                ^^ may outlive borrowed value `v`
    7 |         println!("Here's a vector: {:?}", v);
      |                                           - `v` is borrowed here
      |
    note: function requires argument type to outlive `'static`
     --> src/main.rs:6:18
      |
    6 |       let handle = thread::spawn(|| {
      |  __________________^
    7 | |         println!("Here's a vector: {:?}", v);
    8 | |     });
      | |______^
    help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
      |
    6 |     let handle = thread::spawn(move || {
      |                                ^^^^^^^
```

Rust _推断_ 如何捕获 `v`，由于 `println!` 只需要 `v` 的引用，闭包尝试借用 `v`。然而，这里有个问题：Rust 无法确定被创建的线程会运行多长时间，因此它不知道对 `v` 的引用是否总是有效的。

下一个示例提供了一种更可能导致 `v` 的引用无效的场景：

```rust
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(|| {
            println!("Here's a vector: {:?}", v);
        });

        drop(v); // 糟糕！

        handle.join().unwrap();
    }
```

##### 一个线程试图从一个释放了 v 的主线程中捕获对 `v` 引用的闭包

如果允许运行此代码，有一种可能性是，被创建的线程可能会立即被放在后台而没有运行。在被创建的线程中有一个对 `v` 的引用，但是主线程立即使用我们在第十五章中讨论的 `drop` 函数释放了 `v`。于是，当被创建的线程开始执行时，`v` 已经不再有效，所以对它的引用也是无效的。糟糕！

为了修复关于在线程之间传递向量的示例中的编译错误，我们可以使用错误信息中的建议：

```text
    help: to force the closure to take ownership of `v` (and any other referenced
    variables), use the `move` keyword
      |
    6 |     let handle = thread::spawn(move || {
      |                                ^^^^^^^
```

通过在闭包前加上 `move` 关键字，我们强制闭包对正在使用的值进行所有权获取，而不是让 Rust 推断应该借用这些值。下面代码片段中的修改后的代码将按照我们预期的方式编译并运行：

```rust
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }
```

##### 使用 move 关键字强制闭包获取其使用值的所有权

如果在主线程调用 `drop` 的闭包的线程代码片段中使用 `move` 闭包，会发生什么？`move` 能修复这种情况吗？不幸的是不能，我们会因为试图做一个出于不同原因不被允许的操作而得到不同的错误。如果我们在闭包中添加了 `move`，那么我们会将 `v` 移动到闭包的环境中，因此无法在主线程中再调用 `drop`。相反，我们会得到这个编译器错误：

```text
    error[E0382]: use of moved value: `v`
      --> src/main.rs:10:10
       |
    4  |     let v = vec![1, 2, 3];
       |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
    5  | 
    6  |     let handle = thread::spawn(move || {
       |                                ------- value moved into closure here
    7  |         println!("Here's a vector: {:?}", v);
       |                                           - variable moved due to use in closure
    ...
    10 |     drop(v); // 糟糕！
       |          ^ value used here after move
```

Rust 的所有权规则再次保护了我们！在关于在线程之间传递向量的代码中，我们因为借用了 `v` 而得到了错误，因为 Rust 很保守，只是为线程借用了 `v`，这意味着主线程理论上可以使被创建线程的引用失效。通过告知 Rust 将 `v` 的所有权移动到被创建的线程中，我们向 Rust 保证了主线程将不再使用 `v`。如果我们按照同样的方式修改使用闭包的示例，当我们试图在主线程中使用 `v` 时，就违反了所有权规则。`move` 关键字会覆盖 Rust 的保守借用默认值；它不会让我们违反所有权规则。

通过对线程和线程 API 的基本理解，让我们看看可以用线程做些什么。

你可以参考 Rust 编程语言书中的以下章节：[使用线程同时运行代码](https://doc.rust-lang.org/book/ch16-01-threads.html#using-threads-to-run-code-simultaneously)