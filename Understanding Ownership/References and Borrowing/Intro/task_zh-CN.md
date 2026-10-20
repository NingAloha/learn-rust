## 介绍

在每个函数中获取所有权然后再返回所有权有点繁琐。如果我们想让一个函数使用一个值，但是不想获取它的所有权该怎么办？很烦的一点是，如果我们想再次使用传入的任何东西，同时还想返回函数主体中可能得到的任何数据，我们还需要传回来。

可以使用元组返回多个值，如下面的代码片段所示。

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回一个字符串的长度

    (s, length)
}
```

##### 返回参数的所有权

但这对于一个应该很常见的概念来说是过于繁复和麻烦了。幸运的是，Rust 为这个概念提供了一个特性，叫做_引用_。