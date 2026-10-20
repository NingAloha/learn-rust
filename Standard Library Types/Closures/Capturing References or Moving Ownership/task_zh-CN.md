### 捕获引用或移动所有权

闭包可以通过三种方式从其环境中捕获值，这直接对应于函数获取参数的三种方式：不可变借用、可变借用和获取所有权。闭包将根据函数体对捕获值的操作决定使用哪种方式。

以下清单定义了一个闭包，它捕获了一个对名为 `list` 的向量的不可变借用，因为它只需要一个不可变借用来打印值。此示例还说明了一个变量可以绑定到一个闭包定义，之后可以通过使用变量名和括号调用闭包，就像变量名是函数名一样：

```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("定义闭包之前: {:?}", list);

    let only_borrows = || println!("从闭包: {:?}", list);

    println!("调用闭包之前: {:?}", list);
    only_borrows();
    println!("调用闭包之后: {:?}", list);
}
```

##### 定义和调用捕获不可变借用的闭包的示例

`list` 在闭包定义之前的代码中可访问，定义闭包之后但调用闭包之前也可访问，调用闭包之后也可访问，因为我们可以同时有多个对 `list` 的不可变借用。这个代码编译、运行，并打印：

```console
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/closure-example`
定义闭包之前: [1, 2, 3]
调用闭包之前: [1, 2, 3]
从闭包: [1, 2, 3]
调用闭包之后: [1, 2, 3]
```

下一个清单将闭包定义改变为需要一个可变借用，因为闭包体向 `list` 向量中添加一个元素：

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("定义闭包之前: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("调用闭包之后: {:?}", list);
}
```

##### 定义和调用捕获可变借用的闭包的示例

这个代码编译、运行，并打印：

```console
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/closure-example`
定义闭包之前: [1, 2, 3]
调用闭包之后: [1, 2, 3, 7]
```

注意在 `borrows_mutably` 闭包的定义和调用之间不再有 `println!`：当 `borrows_mutably` 被定义时，它捕获了对 `list` 的可变引用。在闭包被调用之后，因为我们不再在那个点之后使用闭包，可变借用结束。在闭包定义和调用之间，出于打印目的的不可变借用是不被允许的，因为当存在一个可变借用时，不允许其他借用。尝试在那里添加一个 `println!` 看看会得到什么错误信息！

如果您想强制闭包获取它在环境中使用的值的所有权，即便闭包的主体并不严格需要所有权，可以在参数列表之前使用 `move` 关键字。这种技术在将闭包传递给新线程时特别有用，以便将数据搬移到新线程以拥有数据的所有权。在我们讨论并发性时，在第16章中将有更多关于 `move` 闭包的示例。