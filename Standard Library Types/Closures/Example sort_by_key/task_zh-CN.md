### 示例: `sort_by_key`

现在让我们看看定义在切片上的标准库方法 `sort_by_key`，看看它有什么不同。它接受一个实现 `FnMut` 的闭包。该闭包接收一个参数，即对当前考虑的切片项的引用，并返回一个可以排序的类型 `K` 值。当你想根据每个项的特定属性对切片进行排序时，此函数非常有用。在以下列表中，我们有一个 `Rectangle` 实例列表，我们使用 `sort_by_key` 按其 `width` 属性从低到高进行排序：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
```

##### 使用 `sort_by_key` 和一个闭包按 `Rectangle` 实例的 `width` 值对列表进行排序的示例

此代码输出：

```console
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/rectangles`
[
    Rectangle {
        width: 3,
        height: 5,
    },
    Rectangle {
        width: 7,
        height: 12,
    },
    Rectangle {
        width: 10,
        height: 1,
    },
]
```

`sort_by_key` 被定义为接收一个 `FnMut` 闭包的原因是，它会多次调用该闭包：对切片中的每个项调用一次。闭包 `|r| r.width` 不捕获、修改或移动任何环境中的内容，因此它符合特征绑定的要求。

相比之下，下一个列表展示了一个闭包的例子，该闭包因为从环境中移动一个值而仅实现 `FnOnce`。编译器不允许我们将这个闭包与 `sort_by_key` 一起使用：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);
}
```

##### 试图使用 `FnOnce` 闭包与 `sort_by_key` 的示例

这是一个人为且复杂的方法（不起作用）来尝试统计 `sort_by_key` 在对 `list` 排序时被调用的次数。此代码试图通过将闭包环境中的一个 `String` `value` 推入 `sort_operations` 向量来进行计数。闭包捕获 `value` 并将其从闭包中移出，转移 `value` 的所有权给 `sort_operations` 向量。这个闭包可以被调用一次；再次调用将不起作用，因为 `value` 将不再在环境中以推入 `sort_operations`！因此，这个闭包只实现了 `FnOnce`。当我们尝试编译此代码时，我们会收到一个错误，表示 `value` 无法从闭包中移出，因为闭包必须实现 `FnMut`：

```console
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
  --> src/main.rs:27:30
   |
24 |       let value = String::from("by key called");
   |           ----- captured outer variable
25 | 
26 |       list.sort_by_key(|r| {
   |  ______________________-
27 | |         sort_operations.push(value);
   | |                              ^^^^^ move occurs because `value` has type `String`, which does not implement the `Copy` trait
28 | |         r.width
29 | |     });
   | |_____- captured by this `FnMut` closure

For more information about this error, try `rustc --explain E0507`.
error: could not compile `rectangles` due to previous error
```

错误指向在闭包体中将 `value` 从环境中移出的那一行。要解决这个问题，我们需要改变闭包体以不从环境中移出值。如果我们对 `sort_by_key` 被调用的次数感兴趣，可以在环境中保持一个计数器并在闭包体内递增其值来简单计算。以下列表中的闭包可与 `sort_by_key` 一起使用，因为它仅捕获对 `num_sort_operations` 计数器的可变引用，因此可以多次调用：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
```

##### 使用 `FnMut` 闭包与 `sort_by_key` 的示例是允许的

在定义或使用利用闭包的函数或类型时，`Fn` 特征很重要。下一节讨论迭代器，其中许多迭代器方法接受闭包参数。随着我们探讨迭代器，请记住闭包的这些细节！