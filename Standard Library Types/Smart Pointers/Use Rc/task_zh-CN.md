## 使用 `Rc`

在这个练习中，我们希望通过 `Rc<T>` 类型表达多个所有者的概念。这是我们太阳系的模型 - 有一个 `Sun` 类型和多个 `Planet`。这些 `Planet` 拥有太阳，表示它们围绕太阳旋转。

通过使用适当的 `Rc` 原语来表达太阳有多个所有者，使这段代码能够编译。

<div class="hint">
这是一个简单的练习，旨在使用 <code>Rc<T></code> 类型。每个 <code>Planet</code> 拥有 <code>Sun</code> 的所有权，并使用 <code>Rc::clone()</code> 来增加 <code>Sun</code> 的引用计数。在使用 <code>drop()</code> 将行星单独移出作用域之后，引用计数会减少。最后，太阳再次只有一个对自己的引用。<a href="https://doc.rust-lang.org/book/ch15-04-rc.html">更多内容见《Rust 编程语言》</a>。
</div>