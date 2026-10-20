### 函数参数

函数还可以定义_参数_，参数是函数签名的一部分，属于特殊变量。当一个函数有参数时，你可以为这些参数提供具体的值。从技术上讲，这些具体的值称为_实参_，但在日常对话中，人们往往把_参数_和_实参_两个词混用，既指函数定义中的变量，也指在调用函数时传入的具体值。

以下是 `another_function` 的重写版本，展示了 Rust 中参数的样子：

```rust
   fn main() {
       another_function(5);
   }
   
   fn another_function(x: i32) {
       println!("x 的值是: {}", x);
   }
```

尝试运行此程序，你应该得到以下输出：

```text
   $ cargo run
      Compiling functions v0.1.0 (file:///projects/functions)
       Finished dev [unoptimized + debuginfo] target(s) in 1.21 secs
        Running `target/debug/functions`
   x 的值是: 5
```

`another_function` 的声明有一个名为 `x` 的参数。`x` 的类型被指定为 `i32`。当 `5` 被传递给 `another_function` 时，`println!` 宏将 `5` 放在格式字符串中那对花括号的位置上。

在函数签名中，你_必须_声明每个参数的类型。这是 Rust 设计中经过深思熟虑的决定：在函数定义中要求类型注解意味着编译器几乎不需要你在代码的其他地方使用它们来推断你的意思。

当你希望一个函数有多个参数时，用逗号分隔各个参数的声明，如下所示：

```rust
   fn main() {
       another_function(5, 6);
   }
   
   fn another_function(x: i32, y: i32) {
       println!("x 的值是: {}", x);
       println!("y 的值是: {}", y);
   }
```

这个例子创建了一个带有两个参数的函数，这两个参数都是 `i32` 类型。然后，函数打印出两个参数的值。注意，函数的参数不需要都是相同的类型，只是在这个例子中恰巧是相同的。

让我们试着运行这段代码。将当前在你的函数项目中的 src/main.rs 文件的程序替换为上述例子并运行它：

```text
   $ cargo run
      Compiling functions v0.1.0 (file:///projects/functions)
       Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
        Running `target/debug/functions`
   x 的值是: 5
   y 的值是: 6
```

由于我们用 `5` 作为 `x` 的值调用了函数，并将 `6` 作为 `y` 的值传入，因此这两个字符串被打印出来时带有这些值。

_你可以参考 Rust 编程语言书籍中的以下章节：[函数如何工作](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)_