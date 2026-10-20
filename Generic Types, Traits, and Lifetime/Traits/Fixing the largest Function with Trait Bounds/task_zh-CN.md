### 用特征约束修复 `largest` 函数

现在您已经了解了如何使用泛型类型参数的约束来指定所需的行为，让我们回到使用泛型类型参数的[`largest`函数定义](course://Generic+Types,+Traits,+and+Lifetime/Generic+Data+Types/In+Function+Definitions)! 上次我们尝试运行该代码时，收到了这样的错误：

```text
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ^^^^^^^^^^^^^^^^^^^^^^
```

在 `largest` 的主体中，我们希望使用大于（`>`）运算符比较两个 `T` 类型的值。因为该运算符是标准库特征 `std::cmp::PartialOrd` 的默认方法，我们需要在 `T` 的特征约束中指定 `PartialOrd`，以便 `largest` 函数可以处理任意类型的可比较的切片。我们不需要引入 `PartialOrd`，因为它在预导入中。将 `largest` 的签名修改为以下形式：

```rust,ignore
fn largest<T: PartialOrd>(list: &[T]) -> T {
```

这次编译代码时，我们得到了一组不同的错误：

```console
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src/main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       无法在此复制数据
  |                       因为 `list[_]` 类型 `T` 未实现 `Copy` 特征
  |                       help: consider borrowing here: `&list[0]`

error[E0507]: cannot move out of a shared reference
 --> src/main.rs:4:18
  |
4 |     for &item in list {
  |         -----    ^^^^
  |         ||
  |         |数据在此处移动
  |         |移动发生因为 `item` 的类型 `T` 未实现 `Copy` 特征
  |         help: consider removing the `&`: `item`

一些错误有详细说明：E0507，E0508。有关错误的更多信息，请尝试 `rustc --explain E0507`。
error: could not compile `chapter10` due to 2 previous errors
```

此错误的关键行是 `cannot move out of type [T], a non-copy slice`。对于非泛型版本的 `largest` 函数，我们只尝试找出最大的 `i32` 或 `char`。如在 [Clone and Copy](course://Understanding Ownership/What is ownership/Clone and Copy) 任务中“栈内存数据：复制”小节所讨论的，`i32` 和 `char` 这样的类型有已知大小，可以存储在栈上，因此它们实现了 `Copy` 特征。然而，当我们将 `largest` 函数做泛型化时，`list` 参数可能包含未实现 `Copy` 特征的类型。因此，我们无法将值从 `list[0]` 移动到 `largest` 变量中，从而导致该错误。

为了使代码仅调用那些实现了 `Copy` 特征的类型，我们可以在 `T` 的特征约束中添加 `Copy`！下面的代码片段显示了一个完整的泛型 `largest` 函数，只要传入的切片中的值类型实现了 `PartialOrd`和 `Copy` 特征（例如 `i32` 和 `char`），该函数就可以编译。


```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("最大的数字是 {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("最大的字符是 {}", result);
}
```

#### 一个适用于任何实现 `PartialOrd` 和 `Copy` 特征的泛型类型的 `largest` 函数的有效定义

如果我们不希望将 `largest` 函数限制为实现 `Copy` 特征的类型，我们可以指定 `T` 具有 `Clone` 特征约束，而不是 `Copy`。然后我们可以在想要 `largest` 函数拥有所有权时对切片中的每个值进行克隆。使用 `clone` 函数意味着我们在拥有堆数据的类型（如 `String`）的情况下可能会进行更多的堆分配，对于大量数据，堆分配可能很慢。

另一种实现 `largest` 的方法是让函数返回对切片中 `T` 值的引用。如果我们将返回类型改为 `&T` 而不是 `T`，从而改变函数的主体以返回一个引用，就不需要 `Clone` 或 `Copy` 特征约束，也可以避免堆分配。尝试自己实现这些替代解决方案！