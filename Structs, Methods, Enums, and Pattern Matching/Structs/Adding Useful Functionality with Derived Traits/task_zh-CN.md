### 用派生特征添加有用的功能

在调试程序时，能够打印 `Rectangle` 的实例并看到其所有字段的值会很不错。下面的代码尝试使用我们在前几章中使用过的 `println!` 宏。但是，这样做是行不通的。

<span class="filename">文件名: src/main.rs</span>

```rust,ignore,does_not_compile
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```

#### 尝试打印一个 `Rectangle` 实例

编译这段代码时，我们会收到一个带有核心信息的错误：

```text
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

`println!` 宏可以进行多种格式化，默认情况下，大括号告诉 `println!` 使用称为 `Display` 的格式：用于直接供终端用户消费的输出。我们迄今见过的基本类型默认实现 `Display`，因为对于展示给用户的 `1` 或其他任何基本类型只有一种方式。但对结构体来说，由于有更多的显示可能性，比如你是否希望有逗号？是否打印大括号？所有字段是否都显示？因此，Rust 没有尝试猜测我们的意图，结构体没有提供 `Display` 的实现。

如果继续阅读错误信息，会发现这个有用的提示：

```text
   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

试试看吧！`println!` 宏调用现在看起来像 `println!("rect1 is {:?}", rect1);`。在大括号内放置 `:?` 格式化符告诉 `println!` 我们希望使用一种称为 `Debug` 的输出格式。`Debug` 特征使我们能够以一种对开发者有用的方式打印我们的结构体，因此在我们调试代码时可以看到其值。

在进行这个改动后编译代码。糟糕！我们仍然遇到错误：

```text
error[E0277]: `Rectangle` doesn't implement `Debug`
```

但编译器再次给我们一个有用的提示：

```text
    = help: the trait `Debug` is not implemented for `Rectangle`
    = note: add `#[derive(Debug)]` or manually implement `Debug`
```

Rust 确实包含了打印调试信息的功能，但我们必须显式选择参与使该功能对我们的结构体可用。为此，我们在结构体定义前加上注解 `#[derive(Debug)]`，如下所示。

<span class="filename">文件名: src/main.rs</span>

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

#### 增加注解以派生 `Debug` 特征并使用调试格式打印 `Rectangle` 实例

现在当我们运行程序时，不会有任何错误，并且会看到以下输出：

```console
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/structs`
rect1 is Rectangle { width: 30, height: 50 }
```

虽然这不是最漂亮的输出，但它显示了该实例的所有字段的值，这在调试时绝对有帮助。对于较大的结构体，输出如果更易读会更有用；在这种情况下，我们可以在 `println!` 字符串中使用 `{:#?}` 而不是 `{:?}`。当在示例中使用 `{:#?}` 样式时，输出会像这样：

```console
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/structs`
rect1 is Rectangle {
    width: 30,
    height: 50,
}
```

Rust 为我们提供了许多特征，可以配合 `derive` 注解一起使用，为我们的自定义类型添加有用的行为。那些特征及其行为列在Rust书籍的[附录C][app3] 中。在“泛型类型、特征和生命周期”一章中，我们将介绍如何实现这些特征的自定义行为以及如何创建自己的特征。

我们的 `area` 函数非常具体：它仅计算矩形的面积。将这种行为更紧密地关联到我们的 `Rectangle` 结构体中将很有帮助，因为它不适用于任何其他类型。让我们看看如何通过将 `area` 函数转换为 `Rectangle` 类型上的 `area` *方法* 来继续重构这段代码。

[app3]: https://github.com/rust-lang/book/blob/master/src/appendix-03-derivable-traits.md