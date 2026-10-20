## 类型转换

Rust 提供了多种方法将给定类型的值转换为另一种类型。

最简单的类型转换形式是类型转换表达式。它用二元运算符 `as` 表示。例如，`println!("{}", 1 + 1.0);` 将不会编译，因为 `1` 是整数而 `1.0` 是浮点数。然而，`println!("{}", 1 as f32 + 1.0)` 应该可以编译。练习 [`using_as`](using_as.rs) 尝试涵盖这一点。

Rust 还提供了一些特性，通过实现这些特性可以促进类型转换。这些特性在 [`convert`](https://doc.rust-lang.org/std/convert/index.html) 模块中可以找到。
这些特性包括：
- `From` 和 `Into` 在 [`from_into`](from_into.rs) 中涵盖
- `TryFrom` 和 `TryInto` 在 [`try_from_into`](try_from_into.rs) 中涵盖
- `AsRef` 和 `AsMut` 在 [`as_ref_mut`](as_ref_mut.rs) 中涵盖

此外，`std::str` 模块提供了一个特性叫做 [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)，通过字符串上的 `parse` 方法将字符串转换为目标类型。如果为给定类型 `Person` 正确实现此特性，那么 `let p: Person = "Mark,20".parse().unwrap()` 应该既能够编译也能运行而不恐慌。

这些应该是在***标准库内*** 将数据转换为所需类型的主要方法。

#### Rust Book章节

这些在书中没有直接涵盖，但标准库有很好的关于[转换的文档](https://doc.rust-lang.org/std/convert/index.html) 。`FromStr` 特性也在[这里](https://doc.rust-lang.org/std/str/trait.FromStr.html) 涵盖。