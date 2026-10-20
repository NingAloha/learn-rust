## 泛型类型参数、特征约束和生命周期一起使用

让我们简要地看一下在一个函数中同时指定泛型类型参数、特征约束和生命周期的语法！

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("公告! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

这是之前的 `longest` 函数，它返回两个字符串切片中较长的一个。但现在它多了一个名为 `ann` 的额外参数，这个参数是泛型类型 `T`，可以由任何实现了 `Display` 特征的类型填充，如由 `where` 子句指定。这个额外的参数将在函数比较字符串切片的长度之前被打印出来，这就是为什么需要 `Display` 特征约束。因为生命周期是一种泛型，生命周期参数 `'a` 和泛型类型参数 `T` 的声明在函数名后的尖括号中的同一个列表中。