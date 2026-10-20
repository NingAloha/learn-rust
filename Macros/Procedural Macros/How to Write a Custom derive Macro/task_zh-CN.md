### 如何编写自定义 `derive` 宏

让我们创建一个名为 `hello_macro` 的 crate，该 crate 定义了一个名为 `HelloMacro` 的trait，其中包含一个名为 `hello_macro` 的关联函数。与其让我们的 crate 用户为他们的每种类型实现 `HelloMacro` trait，不如我们提供一个过程宏，使用户可以通过为他们的类型添加 `#[derive(HelloMacro)]` 注解来获取 `hello_macro` 函数的默认实现。默认实现将打印 `Hello, Macro! My name is TypeName!`，其中 `TypeName` 是定义了该 trait 的类型名称。换句话说，我们将编写一个 crate，使得其他程序员可以使用我们的 crate 编写如下代码。

```rust
    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Pancakes;

    fn main() {
        Pancakes::hello_macro();
    }
```

##### 我们的 crate 用户在使用我们过程宏时可以编写的代码

完成后，这段代码将打印 `Hello, Macro! My name is Pancakes!`。