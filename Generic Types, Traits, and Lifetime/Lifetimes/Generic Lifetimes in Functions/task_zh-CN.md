### 函数中泛用的生命周期

让我们编写一个函数，该函数返回两个字符串切片中较长的一个。这个函数将接收两个字符串切片并返回一个字符串切片。在我们实现 `longest` 函数之后，下面的代码应该会打印`最长的字符串是 abcd`。

```rust,ignore
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是 {}", result);
}
```

#### 一个 `main` 函数，用于调用 `longest` 函数以找出两个字符串切片中较长的一个

注意，我们希望函数接收字符串切片，这些是引用，因为我们不希望 `longest` 函数取得其参数的所有权。参阅“理解所有权”中的“字符串切片作为参数”部分，进一步讨论为何在上面的代码片段中使用这些参数是我们想要的。

如果我们尝试像下面那样实现 `longest` 函数，将无法编译。

```rust,ignore,does_not_compile
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

#### 一个 `longest` 函数的实现，它返回两个字符串切片中较长的一个，但尚未能编译

相反，我们得到了谈论生命周期的以下错误：

```console
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ^^^^    ^^^^^^^     ^^^^^^^     ^^^
```

帮助文本揭示了返回类型需要一个泛用生命周期参数，因为 Rust 无法确定返回的引用是指向 `x` 还是 `y`。实际上，我们也不清楚，因为该函数的主体中的 `if` 块返回的是 `x` 的引用，`else` 块返回的是 `y` 的引用！

在定义这个函数时，我们不知道将传入该函数的具体值，因此我们不知道会执行 `if` 还是 `else` 案例。我们也不知道传入引用的具体生命周期，因此不能像在本节第二和第三个代码片段中那样查看作用域来确定我们返回的引用是否总是有效。借用检查器也无法确定这一点，因为它不知道 `x` 和 `y` 的生命周期与返回值的生命周期之间的关系。为了解决这个错误，我们将添加泛用生命周期参数，以定义引用之间的关系，从而让借用检查器进行分析。