### 第一步：创建一个库包

第一步是创建一个新的库包。这里我们已经为您准备好，但如果您想自己探索，可以这样做：

```text
$ cargo new hello_macro --lib
```

接下来，我们将在 `lib.rs` 中定义 `HelloMacro` 特性及其关联函数：

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

我们有一个特性及其函数。此时，我们的包用户可以在 `main.rs` 中实现特性以实现所需功能，如下所示：

```rust
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

然而，他们需要为每种希望与 `hello_macro` 一起使用的类型编写实现块；我们希望减轻他们所需做的工作。

此外，我们还不能提供默认实现的 `hello_macro` 函数，该函数将打印实现特性类型的名称：Rust 没有反射功能，因此无法在运行时查找类型名称。我们需要一个宏在编译时生成代码。