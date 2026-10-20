### 哈希映射与所有权

对于实现了 `Copy` trait 的类型，如 `i32`，这些值会被复制到哈希映射中。对于像 `String` 这样的拥有所有权的值，这些值将被移动，而哈希映射将成为这些值的所有者，如下所示。

```rust
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 此时 field_name 和 field_value 已经无效，尝试使用它们，看看会收到什么编译错误！
```

#### 显示键和值被哈希映射拥有，一旦它们被插入

在 `insert` 调用后，我们无法使用变量 `field_name` 和 `field_value`，因为它们已经被移动到哈希映射中。

如果我们插入对值的引用进入哈希映射中，值就不会被移动到哈希映射中。引用指向的值至少必须在哈希映射有效的期间保持有效。您可以在 Rust 书籍第十章的[“使用生命周期验证引用”][validating-references-with-lifetimes]<!-- ignore -->一节中找到更多关于这些问题的信息。

[validating-references-with-lifetimes]: https://github.com/rust-lang/book/blob/master/src/ch10-03-lifetime-syntax.md