## 引擎中的工人

让我编译！

不过最好先阅读解释部分。

`Engine` 包含一个 `Worker` 的向量。最初，这个向量是空的，但我们可以轻松地向其中添加任意数量的工人。一旦引擎有足够的工人，我们就可以运行它。引擎运行其工人。工人们完成他们的工作并写入 `log`。这个 `log` 是什么，从哪里来呢？实际上，它只是引擎中的一个 `String` 向量。你看到问题了吗？

工人需要可写入 `log` 的权限，但 Rust 永远不会允许我们对 `log` 有多个可变引用。不幸的是，仅在各处添加 `mut` 是无效的。我们需要一个名为[内部可变性](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#interior-mutability-a-mutable-borrow-to-an-immutable-value)的东西以及[`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html)为工人提供对引擎日志的临时可写入权限。

还有一个问题。`Worker` 结构体保持对日志的引用。我们知道结构体中的引用需要指定生命周期，但是不要试图遵循编译器的建议。它们不会对你有所帮助。真正的问题是工人和引擎具有不同的生命周期。通过指定生命周期来让借用检查器相信一切正常是不可行的。相反，我们应该使用[Rc&lt;T> 智能指针](https://doc.rust-lang.org/book/ch15-04-rc.html)。

编辑 `worker` 和 `engine` 模块，使程序可以编译、运行并通过测试。不要更改 `main.rs`。它应该按原样工作。

<div class="hint">
而不是遵循编译器的建议，你的第一步可以是在 `worker` 和 `engine` 模块中将日志的类型更改为 <code>Rc&lt;RefCell&lt;Vec&lt;String>>></code>（指向 <code>RefCell</code> 的智能指针，带有字符串的向量）。然后你需要修正其他代码。
</div>

<div class="hint">
你可能需要以下方法：

- `Rc::new()`，`Rc::clone()`;
- `RefCell::new()`，`RefCell::borrow()` 和 `RefCell::borrow_mut()`。
</div>

<div class="hint">
小心不要在打印后销毁日志。确保在 <code>Engine::print_log()</code> 中打印日志条目时通过引用引用它们。
</div>