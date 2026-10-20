## 使用 Box

在编译时，Rust 需要知道一个类型占据多少空间。这对于递归类型来说会变得有问题，因为一个值可以作为其自身的一部分包含另一个相同类型的值。为了绕过这个问题，我们可以使用 `Box` —— 一个用于在堆上存储数据的智能指针，同时也允许我们包装递归类型。

我们在这个练习中实现的递归类型是 `cons 列表`，这是一种在函数式编程语言中常见的数据结构。cons 列表中的每个项目包含两个元素：当前项目的值和下一个项目。最后一个项目是一个名为 `Nil` 的值。

**步骤 1**：在枚举定义中使用 `Box` 使代码编译

**步骤 2**：通过替换 `unimplemented!()` 创建空和非空的 cons 列表

如果遇到困难，可以查看提示！ :)

<div class="hint">步骤 1：
编译器的消息应该有所帮助：由于在处理递归类型时无法存储值的实际类型，我们需要存储指向其值的引用（指针）。因此，我们应该将我们的 <code>List</code> 放在 <code>Box</code> 中。更多细节可在书中找到：
<a href="https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes">https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes</a>。
</div>

<div class="hint">步骤 2：
创建一个空列表应该相对简单。对于非空列表，请记住我们要使用 Cons “列表构建器”。尽管当前列表是一个整数列表（i32），但请随意改变定义并尝试其他类型！</div>