## 所有权与函数

将值传递给函数的语义与将值赋给变量的语义类似。将变量传递给函数将发生移动或复制，就像赋值时一样。下面的例子包含一些注释，展示变量在哪里进入和离开作用域。

```rust
fn main() {
    let s = String::from("hello");  // s进入作用域

    takes_ownership(s);             // s的值移动到函数中...
    // ...因此这里不再有效

    let x = 5;                      // x进入作用域

    makes_copy(x);                  // x将移动到函数中，
    // 但由于i32是Copy，所以还可以在之后使用x

} // 在这里，x离开作用域，然后是s。但因为s的值已被移动，所以没有特别的事情发生。

fn takes_ownership(some_string: String) { // some_string进入作用域
    println!("{}", some_string);
} // 这里，some_string离开作用域并调用`drop`。底层内存被释放。

fn makes_copy(some_integer: i32) { // some_integer进入作用域
    println!("{}", some_integer);
} // 这里，some_integer离开作用域。没有特别的事情发生。
```

##### 带有所有权和作用域注释的函数

如果我们尝试在调用`takes_ownership`后使用`s`，Rust会抛出编译时错误。这些静态检查保护我们不犯错误。尝试在`main`里添加代码以使用`s`和`x`，看看你在哪里可以使用它们，以及所有权规则如何阻止你这么做。