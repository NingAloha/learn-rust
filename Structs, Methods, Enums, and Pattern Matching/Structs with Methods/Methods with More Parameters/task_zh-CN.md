### 多参数方法

让我们通过为 `Rectangle` 结构体实现第二个方法来练习使用方法。这次，我们希望 `Rectangle` 的一个实例接收另一个 `Rectangle` 实例，并返回 `true`，如果第二个 `Rectangle` 能完全放入 `self` 中；否则应返回 `false`。也就是说，我们希望能够编写如下所示的程序，一旦我们定义了 `can_hold` 方法。

```rust,ignore
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1 能包含 rect2 吗？{}", rect1.can_hold(&rect2));
    println!("rect1 能包含 rect3 吗？{}", rect1.can_hold(&rect3));
}
```

#### 使用尚未编写的 `can_hold` 方法

预期的输出将如下所示，因为 `rect2` 的两个维度都比 `rect1` 的小，而 `rect3` 比 `rect1` 更宽：

```text
rect1 能包含 rect2 吗？true
rect1 能包含 rect3 吗？false
```

我们知道我们想定义一个方法，所以它将位于 `impl Rectangle` 块中。方法名称将是 `can_hold`，它将接受另一个 `Rectangle` 的不可变借用作为参数。我们可以通过查看调用该方法的代码来确定参数的类型：`rect1.can_hold(&rect2)` 传入了 `&rect2`，即 `rect2` 的不可变借用，一个 `Rectangle` 的实例。这是合理的，因为我们只需要读取 `rect2`（而不是写入，写入则需要可变借用），并且我们希望 `main` 继续拥有 `rect2` 的所有权以便在调用 `can_hold` 方法后还能再次使用它。`can_hold` 的返回值将是一个布尔值，其实现将检查 `self` 的宽度和高度是否都大于另一个 `Rectangle` 的宽度和高度。让我们将新的 `can_hold` 方法添加到本节第一个列表中的 `impl` 块中，如下所示。

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

#### 在 `Rectangle` 上实现 `can_hold` 方法，该方法以另一个 `Rectangle` 实例作为参数

当我们用前面的代码片段中的 `main` 函数运行此代码时，我们将获得我们期望的输出。方法可以接受多个参数，这些参数可以在 `self` 参数之后添加到签名中，这些参数的工作方式与函数中的参数相同。