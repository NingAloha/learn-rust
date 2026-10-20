### 从父模块开始使用 `super` 构造相对路径

我们可以通过在路径开头使用 `super`，构造以父模块而不是当前模块或 crate 根为起点的相对路径。这类似于用 `..` 语法开始文件系统路径。使用 `super` 允许我们引用我们知道在父模块中的项目，这可以使模块关系紧密但父模块可能被移动到模块树其他位置时，更容易重组模块树。

考虑下面的代码，它模拟了一种情景：厨师修正了一个错误的订单，并亲自将其送到顾客那里。在 `back_of_house` 模块中定义的函数 `fix_incorrect_order` 通过指定以 `super` 开始的路径调用了父模块中定义的函数 `deliver_order`：

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

##### 使用以 `super` 开始的相对路径调用函数

函数 `fix_incorrect_order` 位于 `back_of_house` 模块中，因此我们可以使用 `super` 前往 `back_of_house` 的父模块，在此例中是 `crate` 根。从那里，我们寻找 `deliver_order` 并找到了它。成功！我们认为 `back_of_house` 模块和 `deliver_order` 函数可能会保持相同的关系，并在我们决定重组 crate 模块树时一起移动。因此，我们使用了 `super`，这样将来如果这些代码被移动到不同的模块，我们需要更新的代码就会更少。