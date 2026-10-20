## 布尔类型

与大多数其他编程语言一样，Rust 中的布尔类型有两个可能的值：`true` 和 `false`。布尔类型占用一个字节。Rust 中的布尔类型使用 `bool` 进行指定。例如：

```rust
fn main() {
    let t = true;

    let f: bool = false; // 使用显式类型注解
}
```

使用布尔值的主要方式是通过条件语句，例如 `if` 表达式。我们将在本节的 ["条件"](course://Common Programming Concepts/Conditions/Intro) 课程中介绍 Rust 中 `if` 表达式的工作原理。