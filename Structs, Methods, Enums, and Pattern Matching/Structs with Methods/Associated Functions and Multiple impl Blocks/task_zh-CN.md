### 关联函数

`impl` 块的另一个有用功能是我们可以在 `impl` 块内定义不以 `self` 作为参数的函数。这些函数被称为*关联函数*，因为它们与结构体关联。它们仍是函数，而非方法，因为它们不与结构体的实例协作。你已经使用过 `String::from` 关联函数。

关联函数通常用于返回结构体新实例的构造器。例如，我们可以提供一个关联函数，用一个维度参数作为宽度和高度，这样更容易创建一个正方形 `Rectangle`，而不是每次都需要指定相同的值两次：

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

要调用这个关联函数，我们使用结构体名称和 `::` 语法；`let sq = Rectangle::square(3);` 是一个示例。这个函数是在结构体的命名空间内：`::` 语法用于关联函数和由模块创建的命名空间。在“模块”一章中，我们将讨论模块。

### 多个 `impl` 块

每个结构体都允许有多个 `impl` 块。例如，“在 `Rectangle` 上实现 `can_hold` 方法，以另一个 `Rectangle` 实例作为参数” 的列表相当于下面显示的代码，每个方法在它自己的 `impl` 块中。

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

#### 使用多个 `impl` 块重写“在 `Rectangle` 上实现 `can_hold` 方法，以另一个 `Rectangle` 实例作为参数”的列表

这里没有理由将这些方法分成多个 `impl` 块，但这在语法上是有效的。在“泛型类型、特征和生命周期”一章中，我们将看到多个 `impl` 块有用的情况。