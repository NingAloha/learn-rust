### 使用外部包

假设我们想要获取随机数。为此，我们需要一个名为`rand`的外部包。要在我们的项目中使用`rand`，我们在_Cargo.toml_中添加以下行：

```toml
    [dependencies]
    rand = "0.8.5"
```

在_Cargo.toml_中将`rand`添加为依赖项告诉Cargo从_https://crates.io_下载`rand`包及其任何依赖项，并使`rand`对我们的项目可用。

然后，为了将`rand`的定义引入我们的包的作用域，我们添加一个以crate名`rand`开始的`use`行，并列出我们想引入作用域的项目。例如，我们将`Rng`特性引入作用域，并调用`rand::thread_rng`函数：

```rust
    use rand::Rng;
    fn main() {
        let secret_number = rand::thread_rng().gen_range(1..=100);
    }
```

Rust社区的成员们在_https://crates.io_上提供了许多包，并将其中任何一个引入你的包需要遵循相同的步骤：在你的包的_Cargo.toml_文件中列出它们，并使用`use`从它们的crate中引入项目。

注意，标准库（`std`）也是我们包外部的一个crate。因为标准库是随Rust语言一起发布的，我们不需要更改_Cargo.toml_来包含`std`。但我们确实需要用`use`引用它，以便将其中的项目引入我们包的作用域。例如，对于`HashMap`我们会使用以下行：

```rust
    use std::collections::HashMap;
```

这是一个以`std`开头的绝对路径，它是标准库crate的名称。