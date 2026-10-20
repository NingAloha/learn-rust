## 返回值和作用域

返回值也可以转移所有权。下面的代码片段是一个示例，带有与前一个片段类似的注释。

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership 将其返回值移动到 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到 takes_and_gives_back，
    // 同时它的返回值也被移动到 s3
} // 在这里，s3 超出作用域并被释放。s2 超出作用域但已被移动，所以什么也不发生。s1 超出作用域并被释放。

fn gives_ownership() -> String {             // gives_ownership 将其返回值移动到调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域

    some_string                              // some_string 被返回并移出到调用函数
}

// takes_and_gives_back 将接收一个字符串并返回一个字符串
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // a_string 被返回并移出到调用函数
}
```

##### 转移返回值的所有权

变量的所有权每次都遵循相同的模式：将值赋给另一个变量会移动它。当包含堆上数据的变量超出作用域时，值将由 `drop` 清理，除非数据被移动并由其他变量拥有。

_您可以参考 Rust 编程语言书中的以下章节：[所有权](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html)_