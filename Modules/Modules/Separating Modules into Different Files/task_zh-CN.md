## 将模块分离到不同的文件中

到目前为止，本章中的所有示例都在一个文件中定义了多个模块。当模块变得庞大时，您可能希望将其定义移到单独的文件中，以便使代码更易于浏览。

例如，让我们从先前任务中的代码开始，将 `front_of_house` 模块移动到其自己的文件 _src/front_of_house.rs_，通过更改 crate 根文件使其包含下面显示的代码。在这个例子中，crate 根文件是 _src/lib.rs_，但这种方法同样适用于 crate 根文件为 _src/main.rs_ 的二进制 crates。

```rust
    mod front_of_house;

    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### 声明在 _src/front_of_house.rs_ 中定义的 front_of_house 模块

然后，_src/front_of_house.rs_ 从 `front_of_house` 模块的内容获得定义，如下所示。

<span class="filename">文件名: src/front_of_house.rs</span>

```rust
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
```

##### 在 _src/front_of_house.rs_ 中 front_of_house 模块内的定义

在 `mod front_of_house` 后使用分号而不是使用一个块，告知 Rust 从另一个与模块同名的文件中加载模块内容。为了在我们的例子中继续并将 `hosting` 模块提取到其自己的文件中，我们将 _src/front_of_house.rs_ 更改为仅包含 `hosting` 模块的声明：

```rust
    pub mod hosting;
```

然后我们创建 _src/front_of_house_ 目录和文件 _src/front_of_house/hosting.rs_ 来包含在 `hosting` 模块中进行的定义：

```rust
    pub fn add_to_waitlist() {}
```

模块树保持不变，并且即使定义分布在不同的文件中，`eat_at_restaurant` 中的函数调用也会正常工作。这种技术允许您随着模块的扩展将它们移动到新的文件中。

注意，在 _src/lib.rs_ 中的 `pub use crate::front_of_house::hosting` 语句也没有改变，`use` 对于哪些文件作为 crate 的一部分进行编译没有任何影响。`mod` 关键字用来声明模块，Rust 会在一个与模块同名的文件中查找用于该模块的代码。