### 使用特征约束有条件地实现方法

通过在 `impl` 块中使用泛型类型参数的特征约束，我们可以为实现指定特征的类型有条件地实现方法。例如，下面代码中的 `Pair<T>` 类型总是实现 `new` 函数。但 `Pair<T>` 只有在其内部类型 `T` 实现了使比较成为可能的 `PartialOrd` 特征和使打印成为可能的 `Display` 特征时，才实现 `cmp_display` 方法。

```rust,noplayground
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

#### 根据特征约束有条件地在泛型类型上实现方法

我们还可以根据另一个特征有条件地为任意实现该特征的类型实现一个特征。对于满足特征约束的任意类型上的特征实现称为*通用实现*，并广泛用于 Rust 标准库中。例如，标准库在实现了 `Display` 特征的任何类型上实现了 `ToString` 特征。标准库中的 `impl` 块看起来类似于以下代码：

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

由于标准库有此通用实现，我们可以在任何实现了 `Display` 特征的类型上调用由 `ToString` 特征定义的 `to_string` 方法。例如，我们可以将整数转化为其对应的 `String` 值，如下所示，因为整数实现了 `Display`：

```rust
let s = 3.to_string();
```

通用实现显示在特征文档的“Implementors”部分。

特征和特征约束允许我们编写使用泛型类型参数的代码，从而减少重复，但同时指示编译器我们希望泛型类型具备特定行为。编译器可以利用特征约束信息检查与我们代码一起使用的所有具体类型是否提供正确的行为。在动态类型语言中，如果我们对未定义该方法的类型调用方法，将在运行时出错。但 Rust 将这些错误转移到编译时期，因此我们被迫在代码能够运行之前解决问题。此外，我们不必编写在运行时检查行为的代码，因为我们已经在编译时进行了检查。这种做法在提高性能的同时不必放弃泛型的灵活性。