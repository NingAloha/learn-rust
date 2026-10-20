### 字符串索引

在许多其他编程语言中，通过索引引用字符串中的单个字符是一个有效且常见的操作。然而，如果你尝试在Rust中使用索引语法访问`String`的一部分，你将会得到一个错误。请考虑以下无效代码。

```rust
    let s1 = String::from("hello");
    let h = s1[0];
```

##### 尝试对String使用索引语法

这段代码将导致以下错误：

```text
error[E0277]: the type `String` cannot be indexed by `{integer}`
 --> src/main.rs:3:13
  |
3 |     let h = s1[0];
  |             ^^^^^ `String` cannot be indexed by `{integer}`
  |
  = help: the trait `Index<{integer}>` is not implemented for `String`
```

错误和提示揭示了问题：Rust字符串不支持索引。但为什么呢？要回答这个问题，我们需要讨论Rust是如何在内存中存储字符串的。你可以在[书籍](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#indexing-into-strings)中阅读详细内容。