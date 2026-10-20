## 示例：处理矩形

为了理解何时需要使用结构体，让我们编写一个程序来计算矩形的面积。我们将从单个变量开始，然后重构程序，直到使用结构体为止。

让我们使用 Cargo 创建一个名为 *rectangles* 的新二进制项目，该项目将接收用像素指定的矩形的宽度和高度，并计算矩形的面积。下面的代码展示了一个简短的程序，在项目的 *src/main.rs* 中可以实现这一点。

<span class="filename">文件名：src/main.rs</span>

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "矩形的面积是 {} 平方像素。",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

#### 通过单独的宽度和高度变量计算矩形的面积

现在，使用 `cargo run` 运行这个程序：

```console
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/structs`
矩形的面积是 1500 平方像素。
```

尽管上面的程序运行良好，并且通过调用 `area` 函数计算出了矩形的面积，但我们可以做得更好。宽度和高度是相关的，因为它们一起描述了一个矩形。

此代码的问题在 `area` 函数签名中很明显：

```rust
fn area(width: u32, height: u32) -> u32 {
```

`area` 函数应该计算一个矩形的面积，但我们编写的函数有两个参数。这些参数是相关的，但这在程序中的任何地方都没有表述。将宽度和高度组合在一起会让人更容易理解和管理。在课程“常见编程概念/数据类型”的“元组”部分，我们已经讨论了一种实现方法：使用元组。