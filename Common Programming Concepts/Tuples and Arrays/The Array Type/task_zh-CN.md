## 数组类型

另一种存储多个值的集合方式是使用_数组_。与元组不同的是，数组的每个元素必须是相同的类型。Rust中的数组与其他一些语言中的数组不同，因为Rust中的数组具有固定长度，就像元组一样。

在Rust中，数组中的值写作方括号内用逗号分隔的列表：

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

当你希望数据分配在堆栈上而不是堆上（我们将在第4章讨论堆栈和堆）或者你想确保总是有固定数量的元素时，数组是很有用的。然而，数组没有vector类型那么灵活。vector是一种标准库提供的类似集合类型，允许其大小增减。如果你不确定使用数组还是vector，那么你应该使用vector。第8章将更详细地讨论vector。

你可能想用数组而不是vector的一个例子是一个需要知道一年中月份的程序。这样的程序不太可能需要增加或删除月份，因此你可以使用数组，因为你知道它将始终包含12个元素：

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

你可以通过使用方括号来写数组的类型，方括号内包括每个元素的类型、一个分号和数组中元素的数量，如下所示：

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

这里，`i32`是每个元素的类型。在分号之后，数字`5`表示数组包含五个元素。

以这种方式书写数组的类型与初始化数组的另一种语法类似：如果你想创建一个每个元素都包含相同值的数组，可以指定初始值，后跟一个分号，然后在方括号中写数组的长度，如下所示：

```rust
let a = [3; 5];
```

数组 `a` 将包含 `5` 个元素，所有这些元素最初都会被设置为值 `3`。这与写作 `let a = [3, 3, 3, 3, 3];` 是相同的，只是更为简洁。

#### 访问数组元素

数组是在堆栈上分配的单个内存块。你可以使用索引来访问数组元素，如下所示：

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

在这个例子中，名为 `first` 的变量将获取值 `1`，因为这是数组中索引 `[0]` 处的值。名为 `second` 的变量将从数组索引 `[1]` 获取值 `2`。

#### 访问无效的数组元素

如果你尝试访问数组中超出范围的元素会发生什么？假设你将示例更改为以下代码，当运行时会编译但会产生错误退出：

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
```

使用 cargo run 运行此代码会产生以下结果：

```text
   Finished dev [unoptimized + debuginfo] target(s) in 0.05s
   Running `target/debug/Test_Rust_Project`
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:5:19
```

编译没有产生错误，但程序产生了_运行时_错误并未成功退出。当你试图使用索引访问一个元素时，Rust会检查你指定的索引是否小于数组的长度。如果索引大于或等于数组的长度，Rust将会panic。

这是Rust安全原则在起作用的第一个例子。在许多低级语言中，这种检查不会进行，当你提供一个不正确的索引时，可能会访问无效内存。Rust通过立即退出来保护你免受此类错误的影响，而不是允许内存访问和继续执行。第9章将讨论更多关于Rust的错误处理。

_你可以参考以下章节的Rust编程语言书：[复合类型](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#compound-types)_