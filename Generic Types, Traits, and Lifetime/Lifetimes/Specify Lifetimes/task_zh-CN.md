## 指定生命周期

Rust编译器需要知道如何检查所提供的引用是否有效，以便在引用在使用前有可能超出作用域时通知程序员。请记住，引用是借用，不拥有自己的数据。如果它们的所有者超出作用域会怎样？

<div class="hint">
让编译器指导你。

如果需要帮助，也可以查看[这本书](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)。
</div>