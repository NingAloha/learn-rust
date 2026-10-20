### 使用结构体重构：增加更多意义

我们使用结构体通过为数据标注来增加意义。我们可以将使用的元组转换成一个包含整体名称和部分名称的数据类型，如下所示。

<span class="filename">文件名：src/main.rs</span>

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "矩形的面积是 {} 平方像素。",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

#### 定义 `Rectangle` 结构体

这里我们定义了一个结构体，并将其命名为 `Rectangle`。在大括号内，我们将字段定义为 `width` 和 `height`，它们的类型都是 `u32`。然后在 `main` 中，我们创建了一个具有宽度 30 和高度 50 的 `Rectangle` 实例。

我们的 `area` 函数现在定义了一个参数，我们将其命名为 `rectangle`，其类型是对结构体 `Rectangle` 实例的不可变借用。正如在“理解所有权”章节中提到的，我们希望借用结构体而不是取得它的所有权。这样一来，`main` 保留其所有权并可以继续使用 `rect1`，这就是我们在函数签名和调用函数时使用 `&` 的原因。

`area` 函数访问 `Rectangle` 实例的 `width` 和 `height` 字段。我们现在对 `area` 的函数签名准确表达了我们的意思：计算 `Rectangle` 的面积，使用其 `width` 和 `height` 字段。这表明宽度和高度彼此关联，并为值提供了描述性名称，而不是使用元组索引值 `0` 和 `1`。这在清晰性上是一个进步。