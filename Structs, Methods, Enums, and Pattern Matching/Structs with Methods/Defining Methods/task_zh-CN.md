### 定义方法

让我们将以 `Rectangle` 实例为参数的 `area` 函数更改为在 `Rectangle` 结构体上定义的 `area` 方法，如下所示。

<span class="filename">文件名: src/main.rs</span>

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "矩形的面积是 {} 平方像素。",
        rect1.area()
    );
}
```

#### 在 `Rectangle` 结构体上定义 `area` 方法

要在 `Rectangle` 的上下文中定义函数，我们开始一个 `impl`（实现）块。然后我们将 `area` 函数移到 `impl` 的大括号内，并将函数签名中的第一个（且在此情况下唯一的）参数更改为 `self`，并在方法体的各处使用。在 `main` 中，我们调用 `area` 函数并传递 `rect1` 作为参数，现在可以改为使用*方法语法*来调用我们的 `Rectangle` 实例上的 `area` 方法。方法语法跟在实例后面：我们添加一个点，后跟方法名、括号和任何参数。

在 `area` 的签名中，我们使用 `&self` 而不是 `rectangle: &Rectangle`，因为由于此方法位于 `impl Rectangle` 中，Rust 知道 `self` 的类型是 `Rectangle`。请注意，我们仍然需要在 `self` 前面使用 `&`，就像在 `&Rectangle` 中一样。方法可以获取 `self` 的所有权、像我们这里做的那样不可变地借用 `self`，或者可变地借用 `self`，就像它们可以对任何其他参数那样。

我们选择 `&self` 是出于与在函数版本中使用 `&Rectangle` 相同的原因：我们不想获取所有权，仅想读取结构体中的数据，而不对其进行写操作。如果我们想在方法的操作中修改调用方法的实例，我们会使用 `&mut self` 作为第一个参数。仅仅使用 `self` 作为第一个参数以获取实例所有权的方法是非常少见的；这种技术通常在方法将 `self` 转换为其他东西时使用，并且您想防止调用者在转换后使用原始实例。

使用方法而不是函数的主要优点，除了使用方法语法以及不必在每个方法的签名中重复 `self` 的类型外，还在于组织。在一个 `impl` 块中，我们汇总了可以用类型实例做的所有事情，而不是让我们代码的未来用户在我们提供的库中四处寻找 `Rectangle` 的功能。

> ### `->` 操作符在哪里？
>
> 在 C 和 C++ 中，调用方法使用两个不同的操作符：直接在对象上调用方法用 `.`，在指针对象上调用方法并首先需要解引用指针用 `->`。换句话说，如果 `object` 是一个指针，`object->something()` 类似于 `(*object).something()`。
>
> Rust 没有 `->` 操作符的等效操作符；相反，Rust 有一个称为*自动引用和解引用*的功能。调用方法是在 Rust 中拥有此行为的少数情况之一。
>
> 它的工作原理是这样的：当你使用 `object.something()` 调用一个方法时，Rust 会自动添加 `&`、`&mut` 或 `*` 以便 `object` 匹配方法的签名。换句话说，以下两者是相同的：
>
<!-- CAN'T EXTRACT SEE BUG TODO -->
> ```rust
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> 第一个看起来更干净。这种自动引用行为之所以有效，是因为方法有一个明确的接收者——`self` 的类型。给定接收者和方法的名称，Rust 可以明确判断方法是读取（`&self`）、修改（`&mut self`），还是消耗（`self`）。Rust 将借用对方法接收者隐式化是让所有权在实践中易于使用的重要部分。