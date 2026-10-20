### 结构体定义中的生命周期注解

到目前为止，我们只定义了持有所有权类型的结构体。结构体可以持有引用，但在这种情况下，我们需要在结构体定义中的每个引用上添加生命周期注解。下面的示例有一个名为`ImportantExcerpt`的结构体，它持有一个字符串切片。

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

#### 一个持有引用的结构体，所以其定义需要生命周期注解

这个结构体有一个字段，`part`，它持有一个字符串切片，这是一个引用。与泛型数据类型一样，我们在结构体名称后面用尖括号声明泛型生命周期参数的名称，以便在结构体定义体中使用生命周期参数。这个注解意味着`ImportantExcerpt`的实例不能比它在`part`字段中持有的引用活得更长。

这里的`main`函数创建了一个`ImportantExcerpt`结构体的实例，持有对由变量`novel`所拥有的`String`的第一句话的引用。在创建`ImportantExcerpt`实例之前，`novel`中的数据已经存在。此外，`novel`在`ImportantExcerpt`超出作用域之前不会超出作用域，因此`ImportantExcerpt`实例中的引用是有效的。