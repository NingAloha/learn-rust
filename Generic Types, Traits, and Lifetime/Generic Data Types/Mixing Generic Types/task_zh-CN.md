### 混合泛型类型

在结构体定义中的泛型类型参数并不总是与那些方法签名中使用的相同。例如，下面的示例定义了`mixup`方法，该方法用于本节早些时候的`Point<T, U>`结构体。该方法将另一个`Point`作为参数传入，这个`Point`可能与我们在其上调用`mixup`的`self` `Point`类型不同。该方法创建一个新的`Point`实例，其中的`x`值来自于`self` `Point`（类型为`T`），`y`值则来自于传入的`Point`（类型为`W`）。

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

#### 使用不同于结构体定义的泛型类型的方法

在`main`中，我们定义了一个`Point`，该`Point`的`x`为`i32`类型（值为`5`），`y`为`f64`类型（值为`10.4`）。变量`p2`是一个`Point`结构体，其`x`是一个字符串切片（值为`"Hello"`），`y`是一个`char`（值为`c`）。在`p1`上调用`mixup`并传入参数`p2`会得到`p3`，其`x`将是`i32`类型，因为`x`来自`p1`。变量`p3`的`y`将是`char`类型，因为`y`来自`p2`。`println!`宏调用将会输出`p3.x = 5, p3.y = c`。

此示例的目的是展示一种情况，其中一些泛型参数在`impl`中声明，而另一些则在方法定义中声明。在这里，泛型参数`T`和`U`在`impl`之后声明，因为它们与结构体定义相关联。泛型参数`V`和`W`则在`fn mixup`之后声明，因为它们仅与方法相关。